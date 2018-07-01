extern crate prost_build;
extern crate capnpc;

fn main() {
    prost_build::compile_protos(&["src/msg.proto"], &["src/"]).unwrap();
    capnpc::CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/msg.capnp")
        .run().expect("schema compiler command");
}
