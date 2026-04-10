use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read.");

    let mut a: u32 = input.trim().parse().expect("Invalid Number.");

    let mut n = 1;
    
    while a != 1 {
        print!("{} ", a);
        if a % 2 == 0 {
            a = a / 2;
        } else {
            a = a * 3 + 1;
        }
        n += 1;
    }
    
    println!("{}", a);
    println!("{}", n);



}