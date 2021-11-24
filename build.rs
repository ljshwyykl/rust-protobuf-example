extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["src/protos/example.proto"])
        .include("src/protos")
        .run()
        .expect("Running protoc failed.");
}
