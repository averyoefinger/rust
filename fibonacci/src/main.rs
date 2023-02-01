use std::io;

fn main() {
    let mut input = String::new();
    let n: usize;
    loop {
        println!("Input n to calculate nth fibonacci number:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        n = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    let result: u32;
    // store previous two numbers
    let mut f: [u32; 2] = [0, 1];
    if n < 2 {
        result = f[n];
    } else {
        for _ in 1..n {
            let tmp = f[0];
            f[0] = f[1];
            f[1] = f[0] + tmp;
        }
        result = f[1];
    }
    // print result
    println!("fibonacci({n}) = {result}");
}
