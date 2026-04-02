use std::io;

fn main() {
    let mut input = String::new();

    // read_line(&mut input) captures all user input from stdin 
    // and stores it in the 'input' buffer created above.
    // .unwrap() is used here for basic error handling (standard in learning phases).
    io::stdin().read_line(&mut input).unwrap();

    // split_whitespace() divides the string by spaces/tabs/newlines.
    // .collect() gathers these parts into a Vector (Collection) of string slices.
    let palavras: Vec<&str> = input.split_whitespace().collect();

    if palavras.len() == 2 {
        // .parse() attempts to convert the String into another type.
        // Rust infers the target type (i64) from the variable declaration.
        let a: i64 = palavras[0].parse().unwrap();

        // .chars() transforms the string into a sequence of characters.
        // .next() retrieves the first element from this iterator.
        let b: char = palavras[1].chars().next().unwrap();

        
        println!("{}", a);
        println!("{}", b);
    }

/*
    LEARNING LOG / TROUBLESHOOTING:
    Initially, I faced a compiler error regarding variable scope. 
    I tried to print 'a' and 'b' outside the 'if' block, but Rust's ownership 
    and scope rules prevented it. 
    Key Concept: A parent scope cannot access variables created within a 
    child scope (the 'if' block), although the child can access variables 
    from the parent (like the 'words' vector).
*/
}
