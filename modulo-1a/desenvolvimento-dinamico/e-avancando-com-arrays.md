- Splice

```
var lista = [1, 2, 3, 4, 5, 6, 7];
lista.splice(2,3); // (posição, qtd-elementos)
console.log(lista);

// sem splice output
console.log(lista);
[1, 2, 3, 4, 5, 6, 7]

// splice
console.log(lista);
[1, 2, 6, 7]

```

- Splice Nomes

```
var lista = ["alice", "beto", "carlos", "dani", "elsa", "fernando"];
console.log(lista);

// outuput
['alice', 'beto', 'carlos', 'dani', 'elsa', 'fernando']

// splice
var lista-nome = lista.splice(1, 1, "Alberto") // (posição, qtd-elementos, "nome")
console.log(list-nome);
['Alberto', 'beto', 'carlos', 'dani', 'elsa', 'fernando']

```

- Slice

```

var lista = ['A', 'B', 'C', 'D' ,'E'];
var letras = lista.slice(1.3);

// Output
console.log(lista);
['A', 'B', 'C', 'D' ,'E']

// Output
console.log(letras);
['B', 'C']


```

- Concat

```

var letras-a = ['A', 'B', 'C', 'D', 'E'];
var letras-b = ['F', 'G', 'H', 'I', 'J'];

var alfabeto = letras-a.concat(letras-b);
console.log(alfabeto);

// Output
['A', 'B', 'C', 'D' ,'E', 'F', 'G', 'H', 'I', 'J']

```

- Filter

```

var num = [1,2,3,4,5,6,7,8,9,10];
var result = num.filter(item => item % 2 == 0);
console.log(result);

// Output 
[2,4,6,8,10]



// Exemplo 2
var num = [1,2,3,4,5,6,7,8,9,10];

var maior = num.filter(
	function(valor){
	return valor > 5;
	}
);

console.log(result);

```

- Map

```

var num = [1,2,3,4,5,6,7,8,9,10];

var nummap = num.map(function(valor){
	return valor = 2;
});

console.log(nummap);



// Exemplo 2

var funcionarios = [
	(nome: "Luiz", idade: 60),
	(nome: "Carlos", idade: 40),
	(nome: "Pedro", idade: 20),
	(nome: "Antonio", idade: 80),
]

nomes = funcionarios.map(pessoas => pessoas.nome);
console.log(nomes);

```


# Problemas
01
Quais das seguintes opções não é uma opção de iterar (iterar índices) em um array?
fork

02
Assinale a opção que pode ser utilizada para gerar um array, com base em array:
map

03
Qual das opções abaixo pode ser utilizada para gerar um valor único, com base em um array?
reduce

04
Qual das opções abaixo pode ser utilizada para encontrar valores, com base em uma condição específica?
filter

05
Qual dos métodos de array listados abaixo é usado para reduzir todos os valores de um array a um único valor?
reduce()

06
Analise as seguintes afirmativas sobre os métodos de manipulação de arrays em JavaScript e escolha as verdadeiras:

I. O método reduce() é utilizado para reduzir todos os valores de um array a um único valor.
II. O método map() é usado para gerar um novo array com base nos valores de um array existente.
III. O método filter() retorna um novo array contendo apenas os elementos do array original que passam em um teste fornecido.
IV. O loop for...in é recomendado como a maneira ideal de iterar sobre os valores de um array.

Apenas I, II e III são verdadeiras.
