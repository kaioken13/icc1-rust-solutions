use std::io;

fn main() {
    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_ok() {
        let palavras: Vec<&str> = input.split_whitespace().collect();

        if let [raw_a, raw_b, raw_c] = palavras.as_slice() {
            
            let a = raw_a.parse::<u16>().ok();
            let b = raw_b.parse::<u16>().ok();
            let c = raw_c.parse::<u16>().ok();

            if let (Some(mut val_a), Some(mut val_b), Some(mut val_c)) = (a, b, c) {
                
                val_a &= !1;
                val_b &= !1;
                val_c &= !1;

                let resultado: u64 = ((val_c as u64) << 32) | ((val_b as u64) << 16) | (val_a as u64);

                println!("{}", resultado);
            }
        }
    }
}