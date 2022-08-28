use futures::future;

async fn print_number(i: i32) {
    println!("{}", i);
}

#[tokio::main]
async fn main() {
    let fn_objs = (1..(10+1)).map(|i|
        print_number(i)
    );
    future::join_all(fn_objs).await;
}