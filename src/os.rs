use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use capnp::message::{Builder, HeapAllocator};
use deno;
use msg_capnp::cap_msg;
use msg_capnp::cap_msg::Command;

pub fn code_fetch(msg: &cap_msg::Reader) -> Builder<HeapAllocator> {
    let mut use_alt_path = false;
    let module_specifier = msg.get_code_fetch_module_specifier().unwrap();
    let base_path = Path::new("dist/").join(module_specifier);
    let filepath = Path::new(module_specifier);
    let mut f: File;
    let mut contents = String::new();
    if filepath.exists() {
        f = File::open(filepath).unwrap();
        f.read_to_string(&mut contents).expect("i/o error");
    } else {
        use_alt_path = true;
        let result = File::open(base_path.as_path());
        if result.is_ok() {
            f = result.unwrap();
            f.read_to_string(&mut contents).expect("i/o error");
        }
    }

    let filepath_str: &str;
    if use_alt_path {
        filepath_str = base_path.to_str().unwrap();
    } else {
        filepath_str = filepath.to_str().unwrap();
    }

    let mut reply_msg = Builder::new_default();
    {
        let mut _m = reply_msg.init_root::<cap_msg::Builder>();
        _m.set_command(Command::CodeFetchRes);
        _m.set_code_fetch_res_module_name(filepath_str);
        _m.set_code_fetch_res_filename(filepath_str);
        _m.set_code_fetch_res_source_code(&contents);
    }
    reply_msg
}

pub fn read_file_sync(msg: &cap_msg::Reader) -> Builder<HeapAllocator> {
    let mut buf = Vec::new();
    let filename = msg.get_read_file_sync_filename().unwrap();
    let f = File::open(filename);
    if f.is_ok() {
        f.unwrap().read_to_end(&mut buf);
    }
    let mut reply_msg = Builder::new_default();
    {
        let mut _m = reply_msg.init_root::<cap_msg::Builder>();
        _m.set_command(Command::ReadFileSyncRes);
        _m.set_read_file_sync_data(&buf);
    }
    reply_msg
}
