fn main(){

    /* Aula 01 - Introdução */

    //O "println" é uma macro.
    println!("Hello World!");


    /* Aula 02 - Variáveis e Constantes */

    println!("Imprimir a quantidade de horas trabalhadas!");

    const SECONDS_IN_MINUTE: u32 = 60;
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;

    println!("Trabalhou {} segundos!", total_em_segundos);

    // {
    //     //Estamos criando um subescopo, assim, o escopo interno terá
    //     //preferência sobre o escopo externo.
    //     let total = 40;
    //     println!("Trabalhou {} horas!", total);
    // }


    /* Aula 03 - Tipos Primitivos em Rustlang */

    /* Primitivos Escalares (armazenam apenas um único valor) */

    // let x = 5_u8;
    // let y: u8 = x - 20; -> Um overflow será gerado, pois o valor de y será maior do que o range de um u8.

    let numeroGrande = 199_456_899; // Melhorando a legibilidade.

    let x: f64 = 42.1; //Um float em Rust.

    let x = true; //Um booleano em Rust.

    let letra = 'a'; //Um char em Rust.

    /* Tipos Compostos (agrupar mais de um valor) */

    //Tupla: A tupla possui um tamanho fixo.
    let numbers: (i32, i32, f64) = (1, 2, 3.5);
    println!("{:?}", numbers);
    println!("{:#?}", numbers.2); //Acessando o índice 2 da tupla.

    let (a, b, c) = numbers; //Desestruturando a tupla.

    //Array: O array possui um tamanho fixo.

    let array: [i32;4] = [1, 2, 3, 4];
    println!("{:?}", array);
    println!("{:#?}", array[2]); //Acessando o índice 2 do array.

    //Slice:

    println!("{:?}", &array[1..3]); //Acessando o índice 1 até o 3 do array.
}