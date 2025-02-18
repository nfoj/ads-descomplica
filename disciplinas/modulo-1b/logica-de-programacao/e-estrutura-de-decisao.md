# Estruturas de Decisão

- Algoritmos de controle de fluxo

```
  Algoritmo "Exemplo"

  Var 

  idade:inteiro

  Inicio

  Escreval("Digite sua idade: ")
  Leia(idade)
  
  Se ((idade > 17) E (idade < 60)) ENTAO
    Escreva(idade)
  Fimse

  Fimalgoritmo
    
```

- Caso

```
  Algoritmo "Exemplo"

  Var
  
  op:inteiro
  num, resultado: real

  Inicio

  Escreval("Opcoes")

  Escreval("1 - Calcular o dobro do numero")

  Escreval("2 - Calcular o triplo do numero")

  Escreval("Escolha uma opcao!")

  Leia(op)
  Escreva("Digite um numero")
  Leia(op)

  Escolha(op)
    Caso 1
      resultado <- num*2
    Caso 2
      resultado <- num*3
  FimEscolha

  Escreva(resultado)
  FimAlgoritmo
  
```

# Problema

01

(FAURGS - Adaptado) Leia o trecho a seguir:

“O que é um texto? Usamos tanto no nosso dia a dia que não paramos para pensar no que consiste um texto escrito na tela de um computador. Um texto é uma sequência de caracteres. Esta é uma definição óbvia, porém importante. Esses caracteres podem ser as letras comuns do alfabeto, mas também podem ser símbolos, sinais de pontuação, números e até caracteres que não são visíveis diretamente, mas cujo efeito podemos perceber.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 68.

Com base no texto acima e nos conteúdos abordados sobre Tomada de Decisão, considere a tabela a seguir, que contém dados do IMC (Índice de Massa Corporal) de uma pessoa, com uma escala típica de valores, dentre as várias existentes.

A fórmula para o cálculo do IMC é
IMC = PESO/ ALTURA²

Considerando a expressão acima e de acordo com a tabela, o comando
de tomada de decisão, em pseudocódigo, que expressa corretamente a
lógica para calcular e imprimir o resultado do IMC para um dos dois casos
extremos (subpeso severo ou obesidade mórbida) é:

​se IMC <16 OR IMC >40 então IMPRIMIR IMC​


02

(CESPE / CEBRASPE - Adaptada) Leia o trecho a seguir:
“Imagine a seguinte situação: um programa que apresenta a média escolar de um aluno. Até aqui, muito simples, mas além de calcular a média, o programa deve apresentar se ele está aprovado ou reprovado, segundo a análise de sua média. Será necessário verificar a média do aluno para então tomar uma decisão no sentido de apresentar a sua real situação: aprovado ou reprovado. Para solucionar o problema proposto, é necessário trabalhar uma nova instrução […] Sendo a condição Verdadeira, serão executadas todas as instruções que estejam entre o comando se…entao e o comando fimse, e para condição Falsa senão… e o comando fimse.” (grifo do autor)

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 60.

A partir do texto acima e dos conhecimentos sobre Tomada de Decisão, analise o diagrama a seguir:

​A estrutura lógica presente no diagrama apresentado é do tipo:​

​SE ENTÃO SENÃO​

03

(IBFC - Adaptada) Leia o trecho a seguir:
“A tomada de decisão executada por um computador estabelece uma ação de desvio na operação do fluxo do programa. Desta forma, um determinado trecho do programa pode realizar uma ou outra tarefa de processamento.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 59.

A partir do texto e dos conhecimentos sobre Tomada de Decisão, sabendo que é comum que em linguagem de programação sejam aplicados diferentes tipos de dados; aqueles que são usados como resultados de expressões condicionais, possuindo apenas dois valores, um correspondente a verdadeiro e outro a falso, são do tipo:

Lógico​

04

Leia o trecho a seguir:
“A forma mais básica de tomada de decisão é o esquema que faz a seleção entre duas alternativas[…] Passando o fluxograma para pseudocódigo, temos: ‘Se uma condição for verdadeira faça algo senão faça outra coisa’.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 82.

Com base no texto acima e nos conteúdos abordados sobre Tomada de Decisão, analise o diagrama (Fluxograma) a seguir.

indica a estrutura de Decisão

05

Leia o trecho a seguir:

Um grupo de desenvolvedores está discutindo sobre a estrutura de controle tomada de decisão. A estrutura em questão apresenta as seguintes características: para verificar uma condição o usuário deve inserir um valor na tela, se o valor estiver correto, será apresentada a mensagem “Bem-vindo”.

Com base no texto e nos seus conhecimentos sobre Tomada de Decisão, considere a afirmativas a seguir:

I - A tomada de decisão em questão apresenta uma estrutura composta, com os comandos SE, SENAO e FIMSE

II - A tela exibirá a mensagem Bem-vindo se a condição for Verdadeira, de modo que que serão executadas todas as instruções que estejam entre o comando SE ENTAO e o comando FIMSE

III - O seguinte problema tem a mesma estrutura da tomada de decisão discutida pelos desenvolvedores: ler dois valores numéricos, efetuar a adição e apresentar o seu resultado caso o valor somado seja maior que 10

IV - Se a condição for Falsa não aparecerá a mensagem Bem-vindo, porque serão executadas as instruções que estiverem após o comando FIMSE

Está correto o que se afirma em:

II, III e IV​

06

Leia o trecho a seguir:
“Existem casos em que é necessário estabelecer verificações de condições sucessivas, em que uma ação pode ser executada se um conjunto anterior de instruções ou condições for satisfeito. Sendo a ação executada, ela pode ainda estabelecer novas condições, o que significa fazer uso de uma condição dentro de outra condição.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 69.

A partir do texto e dos seus conhecimentos sobre Tomada de Decisão, a estrutura descrita torna possível a realização de múltiplas verificações condicionais em um pseudocódigo, sendo conhecida como:

Desvio Condicional Aninhado ou Encadeado​
