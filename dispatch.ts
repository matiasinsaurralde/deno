// Copyright 2018 Ryan Dahl <ry@tinyclouds.org>
// All rights reserved. MIT License.
import { typedArrayToArrayBuffer } from "./util";
import { _global } from "./globals";
import { deno as pb } from "./msg.pb";

import * as capnp from "capnp-ts";
import {CapMsg, CapMsg_Channel} from "./msg.capnp";

export type MessageCallback = (msg: CapMsg) => void;

const send = V8Worker2.send;
const channels = new Map<CapMsg_Channel, MessageCallback[]>();

export function sub(channel: CapMsg_Channel, cb: MessageCallback): void {
  let subscribers = channels.get(channel);
  if (!subscribers) {
    subscribers = [];
    channels.set(channel, subscribers);
  }
  subscribers.push(cb);
}

export function pub(channel: CapMsg_Channel, payload: Uint8Array): null | ArrayBuffer {
  console.log("pub is called");
  const msg = pb.BaseMsg.fromObject({ channel, payload });
  const ui8 = pb.BaseMsg.encode(msg).finish();
  const ab = typedArrayToArrayBuffer(ui8);
  return send(ab);
}

// Internal version of "pub".
// TODO add internal version of "sub"
export function pubInternal(obj: capnp.Message): null | CapMsg {
  const ab = obj.toPackedArrayBuffer();
  const res = send(ab);
  const m = new capnp.Message(res);
  const res_msg = m.getRoot(CapMsg);
  return res_msg;
}

V8Worker2.recv((ab: ArrayBuffer) => {
  const m = new capnp.Message(ab);
  const msg = m.getRoot(CapMsg);
  const subscribers = channels.get(msg.getChannel());
  if (subscribers == null) {
    throw Error(`No subscribers for channel "${msg.getChannel()}".`);
  }

  for (const subscriber of subscribers) {
    subscriber(msg);
  }
});

// Delete the V8Worker2 from the global object, so that no one else can receive
// messages.
_global["V8Worker2"] = null;
