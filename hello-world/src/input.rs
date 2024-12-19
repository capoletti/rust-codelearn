use std::io;

pub(crate) fn input() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Digite o primeiro número:");
    io::stdin().read_line(&mut input1).expect("Falha ao ler a linha");
    let num1: i32 = input1.trim().parse().expect("Por favor, digite um número válido");

    println!("Digite o segundo número:");
    io::stdin().read_line(&mut input2).expect("Falha ao ler a linha");
    let num2: i32 = input2.trim().parse().expect("Por favor, digite um número válido");

    if num1 != num2 {
        println!("O maior número é: {}", maior(num1, num2));
    } else {
        println!("Os dois números são iguais");
    }
}

fn maior(num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}