Exemplo:

```

Algoritmo "Exemplo"

funcao imprimir(texto:caracter):caracter

var
	frase:caracter

inicio
	frase <- texto
	retorne frase
fimfincao


var
	res:caracter

inicio
	escreval("Exemplo")
	res <- imprimir("Texto")
	esceva(res)
fimalgortimo 

```


```

algoritmo "Procedimento"

funcao soma (x, y: inteiro): inteiro
	
inicio
	retorne x + y
fimfuncao

var
	n, m, res: inteiro

Inicio
	n <- 4
	m <- -9
	res <- soma(n, m)
	escreva(res)
Fimalgoritmo

```


# Problema

01
Leia o pseudocódigo abaixo:

algoritmo “CALC_SOMA”
funcao soma(A, B : real) : real
inicio
var
SOMA: real
retorne SOMA <- A + B
fimfuncao
var
NUM1, NUM2, somaaux : real
inicio
escreva ("Informe o 1o. valor: " );
leia (NUM1)
escreva ("Informe o 2o. valor: " );
leia (NUM2)
somaaux = soma(NUM1, NUM2);
escreva ("Soma = ", somaaux);
fimalgoritmo

Com base no algoritmo mostrado acima e nos seus conhecimentos sobre Funções, analise as etapas que são executadas pelo programa, e ordene-as de acordo com a sequência em que ocorrem.

( ) O resultado é implicado na variável somaaux que faz o retorno à função soma.

( ) O programa principal lê as variáveis NUM1 e NUM2.

( ) O programa principal transfere os valores das variáveis NUM1 e NUM2 para os parâmetros A e B do tipo real.

( ) É processada a soma dos dois valores.

Agora, assinale a alternativa que apresenta a sequência correta:​
4, 1, 2, 3


02
Leia o texto abaixo:

A função soma, cujo pseudocódigo foi descrito na questão anterior, é representada graficamente através do diagrama de blocos mostrado abaixo.


​Com base no diagrama de blocos e nos seus conhecimentos sobre Funções, é correto afirmar que a função apresenta, respectivamente:

​Duas variáveis de entrada e uma de saída


03
Leia o pseudocódigo abaixo:

algoritmo “COMPARAÇÃO”

funcao compara(c, f : real) : lógico

inicio

retorna c = f

fimfuncao

var n1, n2 : real

inicio

escreva ("Informe o 1o. valor: " )

leia (n1)

escreva ("Informe o 2o. valor: ")

leia (n2)

se (compara(n1, n2)) então

escreva “Números iguais”

senao

escreva “Números diferentes”

fimse

fimalgoritmo

Com base no algoritmo mostrado acima e nos seus conhecimentos sobre Funções, analise as etapas que são executadas pelo programa, e ordene-as de acordo com a sequência em que ocorrem

( ) Efetuar a comparação para determinar se os valores são iguais ou diferentes
( ) Chama a função para retornar o resultado, que consiste numa condição falsa ou verdadeira após a validação da expressão na função
( ) O algoritmo solicita os valores n1 e n2 aos usuários
( ) As variáveis n1 e n2 são declaradas no VAR e são do tipo Real
Agora, assinale a alternativa que apresenta a sequência correta:

​4, 3, 2, 1


04
Leia o texto abaixo:

A função comparação, cujo pseudocódigo foi descrito na questão anterior, é representada graficamente através do diagrama de blocos mostrado abaixo.


​Com base no diagrama de blocos e nos seus conhecimentos sobre Funções, é correto afirmar que a função retorna valores classificados como:
​Lógico


05
Leia o texto abaixo:
A professora Débora propôs a criação de uma sub-rotina de função que execute cálculos segundo o parâmetro de operação fornecido. Abaixo é exibido o pseudocódigo simplificado do programa, sem as sub-rotinas de cálculo de cada operação envolvida.

```

algoritmo “CALCULADORA”

funçao calculo (R, T : real, operador : caractere) : real
var c : real
inicio
caso operador
seja “+” faça retorne c <- R + T
seja “-” faça retorne c <- R - T
seja “*” faça retorne c <-R * T
seja “/” faça retorne c <-R / T
fimcaso
fimalgoritmo

```

Com base no pseudocódigo mostrado acima e nos seus conhecimentos sobre Funções, considere as afirmativas a seguir:

I. Se o operador fornecido for “+”, faz-se a soma dos dois valores.

II. Se o operador for “-”, subtrai-se necessariamente o valor menor do maior.

III. Se o operador for “”, faz-se a multiplicação dos dois valores.

IV. Se o operador for “/”, divide-se necessariamente o valor maior pelo menor.

Está correto o que se afirma em:
​I e III


06
Leia o trecho do pseudocódigo abaixo:

funcao f2 (N:inteiro) : real

var i: inteiro

result: real

inicio

result <- 1

para I de 1 até N passo 1 faca

result <- result*i

fimpara

retorna result

fimfuncao

Com base no algoritmo mostrado acima e nos seus conhecimentos sobre Funções, é correto afirmar que a função apresenta, respectivamente:
​
Uma variável de entrada e uma de saída