Exemplo:

```

algoritmo "vetor"


Var
	nomes: vetor [1..5] de caractere
	contador: inteiro

Inicio
	PARA contador de 1 ATE 5 FACA
		ESCREVAL("Digite o nome  ", contador, " de 5: ")
		LEIA(nome[contador])
	FIMPARA

	PARA contador de 1 ATE 5  FACA
		ESCREVAL("Nome do aluno ", nomes[contador])
	FIMPARA

Fimalgoritmo

```


Exemplo:

```
algoritmo "NotasNomes"

var
	notas: vetor [1..5] de real
	nomes: vetor [1..5] de caractere
	i: inteiro
	mdia: real
	soma: real

inicio

	soma <- 0
	PARA i de 1 ATE 5 FACA
		ESCREVAL("Digite o nome do aluno(a) ", i, " de 5: ")
		LEIA(nomes[i])
		ESCREVAL("Digite o nota do aluno(a) ", i, " de 5: ")
		LEIA(notas[i])
		soma <- soma + notas[i]
	FIMPARA
	media <- soma/i

	ESCREVAL("Media de Notas da Turnma: ", media)

Fimalgoritmo

```


01
(FCC - Adaptado) Leia o texto abaixo:

Um Analista de TI precisou usar uma estrutura de dados simples que utilizasse pouca carga de memória de armazenamento. Tal estrutura é vista como um arranjo cuja capacidade pode variar dinamicamente, isto é, se o espaço reservado for totalmente ocupado e algum espaço adicional for necessário, este será alocado automaticamente não havendo a necessidade de se preocupar com a capacidade de armazenamento ou sua ocupação. Contudo, para que se possa utilizar essa coleção de dados de forma adequada, algumas informações necessárias devem ser mantidas internamente, tais como a quantidade total de elementos e a última posição ocupada na coleção, conforme exemplificado na figura abaixo.


​Com base no texto, na figura e nos seus conhecimentos acerca da disciplina Criação de Aplicações e Sistema, trata-se da estrutura do tipo:
Homogênea unidimensional, portanto um vetor


02
(Quadrix/ AOCP - Adaptada) Leia o trecho a seguir:
“A matriz de uma dimensão (vetor) é a forma mais simples de tabela de valores com apenas uma coluna e várias linhas de dados, definida em uma única variável com tamanho específico.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 107.

A partir do texto acima e dos conhecimentos sobre Vetor/Array, considere a afirmativas a seguir:

I. O tamanho dos vetores são declarados, geralmente, por meio de colchetes, os quais são usados também para limitar o número de elementos que podem ser armazenados.

II. A atribuição de valores a um vetor já criado é procedida de elemento em elemento, alterando‐se o valor do índice do vetor.

III. Para manipular um elemento em um vetor, de uma estrutura simples (unidimensional) de dados de mesmo tipo, não há a necessidade de se fornecer a posição do elemento desejado.

IV. Um vetor é declarado definindo‐se o seu nome, seu tamanho e seu tipo.

V. Um conjunto homogêneo de dados se dá quando uma determinada estrutura de dados é composta de variáveis com o mesmo tipo.

Está correto o que se afirma em:
I, II, IV e V


03
​(FCC - Adaptada) Considere o vetor vet a seguir:​

Agora analise a seguinte sequência de comandos de atribuição que foi feita por um profissional de programação:

aux ← vet[8]

vet[8] ←vet [1]

vet[4] ← vet[6]

vet[6] ← vet[3]

vet[3] ← vet[1] ← aux

A configuração do vetor (do índice 1 ao 8) será:
​AMAZONAS


04
(IDECAN - Adaptada) Leia o trecho a seguir:

Arrays são consideradas estruturas de dados que consistem em itens de dados do mesmo tipo. São entidades “estáticas” porque uma vez que são criadas, permanecem do mesmo tamanho. É um grupo de posições de memória adjacentes, que possuem o mesmo nome e tipo.

Com base no texto acima e nos seus conhecimentos sobre Vetor/Array, analise as afirmativas a seguir e assinale V para a(s) verdadeira(s) e F para a(s) falsa(s).

I. ( ) O vetor/array é declarado em uma seção de variáveis.
II. ( ) A palavra vetor (no Visualg) não é uma palavra reservada.
III. ( ) Um vetor com valor inicial e valor final de um tipo (separados por dois pontos), pode ser do tipo inteiro, real, ou caracter.
IV. ( ) O número da posição em colchetes é mais formalmente chamado de índice. Esse número deve ser um inteiro.

Agora, assinale a alternativa que apresenta a sequência correta:
​V, F, V, V


05
(CESPE/ CEBRASPE - Adaptada) Leia o trecho a seguir:
“[…] o vetor ocupa uma área de memória contígua, portanto, o segundo elemento está na memória logo após o primeiro elemento; o terceiro, após o segundo; e assim até que o último elemento esteja após o penúltimo. Um vetor de n elementos inteiros ocupa a memória equivalente a pelo menos n inteiros.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 136.

A partir do texto acima e dos conhecimentos sobre Vetor/Array, considere a afirmativas a seguir:

I. Vetores são utilizados quando estruturas indexadas necessitam de mais que um índice para identificar um de seus elementos.

II. Vetores podem ser considerados como listas de informações armazenadas em posição contígua na memória.

III. Uma posição específica de um vetor pode ser acessada diretamente por meio de seu índice.

Está correto o que se afirma em
​II e III


06
(IBFC - Adaptada) Leia o trecho a seguir:

Como resposta a uma questão da prova, a definição de estrutura de dados elementar denominada vetor foi apresentada por 5 alunos de formas diferentes, como mostrado a seguir:

Tatiana definiu que: o vetor é a representação gráfica de uma reta orientada possuindo mesma intensidade, direção e sentido.

Rivaldo escreveu que: o vetor é o conjunto de todos os segmentos orientados equipolentes a um segmento dado.

Daniele definiu que: o vetor permite o acesso a uma grande quantidade de dados em memória usando-se apenas um nome de variável.

Fernando escreveu que: os vetores são utilizados em computação gráfica representando os minúsculos pontos diferenciados pela sua cor.

Ana definiu que: o vetor é uma representação mais simples de uma estrutura de repetição, de modo que toda matriz é um vetor, mas nem todo vetor é uma matriz.

A professora Débora corrigiu todas as respostas dadas pelos alunos e chegou a conclusão de que apenas uma delas estava correta, a resposta de:
​​​Daniele