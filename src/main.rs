fn fibonacci(n: u32) {
    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        print!("{} ", a);
        let next = a + b;
        a = b;
        b = next;
    }
    println!();
}

fn main() {
    let n = 10;
    println!("Fibonacci series up to {} terms:", n);
    fibonacci(n);
}


// Not using chatGpt