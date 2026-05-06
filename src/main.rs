mod bcp;
mod codec;

#[allow(dead_code)]
mod generated;

use generated::MsgB;

fn main() {
    let a = MsgB {
        ticks: 1,
        status: -1,
    };
    let bits = a.encode();
    println!("{:?}", bits);
}
