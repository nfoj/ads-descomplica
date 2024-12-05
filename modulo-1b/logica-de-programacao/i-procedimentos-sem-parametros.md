- Modularização: é quando quebramos um problema em pedaços menores para poder facilitar a organização do código.

- modularização: dividi a tarefa em pequenos pedação.
- procedimento: cada bloco de código executa uma determinada tarefa.


- Procedimentos

```

	algoritmo "Procedimento"
	
	var

	procedimento MostraNaTela
	var

	inicio
		ESCREVAL("Mensagem")
	fimprocedimento

	inicio
		ESCREVAL("Mensagem")
		MostraNaTela

	fimalgoritmo

```


- Exemplo:

```

algoritmo "Procedimento"

procedimento soma
var
	aux: inteiro
inicio
	aux <- n + m
	res <- aux
fimprocedimento

var
res, n, m: inteiro

Inicio
	n <- 4
	m <- -9
	soma
	ESCREVA(res)
Fimalgoritmo

```


#  Problema

01
Leia o algoritmo abaixo:

Algoritmo “PRIMEIRA PERGUNTA”
Procedimento OPERACAOALPHA

var
aux: inteiro
inicio
aux <- a – b
res <- aux
fimprocedimento

var
res, a, b: inteiro

inicio
a <- 20
b <- 6

OPERACAOALPHA

escreval(res)

fimalgoritimo

Com base no algoritmo mostrado acima e nos conteúdos abordados sobre Procedimentos, analise as opções abaixo, e assinale a que expressa corretamente o resultado que seria impresso ao executar o procedimento OPERACAOALPHA.
14


02
Com base em Procedimentos verifique se as afirmativas são verdadeiras (V) ou falsas (F)

( ) Procedimentos não são módulos, somente Funções.
( ) Procedimento seu único objetivo é mostrar dados na tela.
( ) Procedimento é um comando.
( ) Procedimentos são considerados módulos ou sub-rotinas.

Agora, assinale a alternativa que apresenta a sequência correta:
​FFFV



03
Leia o texto a seguir:

Uma progressão aritmética (PA) é uma sequência numérica em que cada termo, a partir do segundo, é igual à soma do termo anterior com uma constante r. Podemos representar uma PA na forma PA (a1, a2, a3, … , an). Sabendo que podemos descobrir o valor do termo em qualquer posição pela fórmula:

an = a1 + (n - 1)*r

Com base no texto acima e nos conteúdos abordados sobre Procedimentos, analise as expressões de pseudocódigo abaixo e assinale aquela que corresponde à um procedimento dedicado a encontrar o valor do enésimo termo de uma PA de termos inteiros, razão r e primeiro termo igual a a1
​procedimento TERMOPA
​
Var
​Aux: inteiro
​Inicio
​Aux <- a1 + (n-1) * r
​Res <- Aux
​fimprocedimento


04
Leia o algoritmo abaixo:

Algoritmo “QUARTA PERGUNTA”
Procedimento TERMODAPA

var
aux: inteiro
inicio
aux <- a1 + (n - 1) * r
res <- aux
fimprocedimento

var
res, a1, n, r: inteiro

inicio
a1 <- 20
n <- 8
r <- 4

TERMODAPA

escreval(res)

fimalgoritimo

Com base no algoritmo acima e nos conteúdos abordados sobre Procedimentos, analise as opções abaixo, e assinale a que expressa corretamente o resultado que seria impresso ao executar o procedimento TERMODAPA.
​48


05
Leia o texto a seguir:

Um estagiário é encarregado de descobrir a idade dos funcionários de um escritório para determinar a posição de cada um na fila da vacina. A empresa guarda em seus arquivos a data de nascimento de cada um dos seus funcionários. Para facilitar seu trabalho ele decide criar o seguinte algoritmo.

Algoritmo “CALCULAR IDADE”
Procedimento IDADE

var
aux: inteiro
inicio
aux <- anoatual – anonascimento
res <- aux
fimprocedimento

var
res, anoatual, anonascimento: inteiro

inicio
anoatual <- 2021
anonascimento <- 1970

IDADE

escreval(res)

fimalgoritimo

Com base no algoritmo acima e nos conteúdos abordados sobre Procedimentos, analise as opções abaixo, e assinale a que expressa corretamente o resultado que seria impresso ao executar o procedimento IDADE para o funcionário em questão.

51



06
Leia o texto a seguir:

A velocidade média de um objeto pode ser calculada a partir da sua variação de distância percorrida e o tempo que levou para percorrer esta distância. A fórmula da velocidade média é:

Vm = variação de distância / variação de tempo

Um radar na estrada funciona calculando o tempo que um veículo levou para percorrer uma distância conhecida pelo programa. Supondo que o radar possui o seguinte pseudocódigo:

Algoritmo “CALCULAR VELOCIDADE MÉDIA”
Procedimento VELOCIDADE

var
aux, tempo: inteiro
inicio
tempo = tempofinal - tempoinicial
aux <- d / tempo
res <- aux
fimprocedimento

var
res, tempofinal, tempoinicial, d: inteiro

inicio
tempofinal <- 10
tempoinicial <- 8
d <- 60

VELOCIDADE

escreval(res)

fimalgoritimo

Com base no algoritmo acima e nos conteúdos abordados sobre Procedimentos, analise as opções abaixo, e assinale a que expressa corretamente o resultado que seria impresso ao executar o procedimento VELOCIDADE para os dados coletados pelo radar.
30