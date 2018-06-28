use std::fs;

use v8worker2;
use runtime;

#[test]
fn test_runtime_init() {
    let mut handler = v8worker2::new_handler();
    handler.init();

    let test_script = "console.log(1+1)";
    fs::write("test.js", test_script).expect("Couldn't write test script");
    let mut r = runtime::new("test.js".to_string());
    r.start();
}