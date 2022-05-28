# Formula resolvente em Rust
Fiz este programa com a minha filha para a ensinar a programar em Rust.


## Descrição

Programa em Rust que calcula a formula resolvente das raízes de um polinómio de segundo grau. Este programa foi feito no decurso de ensinar à minha filha a programar em Rust. O programa tem duas funções main, main_1 e main_2 para executar tem de se comentar uma delas entre // main_1() ou // main_2(), contudo eu decidi fazer uma mais simples com a minha filha e outra mais complexa mas mais correta. Para ela perceber como seria essa função. Em outras linguagens como o Python seria um pouco mais simples.


## Este programa tem as seguintes funções

O código está num unico ficheiro .rs em [main.rs](./src/main.rs) <br>
<br>


```
print_descricao_programa() - Função que escreve no terminal uma descrição do que
                             faz o programa.

calcula_formula_resolvente(a, b, c) - A função principal do programa. Ela
                                      calcula as raízes reais de um polinómio
                                      de 2º grau pela formula resolvente.
           Retorna uma lista ou seja um vector Vec<f32> com zero, uma ou
           duas raízes.

print_raizes(raizes_vec) - A função que recebe uma lista com as raízes e
                           que escreve no ecrã uma descrição do tipo de
                           raíz e o valor das raizes caso existam.

main() - Esta função é o ponto de entrada para o programa, é a primeira
         função que vai ser executada no nosso código e é a partir do
         código que está dentro desta função que as outras funções vão
         ser chamadas e executadas, umas dentro das outras.

main_1() - Função mais simples.
           Função que faz o mesmo que a função main_2() e que pede os
           números de "a", "b" e "c" ao utilizador. Ou "s" para sair
           do programa.
           Os número podem ser inteiros, ou decimais e positivos ou
           negativos. 
            
main_2() - Função mais complexa.
           Função que faz o mesmo que a função main_1() e que pede os
           números de "a", "b" e "c" ao utilizador. Ou "s" para sair
           do programa.
           Os número podem ser inteiros, ou decimais e positivos ou
           negativos. Esta função faz uso de returns dentro de loops
           com labels exteriores. 

```


## Licença
MIT Open Source


## Divirtam-se a programar Rust
Cumprimentos, <br>
Diana e João <br>