use std::io;

fn main() {
    let n: u32 = get_positive_number_from_user();
    let nth = find_nth_fibonacci_number(n);
    println!("The {}th Fibonacci numnber is {}!", n, nth);
}


fn find_nth_fibonacci_number(n: u32) -> u128 {
    // 0,1,2,3,5,8,13,...
    // O(n) time and constant space complexity with dynamic memoization.
    if n <= 3 {
        return n.into();
    }
    let mut two_prev = 1;
    let mut one_prev = 2;
    let mut curr: u128 = 3;
    for _ in 4..n + 1 {
        curr = two_prev + one_prev;
        two_prev = one_prev;
        one_prev = curr;
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
