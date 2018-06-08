import {rawlog, log} from "./util";

// import * as util from "./util";
import { pubInternal, sub } from "./dispatch";
import { main as pb } from "./msg.pb";

const servers = new Map<number, Server>();
const dec = new TextDecoder("utf8");
const enc = new TextEncoder();

export function initHttp() {
  sub("http", (payload: Uint8Array) => {
      const msg = pb.Msg.decode(payload);
      const id = msg.httpServerId;
      const s = servers.get(id);
      s.onMsg(msg);
  });
}

export function abc() {
    rawlog("abc is called!");
    return 1+1;
}

export interface RequestOptions {
    method?: string;
    referrer?: string;
    mode?: string;
    credentials?: string;
    redirect?: string;
    integrity?: string;
    cache?: string;
}

export class Request {
    opts: RequestOptions;
    readonly default_method = "GET";
    path: string;
    method: string;
    body: any | string;
    constructor(url: string, opts?: RequestOptions) {
      if(opts == null) {
        opts = {
            method: this.default_method
        }
      }
      this.opts = opts;
      this.method = opts.method;
    }
}

export class Response {
    server_id: number;
    req_id: number;
    constructor(Server?: Server) {
    }
    write(data: Uint8Array | string) {
        let raw_data: Uint8Array
        if (typeof(data) == "string") {
            raw_data = enc.encode(data);
        } else {
            raw_data = data as Uint8Array;
        }
        pubInternal("http", {
            command: pb.Msg.Command.HTTP_RES_WRITE,
            // httpServerId: this.server_id,
            httpReqId: this.req_id,
            httpResBody: raw_data
        })
    }
    status(code: number) {
        pubInternal("http", {
            command: pb.Msg.Command.HTTP_RES_STATUS,
            httpReqId: this.req_id,
            httpResCode: code
        })
    }
    end() {
        pubInternal("http", {
            command: pb.Msg.Command.HTTP_RES_END,
            httpServerId: this.server_id,
            httpReqId: this.req_id
        })
    }
}

let nextServerId = 0;
export class Server {
    private readonly id: number;
    private port: number;
    private cb: (req: Request, res: Response) => void;
    constructor(cb: (req: Request, res: Response) => void) {
        this.id = nextServerId++;
        this.cb = cb;
        servers.set(this.id, this);
        pubInternal("http", {
            command: pb.Msg.Command.HTTP_CREATE,
            httpServerId: this.id
        })
    }
    listen(port: number) {
        log("Starting server on", port);
        pubInternal("http", {
            command: pb.Msg.Command.HTTP_LISTEN,
            httpServerId: this.port,
            httpListenPort: port
        })
    }
    buildRequest(msg: pb.Msg) {
        let req = new Request("", {});
        req.path = msg.httpReqPath;
        req.method = msg.httpReqMethod;
        let raw_body = dec.decode(msg.httpReqBody);
        // Implement a JSON body parser by default, fallback to string representation:
        try {
            let body = JSON.parse(raw_body)
            req.body = body;
        } catch(e) {
            req.body = raw_body.toString();
        };
        return req
    }
    buildResponse(msg: pb.Msg) {
        let res = new Response();
        res.server_id = msg.httpServerId;
        res.req_id = msg.httpReqId;
        return res;
    }
    onMsg(msg: pb.Msg) {
        if (msg.command = pb.Msg.Command.HTTP_REQ) {
            let req = this.buildRequest(msg);
            let res = this.buildResponse(msg);
            this.cb(req, res);
        }
    }
}

export function createServer(cb: (req: Request, res: Response)=>void): Server {
    let s = new Server(cb);
    return s;
}