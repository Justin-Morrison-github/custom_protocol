mod bcp;
mod codec;

#[allow(dead_code)]
mod generated;

use generated::CounterMsg;

fn main() {
    let a = CounterMsg {
        ticks: 1,
        status: -1,
    };
    let bits = a.encode();
    println!("{:?}", bits);
}
