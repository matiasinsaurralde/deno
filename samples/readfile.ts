import { readFileSync } from "deno";
var startTs = Date.now()
for(let i = 0; i < 5000; i++) {
    readFileSync("samples/file.txt");
}
var endTs = Date.now();
console.log("elapsed = ", endTs - startTs);

