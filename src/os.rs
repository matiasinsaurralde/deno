use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use deno;

pub fn code_fetch(msg: deno::Msg) -> deno::Msg {
    // Construct the reply, we'll return an empty message if an error occurs:
    let mut reply_msg = deno::Msg::default();

    let mut use_alt_path = false;
    let base_path = Path::new("dist/").join(&msg.code_fetch_module_specifier);
    let filepath = Path::new(&msg.code_fetch_module_specifier);

    // Try to load the file, use the base path prefix if it doesn't work:
    let f = match File::open(&filepath) {
        Ok(file) => {
            Some(file)
        },
        Err(_) => {
            match File::open(base_path.as_path()) {
                Ok(file) => {
                    use_alt_path = true;
                    Some(file)
                },
                Err(_) => {
                    return reply_msg
                }
            }
        }
    };
    let mut contents = String::new();
    f.unwrap().read_to_string(&mut contents).expect("i/o error");
    reply_msg.command = deno::msg::Command::CodeFetchRes as i32;

    if use_alt_path {
        let filepath_str = base_path.to_str().unwrap();
        reply_msg.code_fetch_res_module_name = String::from(filepath_str);
        reply_msg.code_fetch_res_filename = String::from(filepath_str);
        reply_msg.code_fetch_res_source_code = contents;
        return reply_msg;
    }
    let filepath_str = filepath.to_str().unwrap();
    reply_msg.code_fetch_res_module_name = String::from(filepath_str);
    reply_msg.code_fetch_res_filename = String::from(filepath_str);
    reply_msg.code_fetch_res_source_code = contents.clone();
    reply_msg
}

pub fn read_file_sync(msg: deno::Msg) -> deno::Msg {
    let mut reply_msg = deno::Msg::default();
    let f = match File::open(&msg.read_file_sync_filename) {
        Ok(file) => {
            Some(file)
        },
        Err(_) => {
            return reply_msg
        }
    };
    let mut buf = Vec::new();
    f.unwrap().read_to_end(&mut buf);
    reply_msg.command = deno::msg::Command::ReadFileSyncRes as i32;
    reply_msg.read_file_sync_data = buf;
    reply_msg
}