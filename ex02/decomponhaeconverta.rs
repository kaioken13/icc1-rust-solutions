use std::io;

fn main() {
    
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let palavras: Vec<&str> = input.split_whitespace().collect();

    if palavras.len() == 2 {
        let t: u64 = palavras[0].parse().unwrap();
        let s: char = palavras[1].chars().next().unwrap();

        let micro_por_segundo: u64 = 1_000_000;
        let segundos_por_hora: u64 = 3600;
        let horas_por_dia: u64 = 24;
        let dias_por_semana: u64 = 7;

        let segundos_totais: u64 = t / micro_por_segundo;
        let horas_totais: u64 = segundos_totais / segundos_por_hora;

        let semanas: u64 = horas_totais / (horas_por_dia * dias_por_semana);
        let resto_horas_semanas: u64 = horas_totais % (horas_por_dia * dias_por_semana);

        let dias: u64 = resto_horas_semanas / horas_por_dia;
        let horas: u64 = resto_horas_semanas % horas_por_dia;

        println!("Decomposição do setor {}", s);
        println!("{} Semana(s)", semanas);
        println!("{} Dia(s)", dias);
        println!("{} Hora(s)", horas);
    }

}