use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut words = input.split_whitespace();

    // PART 1: Binary -> Decimal
    let n: usize = words.next().unwrap().parse().unwrap();
    let m: usize = words.next().unwrap().parse().unwrap();

    let mut val1: i64 = 0;
    for _ in 0..n {
        let bit: i64 = words.next().unwrap().parse().unwrap();
        val1 = (val1 << 1) | bit;
    }

    let mut val2: i64 = 0;
    for _ in 0..m {
        let bit: i64 = words.next().unwrap().parse().unwrap();
        val2 = (val2 << 1) | bit;
    }

    // Bitwise Operations: OR, AND, XOR
    println!("{}", val1 | val2);
    println!("{}", val1 & val2);
    println!("{}", val1 ^ val2);

    // Blank line between parts
    println!();

    // PART 2: Decimal -> Binary
    let a: i64 = words.next().unwrap().parse().unwrap();
    let b: i64 = words.next().unwrap().parse().unwrap();

    let resultados = [a | b, a & b, a ^ b];

    for &res in &resultados {
        if res == 0 {
            println!("0");
        } else {
            let mut started = false;
            for i in (0..63).rev() {
                let bit = (res >> i) & 1;
                if bit == 1 {
                    started = true;
                }
                if started {
                    print!("{}", bit);
                }
            }
            println!();
        }
    }
}
