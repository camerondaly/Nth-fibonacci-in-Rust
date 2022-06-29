use std::io;

fn main() {
    let n: u32 = get_positive_number_from_user();
    let nth = find_nth_fibonacci_number(n);
    println!("Fibonacci number {} is {}.", n, nth);
    // test();
}


fn find_nth_fibonacci_number(n: u32) -> u128 {
    // 0,1,1,2,3,5,8,13,...
    // O(n) time and constant space complexity with dynamic memoization.
    if n <= 1 {
        return n.into();
    }
    let mut two_prev: u128 = 0;
    let mut one_prev: u128 = 1;
    let mut curr: u128 = 1;
    for _ in 2..n + 1 {
        curr = two_prev + one_prev;
        two_prev = one_prev;
        one_prev = curr;
        println!("now we are on {}", curr);
    }
    return curr;
}


fn get_positive_number_from_user() -> u32 {
    loop {
        let mut n = String::new();
        println!("Please enter the (positive) number in the Fibonacci series you would like to compute:");
        io::stdin().read_line(&mut n).expect("Failed to read number.");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return n;
    }
}

fn test() {
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21];
    for i in 0..9 {
        let curr_res = find_nth_fibonacci_number(i as u32);
        if curr_res != expected[i as usize] {
            println!("Test failed! Actual {} does not equal expected {}", curr_res, expected[i as usize]);
            return;
        }
    } 
    println!("All tests passed!");
}