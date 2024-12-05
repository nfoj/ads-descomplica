Exemplo:

```

Algoritmo "Funcao"

funcao imprimir:caracter

var frase:caracter

inicio
	frase <- "Exemplo"
	retorne frase
fimfuncao

var


Inicio

escreval("Mesangem da função: ")
escreval imprimir

Fimalgoritmo

```

Exemplo:

```

algoritmo "Procedimento"

funcao soma:inteiro

var aux: inteiro
	
inicio
	aux <- n + m
	retorne aux
fimfuncao

var
n, m: inteiro
res: inteiro

Inicio
	n <- 4
	m <- -9
	res <- soma
	escreva(res)
Fimalgoritmo

```


Exemplo:

```

algoritmo "Procedimento"

funcao mult:inteiro

var aux: inteiro
	
inicio
	aux <- n * m
	retorne aux
fimfuncao

var
n, m: inteiro
res: inteiro

Inicio
	n <- 1
	m <- 2
	res <- mult
	escreva(res)
Fimalgoritmo

```


# Problemas

01
Leia o texto abaixo:

O volume de uma esfera é calculada por 
V
=
4
3
π
r
3
V= 
3
4
​
 πr 
3
 

O código abaixo foi criado para implementar um programa que informa o valor do volume, em cm³, após o usuário digitar o valor do raio, em cm.



```
Algoritmo “volume”

Var
// Seção de Declarações das variáveis
r, V: real

funcao volume: real
var
b: real;
inicio

b <- r^3*(4/3)*pi
retorne b
fimfuncao

Inicio
// Seção de Comandos, procedimento, funções, operadores, etc…

Escreval(“Insira o valor do raio em cm”)

Leia (r)

V <- r^3*(4/3)*pi

Escreval("O Volume de uma esfera de raio ", r, " cm é “, V, " cm³”)

Fimalgoritmo

```


Com base no algoritmo acima e nos conteúdos abordados sobre Funções, assinale a alternativa que expressa corretamente o resultado que será impresso ao executar o procedimento, caso o usuário insira o valor 3 para o raio “r”

113,097 cm³


02

Com base em Funções verifique se as afirmativas são verdadeiras (V) ou falsas (F)

( ) Funções não são módulos, somente Procedimentos.
( ) Função seu único objetivo é mostrar dados na tela.
( ) Função é um comando.
( ) Funções são considerados módulos ou sub-rotinas.

Agora, assinale a alternativa que apresenta a sequência correta:​​

​FFFV​


03
(FCC - Adaptada) Leia o trecho a seguir:

“Normalmente as funções retornam algum resultado para quem as chamou. Assim, a informação corre nos dois sentidos, entre quem chamou e a função chamada […] mas podemos obter mais de uma resposta se essa resposta vier encapsulada.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, 111-113.

Com base no texto e nos seus conhecimentos sobre Funções, analise as afirmativas a seguir e assinale V para a(s) verdadeira(s) e F para a(s) falsa(s).

I. ( ) A sintaxe apresentada abaixo é utilizada para funções:

funcao <nome da função> :
var
inicio
<sequência de comandos>
retorne < valor >
fimfuncao

II. ( ) A função é um tipo de sub-rotina que retorna algum valor.
III. ( ) A função f1, escrita em pseudocódigo abaixo, recebe uma variável real e retorna um valor inteiro.

funcao f1 (N:inteiro) : real
var
inicio
se (N<=1)
então retorna 1
senão retorna (N* f1 (N-1))
fimse
fim

IV. ( ) A função f1, cujo pseudocódigo foi descrito acima, é executada apenas uma vez, já que em seu corpo existe apenas um comando condicional.

Agora, assinale a alternativa que apresenta a sequência correta:
​
V, V, F, F


04
Leia o pseudocódigo abaixo:

```

funcao f2 (N:inteiro) : real
var i: inteiro
result: real
inicio
result <- 1
para (i<- 2 até N passo 1) faça
result <- result*i
fimpara
retorna result
fimfuncao

```

Com base no algoritmo mostrado acima e nos seus conhecimentos sobre Funções, é correto afirmar que a função apresenta, respectivamente:

​Uma variável de entrada e uma de saída


05
Leia o trecho a seguir:
“[…] programar é muito mais que apenas escrever automaticamente alguns comandos em um editor de texto, é muito mais que saber a sintaxe de uma linguagem. Programar é uma atividade intelectual. Para isso, você precisa pensar, estudar, buscar soluções de problemas. Diversas áreas do conhecimento podem ajudá-lo a buscar melhores soluções. Não despreze nada.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, 113.

Com base no texto acima e nos conteúdos abordados sobre Funções, analise as asserções a seguir e a relação proposta entre elas:

I. Uma função é um bloco de programa contendo início e fim, sendo identificada por um nome de referência pelo qual se fará uso da função pelo programa principal ou pelo trecho que faz a chamada dessa função.

Porque

II. A diferença entre função e procedimento está no fato de que a função sempre retorna um valor como resposta de sua operação e o procedimento não.

​As asserções I e II são proposições verdadeiras, mas a II não é uma justificativa correta da I


06
​Qual é a diferença entre Procedimentos e Funções?

Funções retornam valor e Procedimento não retornam valor