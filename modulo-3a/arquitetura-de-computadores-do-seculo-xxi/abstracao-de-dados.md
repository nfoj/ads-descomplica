# Abstração de dados

1. Array (vetor) or List

Posição e Elemento

x == 0

[x] [x+1] [x+2] 
[A]  [B]   [C]

 0  1  2  3  4  5  6  7  8  9
[a, b, c, d, e, f, e, g, h, i]

2. Matriz

Coordenadas P(0,0)

  P
P[1 2 3]
 [4 5 6]
 [7 8 9]

3. Tree (Árvore) ou Root(Raiz)

  1.
    1.1
    1.2
  2.
    2.1
    2.2

     a (Pai)

  b     c (Filhos)

d         e (Filhos dos Filhos)


# Perguntas

01 Marque a alternativa correta considerando as seguintes afirmações:

I – Em um vetor (array), o tamanho da lista de valores é pré-determinado e um elemento pode ser acessado diretamente
II – Em uma lista ligada, à lista de valores tem um apontador para o início da lista e um código para determinar o fim da lista
III – Inserir ou eliminar um elemento em uma lista ligada depende de ajuste de ponteiros e é um processo relativamente simples
IV – Inserir ou eliminar um elemento de um vetor é relativamente trabalhoso e exige movimentação interna dos valores na lista
V – Uma estrutura em árvore pode ser implementada por meio de vetores ou lista ligada

I, II, III, IV e V estão corretas​


02 

Quanto a diferenças entre vetor e listas ligadas, podemos afirmar:

I – Em um vetor o acesso é direto a um determinado elemento via índice
II – Em uma lista ligada o acesso a um determinado elemento é sequencial um a um, a partir do ponteiro do início da lista
III – Listas ligadas são usadas para alocação dinâmica de memória
IV – Vetores são usados quando se sabe o tamanho de memória necessário

Todas são corretas


03

Quanto a uma estrutura do tipo pilha, podemos afirmar:

Elementos podem ser removidos e inseridos apenas no topo​


04

Quanto a uma estrutura do tipo fila, podemos afirmar

Corresponde a uma estrutura do tipo FIFO (primeiro que entra é o primeiro que sai)​


05

Uma estrutura do tipo árvore possui entre seus elementos:

Raiz (início da árvore), filhos e folhas (nós sem filhos)


06

ENADE adaptada

No desenvolvimento de um software que analisa bases de DNA, representadas pelas letras A, C, G, T, utilizou-se as estruturas de dados: pilha e fila. Considere que, se uma sequência representa uma pilha, o topo é o elemento mais à esquerda; e se uma sequência representa uma fila, à sua frente é o elemento mais à esquerda. Analise o seguinte cenário:

"A sequência inicial ficou armazenada na primeira estrutura de dados na seguinte ordem: (A,G,T,C,A,G,T,T). Cada elemento foi retirado da primeira estrutura de dados e inserido na segunda estrutura de dados, e a sequência ficou armazenada na seguinte ordem: (T,T,G,A,C,T, G,A). Finalmente , cada elemento foi retirado da segunda estrutura de dados e inserido na terceira estrutura de dados e a sequência ficou armazenada na seguinte ordem: (T,T,G,A,C,T,G,A)”

Qual a única sequência de estruturas de dados apresentadas a seguir pode ter sido usada no cenário descrito acima?

Fila-Pilha-Fila​
