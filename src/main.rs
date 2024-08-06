fn main() {
    let result = fibonacci(5);
    println!("{}", result);
}

fn fibonacci(n: u8) -> u8 {
    let mut pre: u8 = 0;
    let mut curr: u8 = 1;
    let mut count: u8 = 3;

    while count <= n {
        let temp = curr;
        curr += pre;

        pre = temp;

        count += 1;
    }
    return curr;
}
