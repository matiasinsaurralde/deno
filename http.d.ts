import { main as pb } from "./msg.pb";

declare module "http" {
    function abc(): number;

    export interface RequestOptions {
        method?: string;
        url?: string;
        referrer?: string;
        mode?: string;
        credentials?: string;
        redirect?: string;
        integrity?: string;
        cache?: string;
    }

    export class Request{
        opts: RequestOptions
        method: string;
        path: string;
        body: any | string;
        constructor(url: string, opts?: RequestOptions)
    }

    export class Response {
        server_id: number;
        req_id: number;
        constructor()
        write(data: Uint8Array | string): void
        status(code: number): void
        end(): void
    }

    export interface ServerOptions {
    }

    export class Server{
        private readonly id: number;
        private port: number;
        private cb: (req: Request, res: Response) => void;
        constructor(cb: (req: Request, res: Response) => void);
        listen(port: number): void
        buildRequest(msg: pb.Msg): Request
        buildResponse(msg: pb.Msg): Response
        onMsg(msg: pb.Msg): void
    }

    function createServer(cb: (req: Request, res: Response) => void): Server;
}