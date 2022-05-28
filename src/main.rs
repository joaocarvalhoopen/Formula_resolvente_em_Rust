// Nome:    Formula resolvente em Rust
// Data:    2022.05.28
// Autores: Diana e João
// Descrição: Programa em Rust que calcula a formula resolvente das raízes de
//            um polinómio de segundo grau.
//            Este programa foi feito no decurso de ensinar à minha filha a
//            programar em Rust. 
//            O programa tem duas funções main, main_1 e main_2 para executar
//            tem de se comentar uma delas entre // main_1() ou // main_2(),
//            contudo eu decidi fazer uma mais simples com a minha filha e
//            outra mais complexa mas mais correta. Para ela perceber como
//            seria essa função. Em outras linguagens como o Python seria
//            um pouco mais simples.
//
// Este programa tem as seguintes funções:
//
//  print_descricao_programa() - Função que escreve no terminal uma descrição do que
//                               faz o programa.
//
//  calcula_formula_resolvente(a, b, c) - A função principal do programa. Ela
//                                        calcula as raízes reais de um polinómio
//                                        de 2º grau pela formula resolvente.
//            Retorna uma lista ou seja um vector Vec<f32> com zero, uma ou
//            duas raízes.
//
//  print_raizes(raizes_vec) - A função que recebe uma lista com as raízes e
//                             que escreve no ecrã uma descrição do tipo de
//                             raíz e o valor das raizes caso existam.
//
//  main() - Esta função é o ponto de entrada para o programa, é a primeira
//           função que vai ser executada no nosso código e é a partir do
//           código que está dentro desta função que as outras funções vão
//           ser chamadas e executadas, umas dentro das outras.
//
//  main_1() - Função mais simples.
//             Função que faz o mesmo que a função main_2() e que pede os
//             números de "a", "b" e "c" ao utilizador. Ou "s" para sair
//             do programa.
//             Os número podem ser inteiros, ou decimais e positivos ou
//             negativos. 
//             
//  main_2() - Função mais complexa.
//             Função que faz o mesmo que a função main_1() e que pede os
//             números de "a", "b" e "c" ao utilizador. Ou "s" para sair
//             do programa.
//             Os número podem ser inteiros, ou decimais e positivos ou
//             negativos. Esta função faz uso de returns dentro de loops
//             com labels exteriores.
//
// Licença: MIT Open Source
//

use std::io;
use std::io::Write;

fn print_descricao_programa() {

    let descricao_string: String = "\nPrograma que calcula a formula resolvente, \
recebe três numeros correspondentes ao valor escalar de cada termo de um polinómio de 2º grau.

  Formula do polinómio:  

        a * x^2 + b * x + c = 0

  Formula resolvente para obter as raízes x:

        delta = b * b - 4 * a * c;

             -b +- sqrt(delta)
        X =  -----------------
                  2 * a

  Carregue em 's' - Sair do programa.
".to_string();

    println!("{}", descricao_string);
}

// Função que calcula a formula resolvente, recebe três numeros correspondentes
// ao valor escalar de cada termo de um polinómio de 2º grau.
//
// Formula do polinomio:  
//
//             a * x^2 + b * x + c = 0
//
// Formula resolvente para obter as raizes x:
//
//       delta = b * b - 4 * a * c;
//
//            -b +- sqrt(delta)
//       X =  -----------------
//                 2 * a
//
//
fn calcula_formula_resolvente(a: f32, b: f32, c: f32) ->  Vec<f32> { 
    let mut list: Vec<f32> = Vec::new();
    let delta = b * b - 4.0 * a * c; 
    if delta < 0.0 {
        // Zero raizes por tanto equação impossivel!
        // return lista vazia; Não preenchemos nada. 
    } else if delta <= 0.000001 {
    // if delta == 0.0 {
        // Uma raiz por tanto equação possivel!
        let x_1 = -b / (2.0 * a);
        list.push(x_1);
    } else if delta > 0.0 {
        // Duas raizes por tanto equação possivel!
        let x_1 = (-b + f32::sqrt(delta) ) / (2.0 * a);
        let x_2 = (-b - f32::sqrt(delta) ) / (2.0 * a);
        list.push(x_1);
        list.push(x_2);
    }
    list
}

// Função que faz o println do resultado de uma raiz no terminal.
fn print_raizes(raizes_vec: Vec<f32>) {
    let comprimento = raizes_vec.len();
    match comprimento {
        0 => println!("\n Zero raizes reais, por tanto equação impossivel!\n A linha do grafico da função não toca no eixo dos XX."),
        1 => {
                println!("\n Uma raiz real, por tanto equação possivel!\n A linha do grafico da função toca no eixo dos XX em x_1.");
                let x_1 = raizes_vec[0];
                println!(" x_1 = {}", x_1);
             },
        2 => {
                println!("\n Duas raizes reais, por tanto equação possivel!\n A linha do grafico da função intercepta o eixo dos XX em x_1 e em x_2.");
                let x_1 = raizes_vec[0];
                let x_2 = raizes_vec[1];
                println!(" x_1 = {}", x_1);
                println!(" x_2 = {}", x_2);
             },
        _ => (),
    }
}


