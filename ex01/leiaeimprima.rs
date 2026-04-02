use std::io;

fn main() {
    let mut input = String::new();

    // Unwrap() é utilizado como um tratamento de erro no Rust
    // O read_line(&mut input) pega tudo que foi digitado pelo usuário
    // e joga dentro de input criado acima
    io::stdin().read_line(&mut input).unwrap();

    // Separa as palavras onde tem espaços e transforma numa Collection
    let palavras: Vec<&str> = input.split_whitespace().collect();

    if palavras.len() == 2 {
        // Parse() tenta transforma a String em outro tipo
        // Aqui o Rust deduz pelo let: i64 qual é o tipo
        let a: i64 = palavras[0].parse().unwrap();

        // Chars() transforma a String numa sequência de caracteres
        // Next() pega o próximo de um iterador
        let b: char = palavras[1].chars().next().unwrap();

        
        println!("{}", a);
        println!("{}", b);
    }

/*
De cara, tive um problema com o compilador de escopo, pois tentei printar
os valores fora do escopo if, mas o Rust não deixa porque o escopo pai
não tem acesso ao escopo filho, o contrário funciona, por isso consigo
usar palavras dentro do if, mas não a e b fora dele.
*/
}
