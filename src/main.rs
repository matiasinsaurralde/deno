extern crate prost;
#[macro_use]
extern crate prost_derive;
pub mod deno {
    include!(concat!(env!("OUT_DIR"), "/deno.rs"));
}

extern crate capnp;
mod msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/msg_capnp.rs"));
}

extern crate bytes;
extern crate getopts;

use getopts::Options;
use std::env;
mod runtime;
mod os;
mod test;

extern crate v8worker2;

fn print_usage(opts: &Options) {
    let brief = format!("Usage: deno [options] SCRIPT");
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Parse CLI arguments:
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("d", "debug", "run in debug mode");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&opts);
        return;
    }
    let _input_file: String;
    if matches.free.is_empty() {
        print_usage(&opts);
        return;
    }
    _input_file = matches.free[0].clone();

    // Start V8 and Deno runtime:
    let mut handler = v8worker2::new_handler();
    handler.init();
    let mut r = runtime::new(_input_file);

    // Enable debug mode:
    if matches.opt_present("d") {
        r.use_debug_mode();
    }
    r.start();
}
