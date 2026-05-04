mod bcp;
mod codec;

#[allow(dead_code)]
mod generated_messages {
    include!(concat!(env!("OUT_DIR"), "/bcp_messages.rs"));
}

use generated_messages::MsgB;

fn main() {
    println!("BCP message bindings were generated at build time.");

    let a = MsgB {
        count_1: 1,
        count_2: -1,
    };
    let bits = a.encode();
    println!("{:?}", bits);
}
