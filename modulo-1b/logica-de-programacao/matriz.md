Exemplo:

```
Algoritmo "matriz"

Var
	numero: vetor[1..3, 1..2] de inteiro
	i: inteiro

Inicio
	PARA i de 1 ATE 3 FACA
		ESCREVAL("Digite o valor para a posicao ", i, ", 1: ")
		LEIA(numero[i, 1])
		ESCREVAL("Digite o valor para a posicao ", i, ", 2: ")
		LEIA(numero[i, 2])
	FIMPARA
Fimalgoritmo

```


Exemplo:

```
Algoritmo "matriz"

Var
	numero: vetor[1..3, 1..2] de inteiro
	i, j: inteiro

Inicio
	PARA i de 1 ATE 3 FACA
		PARA j de 1 ATE 2 FACA
			ESCREVAL("Digite da linha ", i, " e coluna ", j, ": ")
		LEIA(numeros[i,j])
	FIMPARA
Fimalgoritmo

```


# Problema

01
Qual é o valor contido na posição da matriz m[2,1]?
180


02
Leia o texto a seguir:
“Uma matriz é uma generalização do conceito de vetor. Enquanto o vetor possui apenas um índice, ou seja, apenas uma dimensão, chamado, portanto, de unidimensional, a matriz pode conter múltiplas dimensões.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 162.

Com base no texto acima e nos conteúdos abordados sobre Matrizes, analise as asserções a seguir e a relação proposta entre elas:

I. A forma correta de atribuir o valor 16 na posição [5,2] de uma matriz chamada aula é utilizando o comando aula[5,2] <- 16.

Porque

II. O comando que apresenta corretamente o elemento da posição [5,2] da matriz de nome aula é escreval(aula[5,2]).
As asserções I e II são proposições verdadeiras, mas a II não é uma justificativa correta da I



03
Leia o trecho a seguir:
Uma técnica de programação que permite trabalhar com o agrupamento de várias informações em uma mesma variável é o agrupamento do mesmo tipo de dado, e por esta razão, denomina-se estrutura de dado homogênea ou matriz.

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019.

Com base nessas informações e no conteúdo estudado sobre Matrizes, analise as figuras abaixo e associe-as com a dimensão das matrizes que elas descrevem

( ) Matriz 1 x 4
( ) Matriz 4 x 1
( ) Matriz 2 x 2
( ) Matriz 2 x 3

Agora, assinale a alternativa que apresenta a sequência correta:

4, 2, 1, 3


04
(Quadrix - Adaptada) Leia o trecho a seguir:

Dada uma declaração de matriz de duas dimensões e uma atribuição:

(1) declarar M[5,4] : literal
(2) M[4,3]￩"ABC"

A literal “ABC” foi armazenada na posição representada pelas dimensões linha e coluna:

​4 e 3


05
Leia o trecho a seguir:

“A matriz de uma dimensão (vetor) é a forma mais simples de tabela de valores com apenas uma coluna e várias linhas de dados, definida em uma única variável com tamanho específico.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 107.

Com base no conhecimento acerca de Variáveis, analise as afirmativas a seguir e assinale V para a(s) verdadeira(s) e F para a(s) falsa(s).

I. ( ) Os elementos de uma matriz podem ser de tipos de dados distintos.

II. ( ) As matrizes são referenciadas através de suas dimensões na seguinte ordem: conjunto e linhas.

III. ( ) A dimensão de uma matriz é formada por constantes inteiras e positivas.

IV. ( ) O nome dado a uma variável composta (matriz) segue as mesmas regras dos nomes dados a variáveis simples.

Agora, assinale a alternativa que apresenta a sequência correta:
​
F, F, V, V


06
Leia o trecho a seguir:
O gerente de uma loja de produtos de construção civil deseja fazer vários orçamentos a fim de averiguar o custo de uma compra de materiais para abastecer o estoque. Os três produtos P1, P2 e P3 são necessários nas quantidades Q1, Q2 e Q3, respectivamente. Ao pesquisar em cinco fornecedores, o gerente encontrou a tabela de preços apresentada na figura a seguir.

Figura 1 - Orçamentos dos fornecedores 1 a 5 para os produtos P1, P2 e P3.


Com base no texto acima e nos seus conhecimentos sobre Matrizes, considere as afirmativas a seguir:

I. A matriz dos orçamentos tem dimensão 5 x 3.

II. O valor disposto na posição [3, 2] é referente ao preço do fornecedor 2 para P3.

III. O valor disposto na posição [2, 4] é referente ao preço do fornecedor 2 para P2.

IV. O valor disposto na posição [1, 5] é referente ao preço do fornecedor 1 para P1.

V. Os valores dispostos nas posições [2, 1], [2, 3] e [2, 4] são idênticos.

VI. Os valores dispostos nas posições [3, 1] e [1, 4] são diferentes.

Está correto o que se afirma em:
​II e V
