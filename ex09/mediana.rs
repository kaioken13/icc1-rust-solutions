use std::io;

fn main() {
    // Read k (size of the array)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Invalid input");
    
    // Read the numbers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    
    // Sort the array
    arr.sort();
    
    // Calculate the median
    let median: f64;
    if k % 2 == 1 {
        // Odd number of elements
        median = arr[k / 2] as f64;
    } else {
        // Even number of elements
        median = (arr[k / 2 - 1] + arr[k / 2]) as f64 / 2.0;
    }
    
    // Print the median with one decimal place
    println!("{:.1}", median);
}