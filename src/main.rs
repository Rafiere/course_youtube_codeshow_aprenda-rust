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
}