fn main() {
    // NOTA_IMPORTANTE: Comentar só uma das seguintes funções.

    // Versão mais simples do input dos números pelo utilizador.
    // main_1();
        
    // Versão mais complexa mas mais correcta do input dos números
    // pelo utilizador.
    main_2();
}


//***********
//   Main 1
//***********

fn main_1() {
    // Formula do polinomio:  a * x^2 + b * x + c = 0 
    let mut input_string: String = String::new();

    'external: loop {

        print_descricao_programa();

        print!("\n a = ");
        // Porque a linha anterior não termina no caracter \n,
        // temos de fazer o flush manual.
        io::stdout().lock().flush();
        input_string.clear();
        let a: f32 = match io::stdin().read_line(&mut input_string) {
                        Err(erro) => {
                                println!(" Erro: Ocorreu um erro. {}", erro);
                                break;
                            },
                        Ok(_) => {
                            if input_string.trim().to_lowercase() == "s" {
                                break;
                            }
                            let num_res = input_string.trim().parse::<f32>();
                            if num_res.is_err() {
                                println!(" Erro: O numero não é valido.");
                                continue;
                            };
                            num_res.unwrap()
                        }
                    };

        print!("\n b = ");
        // Porque a linha anterior não termina no caracter \n,
        // temos de fazer o flush manual.
        io::stdout().lock().flush();
        input_string.clear();
        let b: f32 = match io::stdin().read_line(&mut input_string) {
                        Err(erro) => {
                                println!(" Erro: Ocorreu um erro. {}", erro);
                                break;
                            },
                        Ok(_) => {
                            if input_string.trim() == "s" {
                                break;
                            }
                            let num_res = input_string.trim().parse::<f32>();
                            if num_res.is_err() {
                                println!(" Erro: O numero não é valido.");
                                continue;
                            };
                            num_res.unwrap()
                        }
                    };

        print!("\n c = ");
        // Porque a linha anterior não termina no caracter \n, 
        // temos de fazer o flush manual.
        io::stdout().lock().flush();
        input_string.clear();
        let c: f32 = match io::stdin().read_line(&mut input_string) {
                        Err(erro) => {
                                println!(" Erro: Ocorreu um erro. {}", erro);
                                break;
                            },
                        Ok(_) => {
                            if input_string.trim() == "s" {
                                break;
                            }
                            let num_res = input_string.trim().parse::<f32>();
                            if num_res.is_err() {
                                println!(" Erro: O numero não é valido.");
                                continue;
                            };
                            num_res.unwrap()
                        }
                    };

        // let a = 5.0;
        // let b = 2.0;
        // let c = -1.0;

        let raizes_vec: Vec<f32> = calcula_formula_resolvente(a, b, c); 
        print_raizes(raizes_vec);

        println!("\n Para continuar carregue em enter.");
        input_string.clear();
        let _ = io::stdin().read_line(&mut input_string);
    }

    println!("\nFim do programa!");
}


//***********
//   Main 2
//***********

fn main_2() {
    // Formula do polinomio:  a * x^2 + b * x + c = 0 
    let mut input_string: String = String::new();

    'external: loop {
        print_descricao_programa();
        let mut input_array: [f32; 3] = [0.0, 0.0, 0.0]; 
        
        for i in 0..3 {
        
            let mut flag_numero_invalido = true;
            while flag_numero_invalido {
        
                match i {
                    0 => print!("\n a = "),
                    1 => print!("\n b = "),
                    2 => print!("\n c = "),
                    _ => (),
                }
    
                let _ = io::stdout().lock().flush(); // Porque a linha anterior não termina no caracter \n .
                input_string.clear();
            
                input_array[i] = match io::stdin().read_line(&mut input_string) {
                                Err(erro) => {
                                        println!(" Erro: Ocorreu um erro na stream de stdin. {}", erro);
                                        break 'external;
                                    },
                                Ok(_) => {
                                    if input_string.trim().to_lowercase() == "s" {
                                        // Para sair do programa.
                                        break 'external;
                                    }
                                    let num_res = input_string.trim().parse::<f32>();
                                    if num_res.is_err() {
                                        println!(" Erro: O numero não é valido.");
                                        continue;
                                    };
                                    flag_numero_invalido = false;
                                    num_res.unwrap()
                                }
                            };

                }
        }

        let a: f32 = input_array[0];
        let b: f32 = input_array[1];
        let c: f32 = input_array[2];

        // let a = 5.0;
        // let b = 2.0;
        // let c = -1.0;

        let raizes_vec: Vec<f32> = calcula_formula_resolvente(a, b, c); 
        print_raizes(raizes_vec);

        println!("\n Para continuar carregue em enter.");
        input_string.clear();
        let _ = io::stdin().read_line(&mut input_string);
    }

    println!("\nFim do programa!");
}


