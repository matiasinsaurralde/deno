// Copyright 2018 Ryan Dahl <ry@tinyclouds.org>
// All rights reserved. MIT License.
// This allows us to have async/await in our code. It must be loaded first.
import "babel-polyfill";

import * as dispatch from "./dispatch";

import * as runtime from "./runtime";
import * as util from "./util";

import {CapMsg, CapMsg_Channel} from "./msg.capnp";

// import { initTimers } from "./timers";
// import { initFetch } from "./fetch";

// To control internal logging output
// Set with the -debug command-line flag.
export let debug = false;
let startCalled = false;

// denoMain is needed to allow hooks into the system.
// Also eventual snapshot support needs it.
// tslint:disable-next-line:no-any
(window as any)["denoMain"] = () => {
  // tslint:disable-next-line:no-any
  delete (window as any)["denoMain"];

  // initTimers();
  // initFetch();

  dispatch.sub(CapMsg_Channel.START, (msg: CapMsg) => {
    if (startCalled) {
      throw Error("start message received more than once!");
    }
    startCalled = true;

    /*const msg = pb.Msg.decode(payload);
    const {
      startCwd: msg.getStartCmdCwd(),
      startArgv: argv,
      startDebugFlag: debugFlag,
      startMainJs: mainJs,
      startMainMap: mainMap
    } = msg;*/

    const cwd = msg.getStartCmdCwd();
    const mainJs = msg.getStartCmdMainJs();
    const mainMap = msg.getStartCmdMainMap();
    const argv = msg.getStartCmdArgv();

    debug = msg.getStartCmdDebugFlag();
    util.log("start", { cwd, argv, debug });

    runtime.setup(mainJs, mainMap);

    const inputFn = argv.get(0);
    const mod = runtime.resolveModule(inputFn, `${cwd}/`);
    mod.compileAndRun();
  });
};
