use std::io;

/*
This code is a more Rustacean (idiomatic) version of the decomponhaeconverta.rs
In this code, it was used the if let pattern, which is "syntactic sugar" for match
which turns your code more productive
*/

// Constants in uppercase and outside of the main for clarity and reuse
const MICROS_PER_SEC: u64 = 1_000_000;
const SECS_PER_HOUR: u64 = 3_600;
const HOURS_PER_DAY: u64 = 24;
const DAYS_PER_WEEK: u64 = 7;
const HOURS_PER_WEEK: u64 = HOURS_PER_DAY * DAYS_PER_WEEK;

fn main() {
    let mut input = String::new();

    // .is_err returns a Result and if you got a error the code crashes
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let palavras: Vec<&str> = input.split_whitespace().collect();

    if let [raw_t, raw_s] = palavras.as_slice() {

        // Here I try to convert the values, if fails we don't proceed
        let total_micros = raw_t.parse::<u64>().ok();
        let sector = raw_s.chars().next();

        if let (Some(micros), Some(s)) = (total_micros, sector) {
            
            let total_hours = micros / (MICROS_PER_SEC * SECS_PER_HOUR);

            let weeks = total_hours / HOURS_PER_WEEK;
            let remaining_hours = total_hours % HOURS_PER_WEEK;

            let days = remaining_hours / HOURS_PER_DAY;
            let hours = remaining_hours % HOURS_PER_DAY;

            println!("Decomposição do setor {}", s);
            println!("{} Semana(s)", weeks);
            println!("{} Dia(s)", days);
            println!("{} Hora(s)", hours);
        }
    } 
}
