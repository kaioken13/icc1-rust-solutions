use std::io;

fn main() {
    let mut input = String::new();
    
    // Reading the first line (x)
    io::stdin().read_line(&mut input).expect("Fail to read x");
    let x: f64 = input.trim().parse().expect("x should be a floating-point number");
    
    input.clear();
    
    // Reading the second line (n)
    io::stdin().read_line(&mut input).expect("Fail to read n");
    let n: i32 = input.trim().parse().expect("n should be an integer");

    let mut soma: f64 = 0.0;
    let mut x_potencia: f64 = 1.0;
    let mut fatorial: f64 = 1.0;

    for i in 0..n {
        if i == 0 {
            soma = x;
        } else {
            x_potencia *= x * x;
            
            let base_anterior = (2 * (i - 1) + 1) as f64;
            fatorial *= (base_anterior + 1.0) * (base_anterior + 2.0);
            
            let termo = x_potencia / fatorial;
            
            // Alternating the sign
            if i % 2 == 1 {
                soma -= termo;
            } else {
                soma += termo;
            }
        }
    }

    println!("{:.10}", soma);
}
