fn fibonacci(ind: u8) -> u32 {
    match ind {
        0 => return 0,
        1 => return 1,
        _ => (),
    }

    let mut curr: u32 = 1;
    let mut prev: u32 = 0;

    for _ in 0..ind - 1 {
        let buff: u32 = curr;
        curr += prev;
        prev = buff;
    }

    curr
}

fn main() {
    let fib = fibonacci(20);
    println!("{}", fib);
}
