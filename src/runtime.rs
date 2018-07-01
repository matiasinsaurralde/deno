extern crate v8worker2;
use v8worker2::worker::Worker as Worker;

extern crate bytes;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use prost::Message;
use capnp::serialize_packed;

use msg_capnp::cap_msg;
use msg_capnp::cap_msg::Command;
use deno;
use os;

const MAIN_JS: &str = "/main.js";
const DENO_MAIN_JS: &str = "/deno_main.js";

pub struct Runtime {
    worker: Worker,
    filename: String,
    debug_mode: bool
}

impl Runtime {
    pub fn start(&mut self) {
        let main_js_filename = String::from("dist/main.js");
        let mut main_js = File::open(main_js_filename).expect("File not found");
        let mut main_js_contents = String::new();
        main_js.read_to_string(&mut main_js_contents).expect("I/O error");
        self.worker.load(MAIN_JS, main_js_contents.clone());

        // Call denoMain
        self.worker.load(DENO_MAIN_JS, "denoMain();".to_string());

        // Load main.map:
        let main_map_filename = String::from("dist/main.map");
        let mut main_map = File::open(main_map_filename).expect("File not found");
        let mut main_map_contents = String::new();
        main_map.read_to_string(&mut main_map_contents).expect("I/O error");

        // Get current dir
        let cwd = env::current_dir().unwrap();
        let cwd_str = cwd.into_os_string().into_string().unwrap();

        // Prepare start message:
        let mut message = ::capnp::message::Builder::new_default();
        {
            let mut m = message.init_root::<cap_msg::Builder>();
            m.set_command(Command::StartCmd);
            m.set_start_cmd_cwd(&cwd_str);
            m.set_start_cmd_main_js(&main_js_contents);
            m.set_start_cmd_main_map(&main_map_contents);
            m.set_start_cmd_debug_flag(self.debug_mode);
            let mut argv = m.init_start_cmd_argv(1);
            argv.reborrow().set(0, &self.filename);
        }
        let mut message_bytes = Vec::new();
        serialize_packed::write_message(&mut message_bytes, &message);
        let b = bytes::Bytes::from(message_bytes);
        self.worker.send_bytes(b);
    }

    fn dummy_base_msg() -> Box<bytes::Bytes> {
        let mut _base_msg = deno::BaseMsg::default();
        let _base_msg_length = _base_msg.encoded_len();
        let mut _base_msg_buf = Vec::with_capacity(_base_msg_length);
        _base_msg.encode(&mut _base_msg_buf).unwrap();
        let data: bytes::Bytes = bytes::Bytes::from(_base_msg_buf.as_slice());
        Box::new(data)
    }

    pub fn use_debug_mode(&mut self) {
        self.debug_mode = true;
    }
}

pub fn new(_input_file: String) -> Runtime {
    let r: Runtime;
    let cb = |incoming_data: bytes::Bytes| -> Box<bytes::Bytes> {
        let mut _d = incoming_data.as_ref();
        let _reader = serialize_packed::read_message(&mut _d, ::capnp::message::ReaderOptions::new()).unwrap();
        let _msg = _reader.get_root::<cap_msg::Reader>().unwrap();
        let cmd = _msg.get_command().unwrap();
        if cmd == Command::CodeFetch {
            let reply = os::code_fetch(&_msg);
            let mut message_bytes = Vec::new();
            serialize_packed::write_message(&mut message_bytes, &reply);
            let b = bytes::Bytes::from(message_bytes);
            return Box::new(b);
        }
        if cmd == Command::ReadFileSync {
            let reply = os::read_file_sync(&_msg);
            let mut message_bytes = Vec::new();
            serialize_packed::write_message(&mut message_bytes, &reply);
            let b = bytes::Bytes::from(message_bytes);
            return Box::new(b);
        }
        Runtime::dummy_base_msg()
    };
    let mut _worker = Worker::new(cb);
    r = Runtime{
        worker: _worker,
        filename: _input_file,
        debug_mode: false,
    };
    r
}