use std::io;

/// # Função Main
/// ## Responsavél por interagir Coletar, processar e exibir dados aos usuários.

/// ## Funções Presentes:
/// 1. **soma**
/// 2. **subt**
/// 3. **mult**
/// 4. **divi**
/// ## **Variaveis** -> **Oque Recebem**
/// * **opcao** -> entrada do usuário
/// * **sin** - > io::stdin()
/// * **n1** -> Número Primário
/// * **n2** -> Número Secundário
/// - **n1**, **n2** e **opcao** são convertidos para **i64**
fn main() {
    let mut opcao = String::new();

    let sin = io::stdin();

    let mut n1 = String::new();
    let mut n2 = String::new();

    println!(
        "Insira a opção desejada: 
    \n[1]Soma
    \n[2]Subtração
    \n[3]Multiplicação
    \n[4]Divisão\n"
    );

    sin.read_line(&mut opcao).expect("Falha ao Ler Entrada!");

    println!("Opção Desejada: {}", opcao);

    println!("Insira o número primário: ");

    sin.read_line(&mut n1).expect("Falha ao Ler Entrada!");

    println!("Insira o número Secundário: ");

    sin.read_line(&mut n2).expect("Falha ao Ler Entrada!");

    let n1: i64 = n1.trim().parse().expect("Por favor, digite um número!");

    let n2: i64 = n2.trim().parse().expect("Por favor, digite um número!");

    let opcao: i64 = opcao.trim().parse().expect("Por favor, digite um número!");

    if opcao == 1 {
        println!("A soma de {} e {} é igual a: {}", n1, n2, soma(n1, n2));
    } else if opcao == 2 {
        println!("A diferença de {} e {} é igual a: {}", n1, n2, subt(n1, n2));
    } else if opcao == 3 {
        println!("O produto de {} e {} é igual a: {}", n1, n2, mult(n1, n2));
    } else if opcao == 4 {
        divi(n1, n2);
    } else {
        println!("Opção Inválida!!!!");
    }
}

/// # Soma
/// ## Executa operação de soma.
/// * **x** -> **n1**
/// * **y** -> **n2**
/// * **result** -> **x + y**
fn soma(x: i64, y: i64) -> i64 {
    x + y
}
/// # Subt
/// ## Executa operação de subtração.
/// * **x** -> **n1**
/// * **y** -> **n2**
/// * **result** -> **y - x**
fn subt(x: i64, y: i64) -> i64 {
    let result: i64 = y - x;
    return result;
}

/// # Mult
/// ## Executa operação de multiplicação.
/// * **x** -> **n1**
/// * **y** -> **n2**
/// * **result** -> **x * y**
fn mult(x: i64, y: i64) -> i64 {
    let result: i64 = x + y;
    return result;
}

/// # Divi
/// ## Executa operação de divisão e resto.
/// * **x** -> **n1**
/// * **y** -> **n2**
/// * **result** ->  **x / y**
/// * **rst** -> **x % y**
fn divi(x: i64, y: i64) {
    let result: i64 = x / y;
    let rst = x % y;
    println!(
        "A divisão de {} e {} é igual a: {} e resto {}",
        x, y, result, rst
    );
}
