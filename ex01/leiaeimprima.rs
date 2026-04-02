use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let palavras: Vec<&str> = input.split_whitespace().collect();

    if palavras.len() == 2 {
        let a: i64 = palavras[0].parse().unwrap();

        let b: char = palavras[1].chars().next().unwrap();

        
        println!("{}", a);
        println!("{}", b);
    }
}
