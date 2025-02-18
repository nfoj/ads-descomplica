Exemplo:

```

Algoritmo "Repeticao"


Var
	i, j:inteiro


Inicio
	PARA i de 1 ATE 10 FACA
		ESCREVAL ("Valor de I: ", i)
		PARA j de 1 ATE 3 FACA
			ESCREVAL ("Valor de J: ", j)
		FIMPARA
	FIMPARA	

Fimalgoritmo

```


Exemplo:

```

Algoritmo "Enquanto"

Var
	i:inteiro

Inicio
	i<- 1
	i<= 10 FACA
		ESCREVAL ("Valor de I: ", i)
		i<- i+1
	FIMENQUANTO

Fimalgoritmo

```

Exemplo:

```

Algoritmo "Repita"

Var
	i:inteiro

Inicio
	i<- 1
	REPITA
		ESCREVAL ("Valor de I: ", i)
		i<- i+1
	ATE i>=10

Fimalgoritmo

```


# Problema

01
Leia o trecho a seguir:
“Muitas vezes é necessário repetir um trecho de programa determinado número de vezes. Neste caso, pode ser utilizada a técnica de laço de repetição (looping ou malhas), que tem por finalidade efetuar o processamento de um trecho de instruções tantas vezes quantas forem necessárias.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 89.

A partir do texto acima e dos conhecimentos sobre Repetição, analise o pseudocódigo a seguir:


​Assinale a alternativa que contém o número de vezes que será exibida a frase: Oi, tudo bem?
4


02
Leia o trecho a seguir:
“Os laços de repetição podem ser classificados em duas formas, sendo laços de repetição interativa ou laços de repetição iterativa. São interativos quando necessitam da intervenção de um usuário para repetir a próxima ação do programa um indeterminado número de vezes, são laços iterativos quando executam as repetições previstas de forma automática determinado número de vezes.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 89

Com base no texto acima e nos seus conhecimentos sobre Repetição, analise as afirmativas a seguir e assinale V para a(s) verdadeira(s) e F para a(s) falsa(s)

I. ( ) Um loop ou laço infinito é aquele que apresenta sempre uma condição de teste verdadeira, ou seja , nunca termina.

II. ( ) A instrução que realiza o teste lógico no início da repetição é enquanto.

III. ( ) Em algoritmos, precisamos executar alguns passos mais de uma vez. Ou mesmo executar repetidamente alguns passos até que alguma condição seja atendida. A partir dessa necessidade surgem as estruturas de repetição, também conhecidas como CONDICIONAIS.

IV. ( ) A instrução que realiza o teste lógico no fim da repetição é repita.

V. ( ) A estrutura que implementa a variável de controle e efetua o teste lógico é para.

Agora, assinale a alternativa que apresenta a sequência correta:
​V, V, F, V, V


03
Leia o trecho a seguir:
“Existem comandos apropriados para realizar a repetição de determinados trechos de programa o número de vezes que for necessário. A vantagem desse recurso é que o programa passa a ser menor, podendo sua amplitude de processamento ser aumentada sem alterar o tamanho do código de programação. É possível determinar repetições com números variados de vezes.”

Fonte: MANZANO, J. A. N. G.; OLIVEIRA, J. F. Estudo Dirigido de Algoritmos. 15. ed. São Paulo: Érica, 2012, p. 89

Com base no texto acima e nos seus conhecimentos sobre Repetição, considere a afirmativas a seguir:

I - O comando de repetição é utilizado da seguinte forma: 
REPITA <sequência-de-comandos>
ATE <expressão-lógica>

II - A estrutura da instrução enquanto...faça...fimenquanto é uma estrutura condicional, e executa um conjunto de instruções enquanto a condição verificada for Falsa

III - O comando para é utilizado da seguinte forma: comandos PARA "variável" DE "valor inicial" ATE "valor final" FACA "sequência-de-comandos" FIMPARA

IV - Os laços que possuem um número finito de execuções podem ser processados pela estrutura para, por meio dos comandos para...de...até... passo...faça...fim_para.

V - A estrutura repita...até_que tem o seu funcionamento controlado por decisão, executando um conjunto de instruções pelo menos uma vez antes de verificar a validade da condição estabelecida.

Está correto o que se afirma em:
​I, III, IV e V


04
(EsFCEx)Leia o trecho a seguir:
Uma estrutura de repetição é uma estrutura de desvio do fluxo de controle presente em linguagens de programação que realiza e repete diferentes ações, dependendo se uma condição for verdadeira ou falsa, em que a expressão é processada e transformada em um valor booleano.

Com base no texto acima e nos seus conhecimentos sobre Repetição, analise as afirmativas a seguir:

I. Em uma estrutura de repetição do tipo PARA, o controle do laço é feito pelo uso de uma variável lógica, que é iniciada como VERDADEIRA, encerrando o laço ao ter seu valor modificado para FALSO.

II. Na estrutura de repetição do tipo ENQUANTO, o teste do controle é realizado por um teste lógico, no início do laço, e se esse teste for FALSO, logo na primeira execução, o laço não será executado nenhuma vez.

III. Apenas as estruturas de repetição do tipo PARA e REPITA possuem controle do laço por meio de variáveis lógicas.

IV. Na estrutura de repetição do tipo REPITA, o laço é executado pelo menos uma vez, pois o controle é realizado no final do laço apenas.

Está correto o que se afirma em:
​II e IV


05
(Aeronáutica - Adaptado)
“Os computadores executam muito bem as tarefas repetitivas. Com frequência, temos de executar uma ação enquanto alguma condição seja verdadeira. Assim, por exemplo, quando procuramos um nome em uma lista, devemos ler nomes enquanto o nome lido for diferente daquele que procuramos. O mecanismo em algoritmos para isso é o bloco ‘Enquanto condição faça comandos’.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 92.

A partir do texto acima e dos conhecimentos sobre Repetição, analise o diagrama a seguir:

​Assinale a alternativa que exibe o trecho de programa em português estruturado correspondente ao diagrama de blocos acima
​enquanto (< C1 >) faça
​enquanto ( < C2 > ) faça
​<instruções>
​fim_enquanto
​fim_enquanto


06
Leia o texto a seguir:

Um estudante estava desenvolvendo um algoritmo para somar valores até o usuário digitar o valor 0, utilizando a ferramenta VisuAlg. O algoritmo é mostrado abaixo:

​Assinale a alternativa que explique corretamente o funcionamento do algoritmo:​

Ele somará todos os valores que o usuário digitar, porém quando ele digitar 0 o “loop” acaba. A cada loop são apresentados os resultados atuais da soma