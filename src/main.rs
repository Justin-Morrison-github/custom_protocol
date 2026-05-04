mod bcp;
mod codec;

#[allow(dead_code)]
mod generated;

use generated::MsgB;

fn main() {
    let a = MsgB {
        count_1: 1,
        count_2: -1,
    };
    let bits = a.encode();
    println!("{:?}", bits);
}
