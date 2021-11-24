mod protos;

use protobuf::Message;
use protos::example::HelloRequest;
fn main() {
    println!("Hello, world!");

    let mut out_msg = HelloRequest::new();
    out_msg.set_name("John Smith".to_string());

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    println!("{:?}", out_bytes);

    // Decode example request
    let in_msg: HelloRequest = Message::parse_from_bytes(&out_bytes).unwrap();

    println!("{:?}", in_msg);
}
