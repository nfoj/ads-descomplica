- Modularização: é quando quebramos um problema em pedaços menores para poder facilitar a organização do código.

- modularização: dividi a tarefa em pequenos pedação.
- procedimento: cada bloco de código executa uma determinada tarefa.


```

algoritmo "Procedimento"

procedimento soma(x, y: inteiro)
var
	aux: inteiro
inicio
	aux <- x + y
	res <- aux
fimprocedimento

var
res, n, m: inteiro

Inicio
	n <- 4
	m <- -9
	soma (n, m)
	ESCREVA(res)
Fimalgoritmo


```


Exemplo:

```

algoritmo "Procedimento"

procedimento multi(x, y: inteiro)
var
	aux: inteiro
inicio
	aux <- x * y
	res <- aux
fimprocedimento

var
res, n, m: inteiro

Inicio
	n <- 1
	m <- 2
	multi (n, m)
	escreva(res)
Fimalgoritmo


```

# Problemas
01
(NUCEPE - Adaptada) Leia o texto a seguir:

“Em linhas gerais, problemas complexos exigem para sua solução algoritmos complexos. No entanto, é possível dividir um problema grande em problemas menores (dividir para conquistar), ou seja, usar o processo de modularidade.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 176.

Com base no texto acima e nos seus conhecimentos sobre Modularização, analise as afirmativas a seguir e assinale V para a(s) verdadeira(s) e F para a(s) falsa(s).

I. ( ) A modularização de algoritmos é importante porque permite organizar melhor o código, tornando mais fácil chegar à solução.

II. ( ) As variáveis locais são declaradas no escopo do programa inteiro.

III. ( ) As variáveis definidas no escopo de cada função são acessíveis em todo o programa.

IV. ( ) Um módulo é um bloco de programa que pode realizar operações computacionais de entrada, processamento e saída.

Agora, assinale a alternativa que apresenta a sequência correta:

​V, F, F, V



02
(FCC - Adaptada) Leia o texto a seguir:
“Ao dividir um problema complexo em módulos, automaticamente se usa a ideia de abstração. Abstrair um algoritmo significa considerar isoladamente um ou mais elementos de seu todo, significa, de forma geral, separar o todo em partes.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 176.

Com base no texto acima e nos seus conhecimentos sobre Modularização, considere as afirmativas a seguir:

I. Uma variável é dita global quando sua definição estiver dentro de um procedimento ou quando for declarado como parâmetro formal do procedimento.

II. Na modularização de um programa, as partes que o compõem podem ser desenvolvidas por diferentes equipes, sem necessidade de estabelecimento prévio de padrões de programação.

III. Na passagem de parâmetros por valor, as variáveis globais que estão trabalhando como parâmetros de entrada do procedimento passam seus valores para os parâmetros locais.

IV. Procedimentos são blocos de programas que executam determinada tarefa.
​III e IV



03
Leia o texto a seguir:
“O conceito de modularidade tem sido adotado desde meados da década de 1950 […] Ao trabalhar com essa técnica, pode ser necessário dividir um módulo em outros tantos módulos quantos forem necessários, buscando uma solução mais simples de uma parte do problema maior.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 176.

Com base no texto acima e nos conteúdos abordados sobre Modularização, analise as asserções a seguir e a relação proposta entre elas:

I. Procedimentos são blocos de instruções que servem para realizar tarefas específicas e são considerados sub-rotinas.

Porque

II. As sub-rotinas constituem algoritmos extremamente complexos, representando problemas considerados grandes e de difícil solução.
​
A asserção I é uma proposição verdadeira, e a II é uma proposição falsa



04
Leia o texto a seguir:
“A divisão de um módulo em outros módulos denomina-se refinamento. Tanto os módulos de procedimentos como de funções são formas de estender os recursos de abstração da técnica de programação estruturada.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 176.

Com base no texto acima e nos conteúdos abordados sobre Modularização, analise as asserções a seguir e a relação proposta entre elas:

I. Em um procedimento, a passagem de parâmetros é obrigatória.

Porque

II. Procedimentos são blocos de programas que executam determinada tarefa.
​A asserção I é uma proposição falsa, e a II é uma proposição verdadeira



05
Leia o texto a seguir:
“É importante entender como é feita a passagem de parâmetros em um programa de computador. Cada linguagem realiza esta tarefa segundo suas regras.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 122.

Com base nessas informações e no conteúdo estudado sobre Parâmetros, analise as asserções abaixo.

( ) Dividir e estruturar um algoritmo em partes logicamente coerentes
( ) Facilidade de testar os trechos em separado
( ) Evitar repetição do código-fonte
( ) Maior legibilidade de um algoritmo

Agora, assinale a alternativa que apresenta a sequência correta:
​V, V, V, V


06

Leia o texto a seguir:

“Uma sub-rotina é, na verdade, um programa, e sendo um programa, pode realizar diversas operações computacionais (entrada, processamento e saída). As sub-rotinas são utilizadas na divisão de algoritmos complexos.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 176.

Com base no texto e no conteúdo visto da disciplina, verifique se os itens abaixo são verdadeiros (V) ou falsos (F).

( ) Procedimentos são sub-rotinas.
( ) Parâmetros são sub-rotinas.
( ) Procedimentos podem receber valores (parâmetros)
( ) Procedimentos não recebem valores.

Agora, assinale a alternativa que apresenta a sequência correta de V e F:

​VFVF