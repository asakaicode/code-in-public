async fn print_numbers(i: i32) {
    println!("{}", i);
}

#[tokio::main]
async fn main() {
    for i in 0..10 {
        print_numbers(i).await;
    }
}
