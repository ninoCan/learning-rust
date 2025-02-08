fn fibonacci(n: u128) -> u128 {
    let mut numbers: Vec<u128> = Vec::with_capacity(n as usize + 1);

    numbers.push(0);
    numbers.push(1);

    for i in 2..=n {
        let next_fibonacci_num = numbers[i as usize - 1] + numbers[i as usize - 2];
        numbers.push(next_fibonacci_num);
    }

    numbers[n as usize]
}

fn main() {
    let nth: u128 = 10;

    let result = fibonacci(nth);

    println!("The {}th Fibonacci number is: {}", nth, result);
}
