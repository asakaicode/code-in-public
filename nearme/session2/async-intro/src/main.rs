use std::vec::Vec;
use futures::{executor::block_on, future};

async fn print_number(i: i32) {
    println!("{}", i);
}

async fn async_main() {
    let fn_objs: Vec<_> = (1..(10+1)).map(|i| print_number(i)).collect();
    future::join_all(fn_objs).await;
}

fn main() {
    block_on(async_main());
}