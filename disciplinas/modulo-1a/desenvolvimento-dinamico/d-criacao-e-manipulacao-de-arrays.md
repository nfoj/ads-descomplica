- Array: uma lista de alguma coisa

```

var nome = [item1, item2, ..., itemn];

var num = [1, 2, 3, 4, 5, ... , 10];

```

- Tamanho

```

var arr = [42]; Lista com item 42

var arr = Array (42); Lista om 42 posições

```

- Posições

```

var arr = [0]; posicao [0] [1] [2]


```

- Exemplo:

```

var valores = ['A', 'B', 'C', 'D' ,'E']

console.log(valores[0]); // A
console.log(valores[1]); // B
console.log(valores[2]); // C
console.log(valores[3]); // D
console.log(valores[4]); // E

for (var pos = 0; pos < valores.length; pos++) {
	console.log("Posição: " + pos + " Valor " + valores[pos]);
}

```

- Exemplo:

```

var comida = [];
comidas [0] = "Salada";
comidas [1] = "Arroz";

```

- Métodos

- join = juntas array
- shift = tira repetidos
- push = adicionar elementos no array
- pop = retorna o último alemento ...

```

var valores = ['A', 'B', 'C', 'D' ,'E']
Console.log(valores.reverse());

// Output
[A, B, C, D, E]

-----------------------------------------------

var valores = [1, 2, 3, 4, 5, 6]
Console.log(valores.join(' ')); // Vira string

// Output
1 2 3 4 5 6

-----------------------------------------------

var valores = ['A', 'B', 'C', 'D' ,'E']
Console.log(valores.shift());

// Output
[B, C, D, E]

-----------------------------------------------

var valores = [1, 2, 3, 4, 5, 6]
Console.log(valores.indexOf(3)); // Procura o valor 3

// Output
3: 2

// Exemplo
var valores = ['A', 'B', 'C', 'D' ,'E']
Console.log(valores.indexOf(3)); // Procura o valor B

// Output
B: 1

-----------------------------------------------

var valores = ['A', 'B', 'C', 'D' ,'E']
valores.push('F');
Console.log(valores[]);

// Output
[A, B, C, D, E, F]

-----------------------------------------------

var valores = ['A', 'B', 'C', 'D' ,'E']
valores.pop();
Console.log(valores[]);

// Output
[A, B, C, D, E]

```


# Problemas

01
Sobre o objeto array, assinale a opção correta:
uma matriz é uma coleção de variáveis do mesmo tipo.

02
Assinale o item correto sobre as propriedades de matriz:
length reflete o número de elementos em uma matriz.

03
Qual dos métodos de array listados abaixo Remove o último elemento de uma matriz e retorna esse elemento? Assinale o item correto:
pop()​

04
Qual dos métodos de array listado abaixo adiciona um ou mais elementos ao final de uma matriz e retorna o novo comprimento da matriz? Assinale o item correto:
push()

05
Qual método de array listado abaixo é utilizado para adicionar um elemento ao início de uma matriz?
unshift()

06
Analise as seguintes afirmativas sobre as operações básicas em matrizes em JavaScript:

I. O método push() é utilizado para adicionar um elemento ao final de uma matriz.
II. O método unshift() é utilizado para adicionar um elemento ao início de uma matriz.
III. O método pop() é utilizado para remover um elemento do final de uma matriz.
IV. O método shift() é utilizado para remover um elemento do início de uma matriz.

Escolha a opção correta:
As afirmativas I, II, III e IV estão corretas