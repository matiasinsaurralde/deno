extern crate prost;
#[macro_use]
extern crate prost_derive;
pub mod deno {
    include!(concat!(env!("OUT_DIR"), "/deno.rs"));
}
extern crate bytes;
mod runtime;
mod os;

extern crate v8worker2;

fn main() {
    // Parse flags:
    // let args: Vec<String> = env::args().collect();
    // let _input = &args[1];
    let mut handler = v8worker2::new_handler();
    handler.init();
    let mut r = runtime::new();
    r.start();
}
