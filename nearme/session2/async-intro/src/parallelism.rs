use futures::future;

async fn print_number(i: i32) {
    println!("{}", i);
}

#[tokio::main]
async fn main() {
    let fn_objs = (1..(10+1)).map(|i|
        tokio::spawn(async move { print_number(i).await })
    );
    future::join_all(fn_objs).await;
}