# Programando com JavaScript: primeiros comandos 

1. Tipos de dados na prática
  - String (Texto)
  - Number (Numeros)
  - Char (Caractere)
  - Bool (Boleanos (True ou Falso))

  ```
  let texto = "String";
  console.log(texto);

  let numero = 1;
  console.log(numero);

  let caractere = 'A';
  console.log(caractere);

  let verdadeiro = true;
  console.log(verdadeiro);

  ```

2. Contatenar

  ```

  // + 

  let nome = "João";
  let sobrenome = "Silva";
  let nomeCompleto = nome + " " + sobrenome; // Resultado: "João Silva"
  console.log(nomeCompleto);


  // Concat
  let cidade = "João Pessoa";
  let estado = "Paraíba";
  let localizacao = cidade.concat(" - ", estado); // Resultado: "João Pessoa - Paraíba"
  console.log(localizacao);


  // ${}
  let nome = "Roberto";
  let sobrenome = "Porco";
  let idade = 30;
  let cidade = "Pato Branco";
  let mensagem = `Olá, meu nome é ${nome} ${sobrenome}, tenho ${idade} anos e moro em ${cidade}.`;
  console.log(mensagem);

  
  ```

3. Operadores Aritméticos 
  
  +: Adição
  -: Subtração
  *: Multiplicação
  /: Divisão
  %: Módulo (resto da divisão)
  ++: Incremento (adiciona 1)
  --: Decremento (subtrai 1)


4. Operadores de Atribuição

  =: Atribuição simples
  +=: Atribuição de adição (ex: x += 5 é equivalente a x = x + 5)
  -=: Atribuição de subtração
  *=: Atribuição de multiplicação
  /=: Atribuição de divisão
  %=: Atribuição de módulo


5. Operadores de Comparação

  ==: Igualdade (comparação de valor)
  ===: Igualdade estrita (comparação de valor e tipo)
  !=: Desigualdade (comparação de valor)
  !==: Desigualdade estrita (comparação de valor e tipo)
  >: Maior que
  <: Menor que
  >=: Maior ou igual a
  <=: Menor ou igual a


6. Operadores Lógicos

  &&: E lógico (ambas expressões devem ser verdadeiras)
  ||: Ou lógico (pelo menos uma expressão deve ser verdadeira)
  !: Negação (inverte o valor booleano)


7. Operadores Bitwise

  &: AND bit a bit
  |: OR bit a bit
  ^: XOR bit a bit
  ~: NOT bit a bit
  <<: Deslocamento bit a bit para a esquerda
  >>: Deslocamento bit a bit para a direita
  >>>: Deslocamento bit a bit para a direita com preenchimento de zero


8. Operador Ternário

  condição ? expressão1 : expressão2


9. Operador de Acesso a Propriedades

  .: Acesso a propriedade
  []: Acesso a propriedade usando uma string 


# Exemplo:
    
  ```
  let x = 10;
  let y = 5;

  // Operadores aritméticos
  console.log(x + y); // 15
  console.log(x - y); // 5
  console.log(x * y); // 50
  console.log(x / y); // 2
  console.log(x % y); // 0

  // Operadores de atribuição
  x += 3; // x = x + 3
  console.log(x); // 13

  // Operadores de comparação
  console.log(x > y); // true
  console.log(x === y); // false

  // Operadores lógicos
  console.log(x > 0 && y > 0); // true
  console.log(x > 15 || y > 5); // false

  // Operador ternário
  let resultado = x > 10 ? "Maior que 10" : "Menor ou igual a 10";
  console.log(resultado); // Menor ou igual a que 10

  
```
