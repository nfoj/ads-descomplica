# Sistemas de numeração

1. Bit e Byte

  bit = 0 (Aberto) ou 1(Fechado)
  byte = 8 x bit
    A = 10010010
    a = 11001100

  Sistema binario (0, 1) 
  Sistema decimal (0, 1, 2 .. 9)
  Sistema octal (0 .. 7)
  Sistema Hexadecimal (0 .. 9  A(10) .. F(15))

  Decimal > Binario

    25 / 2 
    Resta 1
  
      12 / 2
      Resta 0
  
        6 / 2
        Resta 0

          3 / 2
          Resta 1

            1 / 2
            Resta 1
  

  Resto = 10011 
  invertendo = 11001 | Ok!

  
Binario > Decimal


  1 1 0 0 1
  =
  4 3 2 1 0


  1
  1 = 1 x 2 ^ 4 = 16

  1
  1 = 1 x 2 ^ 3 = 8 

  0
  0 = 0 x 2 ^ 2 = 0
  
  0
  0 = 0 x 2 ^ 1 = 0
  
  1
  1 = 1 x 2 ^ 0 = 1
  
  --------------------
  = 25


# Perguntas

01 

Quanto é o número 16 na base 2 (em binário)?​

10000


02

Quanto é o número 1001 (base 2) convertido para a base 10 (decimal)?

9


03

O número 50 na base hexadecimal corresponde a qual número na base decimal?

80


04

O número 04C na base hexadecimal corresponde a qual número na base decimal?

76


05

Levando em consideração o tipo de sistema de numeração utilizado pelos computadores digitais, analise as afirmativas abaixo e identifique se a mesma é V-Verdadeira ou F-Falsa

I - O Bit é a menor unidade de informação utilizada pelo computador, tem atribuições lógicas 0 ou 1

II - O Byte é conjunto de 8 bits e pode ter até 2^8 = 256 configurações diferentes

III - No sistema de numeração binário, a notação que é utilizada possui apenas 2 algarismos ou dígitos para representar uma quantidade desejada, o 0 e o 1

É correto apenas o que se afirma em:


06

Na especificação de memória de computador, costuma-se utilizar como unidade de medida o Byte e seus múltiplos (KB, MB, GB, TB, PB, etc.)

Analise as alternativas abaixo e realize as devidas conversões. Dentre as alternativas abaixo, qual corresponde ao valor equivalente a 1,5 MB (1,5 megabyte)?

I - 1.536 KB ou 1.048.576 Bytes
II - 1.572.864 Bytes ou 1.536 KB
III - 1.572.864 KB ou 1.536Bytes
IV - 0,0014 GB ou 1.536Bytes

É correto apenas o que se afirma em:



# Pensar e Responder

Um funcionário de uma antiga biblioteca de uma pequena cidade do interior pretende digitalizar todo o acervo em papel a fim de preservar aquele valioso acervo e permitir novas formas de aproveitamento do material.

O funcionário pretende se empenhar para digitalizar por meio de reconhecimento de caracteres, não por escaneamento por imagens, que é a forma mais comum. Dessa forma os textos poderão ser estudados por busca por palavras, nomes e cruzamento de expressões, por exemplo.

No acervo, existem cerca de 1000 livros com 150 páginas cada livro, em média.

Considera-se que em uma página de um livro existam aproximadamente 200 palavras, com 7 caracteres cada palavra, também em média.

Considerando que um único BYTE armazena um único caractere (uma letra, número, ou símbolo especial), pergunta-se:

Qual a quantidade aproximada de memória (em MegaByte) o computador que esse funcionário precisará utilizar para armazenar todo o acervo?

Para responder, apresente os cálculos e as hipóteses que julgar necessárias! Além dos cálculos, apresente o resultado em MB, o resultado final deve ser informado no campo texto da resposta, mas anexe um print de uma foto ou arquivo com detalhamento do cálculo que fez.


Dados:

Livros: 1000
Paginas por Livro: 150
Palavras por pagina 200
Caracteres por palavras 7


Total de palavras por Livro

150 x 200 = 30.000

Total de Caracteres por Livro 

30.000 x 7 = 210.000

Total de Caracteres para 1000 Livros

210.000 x 1000 =  210.000.000

Cada Caractere ocupa 1 byte, logo:

Total de bytes = 210.000.000

Megabytes

1MB = 1.048.576 bytes

210.000.000 / 1.048.576 = 200,27 MB

Portanto:

Total de palavras: 30.000.000
Total de caracteres: 210.000.000
Total de memória necessária: aproximadamente 200,27 MB

Logo, para armazenar todo o acervo seriam necessarios 200,27Mb no disco!
