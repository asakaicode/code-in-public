use std::vec::Vec;

async fn print_numbers(nums: &Vec<i32>) {
    for num in nums {
        println!("{}", num);
    }
}

async fn add_numbers(nums: &Vec<i32>) {
    let mut add_result = 0;
    let nums_len = nums.len();
    for num in nums {
        add_result += num;
    }
    println!("The result of summation from {} to {} is {}.", nums[0], nums[nums_len-1], add_result);
}

#[tokio::main]
async fn main() {
    // prepare a vector which contains integers from 1 to 10
    let mut nums = Vec::new();
    for i in 1..11 {
        nums.push(i);
    }
    let print_numbers = print_numbers(&nums);
    let add_numbers = add_numbers(&nums);
    tokio::join!(print_numbers, add_numbers);

    println!("Finished!!");
}