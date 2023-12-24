use std::io; //Importando a biblioteca de entrada e saída do Rust.

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


    /* Aula 04 - Memórias Static, Stack e Heap */

    static _Y: i32 = 5; //Alocado na memória estática.

    let x = 5; //Alocado na memória stack.
    let z = true; //Alocado na memória stack.


    /* Aula 05 - String - Textos e Caracteres */

    "Rafa"; //String literal. Nesse código, o Rust irá converter para binário e armazenará essa string literal dentro do "code segment", que é uma parte da memória estática.

    let nome = "Rafa"; //A variável "nome" recebe o tipo "&str". O tipo "&str" é o "string slice" ou "string reference". É uma referência para uma string que está armazenada em outro lugar da memória.

    let mut s = String::new(); //Criando uma string vazia que pode ser modificada e será armazenada na "heap" pois o seu tamanho não é conhecido.
    s.push('a'); //Adicionando um caractere na string.
    s.push_str("b"); //Adicionando uma string na string.
    println!("{}", s);

    //Criando uma string do tipo String:

    let s: String = "Rafa".to_string(); //Criando uma string do tipo String.

    let nome2 = String::from("Rafa");


    /* Aula 06 - Respondendo Perguntas - Compilação Otimizada, Operações em String */

    fn main2(){

        let mut s = String::new();
        println!("Digite um texto: ");

        io::stdin()
            .read_line(&mut s)
            .expect("Falha ao ler o texto!");

        println!("Você digitou: {}", s);
        println!("Quantidade de caracteres {}", s.trim().len()); //O "trim" é para tirar o caractere representado pelo "Enter".
        println!("Você digitou: {}", s.to_uppercase());
        println!("Você digitou: {}", s.replace("n", "x"));

        let nums: Vec<i32> = s.split(",")
            .map(| c | c.trim().parse().expect("Erro!"))
            .collect();

        println!("Você digitou: {:?}", nums);

        let result: i32 = nums.iter().sum();

        println!("A soma dos números é: {}", result);

    }

    main2();

    /* Aula 07 - Funções */

    fn say_hello(name: &str){
        println!("Hello {name}");
    }

    say_hello("Rafa"); //Estamos chamando a função.

    fn add_numbers(x: i32, y: i32) -> i32 {
        // return x + y; //Estamos utilizando o "return". Ele é opcional, já que é uma expressão
        // e, consequentemente, gera um valor, assim, a última expressão sempre será retornada.
        x + y
    }

    fn main3(){

        let input = "56 65 58 48 59 56 87 23";

        let result: Vec<i32> = input
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap()) //Estamos criando uma função anônima, ou seja, um closure.
            .map(|n| n * 2)
            .collect();

        println!("Teste!");
        println!("{:?}", result);
    }

    main3();


    /* Aula 08 - Ownership e Borrowing */

    fn main4(){

        /* Tipos Copy: */

        let a: i32 = 1; //São valores do tipo "copy", pois são valores que possuem um tamanho fixo. "i32", "f64" e etc. Tipos que não implementam o "trait" chamado "Copy" não são copiados.

        // let b = a; //O "Rust" fará a cópias dos valores de "a" para "b", assim, eles são independentes.

        //Para evitarmos a cópia, podemos, explicitamente, pedir por uma referência.

        let b = &a; //Estamos criando uma referência para "a". Assim, "b" não é uma cópia de "a", mas sim uma referência para "a".

        //Armazenando na heap:

        let c = String::from("Teste"); //Estamos armazenando na "heap".

        let d = c; //Estamos movendo a PROPRIEDADE, o ownership de "c" para "d", assim, "c" não é mais válido.

        // println!("{}", c); //O "c" não é mais válido e o borrow checker não irá permitir a sua utilização.
    }

    main4();

    fn say_hello2(name: String){
        println!("Hello {name}");
    }

    fn say_goodbye2(name: String){
        println!("Hello {name}");
    }

    fn main5(){

        let name = "Rafa"; //&str - static
        say_hello(name); //É um valor "copy". Estamos copiando esse texto para dentro da função.

        //Usando a heap:

        let name2 = String::from("Rafa"); //String - heap
        say_hello2(name2); //Estamos movendo a propriedade de "name2" para o argumento da função "say_hello2". Assim, "name2" não é mais válido.
        // say_goodbye2(name2); //O "name2" não é mais válido e o borrow checker não irá permitir a sua utilização.

        //Como resolver o problema acima:

        let name3 = String::from("Rafa");
        // say_hello(name.clone()); //Estamos clonando o valor de "name3" e passando para a função "say_hello". Assim, "name3" continua válido.
        // say_goodbye2(name3); //Estamos movendo a propriedade de "name3" para o argumento da função "say_goodbye2". Assim, "name3" não é mais válido.

        //O "clone" é custoso, pois ele faz a alocação na memória heap e copia o valor para lá.

        //Resolvendo sem o "clone":
        let name4 = String::from("Rafa");
        say_hello(&name4); //Estamos passando uma referência para a função "say_hello". Assim, "name4" continua válido.
        say_goodbye2(name4); //Estamos movendo a propriedade de "name4" para o argumento da função "say_goodbye2". Assim, "name4" não é mais válido.

    }
}