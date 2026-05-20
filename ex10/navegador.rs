use std::io;

fn main() {
    // Read N (size of the array)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // Read the N integers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    
    // Read Q (number of commands)
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let q: usize = input.trim().parse().expect("Invalid input");
    
    // Start pointer pointing to the first element
    let base_ptr = arr.as_mut_ptr();
    let mut ptr = base_ptr;
    
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "E" => {
                let x: isize = parts[1].parse().expect("Invalid number");
                // Move by X elements (each element is sizeof(int) bytes)
                ptr = unsafe { ptr.offset(x) };
            }
            "B" => {
                let y: isize = parts[1].parse().expect("Invalid number");
                // Move by Y absolute bytes using u8 pointer
                ptr = unsafe { (ptr as *mut u8).offset(y) as *mut i32 };
            }
            "P" => {
                // Print the integer at current address (guaranteed aligned)
                unsafe {
                    println!("{}", *ptr);
                }
            }
            "D" => {
                // Print distance in bytes from base address
                let distance = unsafe { (ptr as *mut u8).offset_from(base_ptr as *mut u8) };
                println!("{}", distance);
            }
            _ => {}
        }
    }
}