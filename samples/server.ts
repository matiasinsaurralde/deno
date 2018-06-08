import { Request, Response, createServer } from "http";

const s = createServer((req: Request, res: Response) => {
    console.log(req.method, req.path, req.body)
    if(req.path == "/hello") {
        res.write("Hello from Deno\n");
        res.end();
    } else if (req.path == "/echo") {
        res.write(req.body.echo + "\n");
        res.end();
    } else {
        res.status(400);
        res.end();
    };
})
s.listen(5000);

setTimeout(function(){}, 50000);
