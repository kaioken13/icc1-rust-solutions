use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Lê n
    let n: i64 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return,
    };

    if n < 2 {
        return;
    }

    let soma_esperada = (n * (n + 1)) / 2;
    let mut soma_atual: i64 = 0;

    for line in lines {
        if let Ok(l) = line {
            for num_str in l.split_whitespace() {
                if let Ok(num) = num_str.parse::<i64>() {
                    soma_atual += num;
                }
            }
        }
    }

    println!("{}", soma_esperada - soma_atual);
}
