// Copyright 2018 Ryan Dahl <ry@tinyclouds.org>
// All rights reserved. MIT License.
import { ModuleInfo } from "./types";
import { pubInternal } from "./dispatch";

import * as capnp from "capnp-ts";
import {CapMsg, CapMsg_Channel, CapMsg_Command} from "./msg.capnp";

export function exit(exitCode = 0): void {
}

export function codeFetch(
  moduleSpecifier: string,
  containingFile: string
): ModuleInfo {
  const message = new capnp.Message();
  const msg: CapMsg = message.initRoot(CapMsg);
  msg.setCommand(CapMsg_Command.CODE_FETCH);
  msg.setCodeFetchModuleSpecifier(moduleSpecifier);
  msg.setCodeFetchContainingFile(containingFile);
  msg.setChannel(CapMsg_Channel.OS);
  const res = pubInternal(message);
  return {
    moduleName: res.getCodeFetchResModuleName(),
    filename: res.getCodeFetchResFilename(),
    sourceCode: res.getCodeFetchResSourceCode(),
    outputCode: res.getCodeFetchResOutputCode()
  };
}

export function codeCache(
  filename: string,
  sourceCode: string,
  outputCode: string
): void {
}

export function readFileSync(filename: string): Uint8Array {
  const message = new capnp.Message();
  const msg: CapMsg = message.initRoot(CapMsg);
  msg.setCommand(CapMsg_Command.READ_FILE_SYNC);
  msg.setReadFileSyncFilename(filename);

  msg.setChannel(CapMsg_Channel.OS);
  const res = pubInternal(message);
  return res.getReadFileSyncData().toUint8Array();
}

export function writeFileSync(
  filename: string,
  data: Uint8Array,
  perm: number
): void {
}
