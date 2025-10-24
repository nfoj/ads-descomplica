# Introdução ao Bootcamp - Back-end com Python

- Introdução ao Python
  - Instalar e Hello, world!

  - Quiz:
    - Qual a extensão utilizada para criar um arquivo Python?
    - Python está disponível para quais sistemas operacionais?
    - Quais são os paradigmas que Python implementa?
    - Sobre a tipagem de Python é correto afirmar que:
    - Qual o comando utilizado para exibir a versão do interpretador Python?
    - Em Python 3, qual o comando correto para exibir a mensagem "Hello world!" na tela do usuário?


- Tipos de Dados
  - Numéricos:
    int: 1, 10, 20 ...
    float: 1.0, 10.0, 20.0 ...
    complex: 2 + 3j

  - Texto:
    str: "Texto"

  - Booleano
    true ou false

  - Estruturas de Dados
    list: lista ordenada e mutável
    tuples: lista ordenada e imutável
    dict (dicionário): chave-valor
    set (conjunto): lista não ordenada e mutável de elementos únicos

  - Especial
    NoneType: none

- Modo Interativo
  - python -i app.py

- Dica:
  dir(): atributos de um objeto
  help(): documentação

- Variáveis e constantes
  age = 40
  DEBUG = true

- Convertendo
  preco = 10.10
  preco  = int(preco)
  preco = str(preco)

- Input
  nome = input("Digite seu nome:")
  sobrenome = input("Digite seu sobrenome:")
  print(nome, sobrenome)
  print(nome, sobrenome, end="...\n")
  print(nome, sobrenome, sep="#")

- Quiz:
  - float("a"), exibe qual valor para o usuário?
  - Seguindo a convenção, qual a melhor forma de declarar a constante limite do saque em Python?
  - print(5 // 2), exibe qual valor para o usuário?
  - Qual a função usada para ler valores do teclado do usuário em Python 3?
  - Qual o retorno da função builtin dir?
  - Quais são os comandos que podemos utilizar para executar o interpretador em modo interativo?
  - Quais são as classes utilizadas para representar valores booleanos e cadeias de caracteres respectivamente?
  - Seguindo a convenção, qual a melhor forma de declarar a variável limite do saque em Python?
  - Por que usamos tipos de dados?
  - Quais são as classes utilizadas para representar números inteiros e ponto flutuante respectivamente?

- Operadores
  - (+)  print(2 + 2)
  - (-)  print(2 - 2)
  - (*)  print(2 * 2)
  - (/)  print(2 / 2)
  - (%)  print(2 % 2)
  - (//) print(2 // 2)
  - (**) print(2 ** 2)

- Precedencia
  - ()
  - **
  - *, /
  - +, -

- Comparação
  - (==)
  - (>)
  - (<)
  - (>=)
  - (<=)
  - (!=)

- Atribuição
  - (+=)
  - (-=)
  - (*=)
  - (/=)
  - (//=)
  - (**=)

- Lógicos
  - and
  - or
  - not

- Identidade
  - is
  - is not

- Associação (Esta presente em | nao esta presente)
  - in
  - in not
  - eg. =
    curso = "Curso de Python"
    "Python" in curso >>> True

    frutas = ["Laranja", "Uva", "Pera"]
    "Tangerina" in frutas >>> False

  - Quiz
  - Qual operador de comparação podemos utilizar para verificar a variável saldo é maior ou igual a variável saque?
  - No programa: saldo = 500 saldo += 300 Qual o valor de saldo?
  - O que são os operadores de associação?
  - Qual é o operador de negação para uma expressão lógica?
  - x = (22 - 10) * 3, Qual o valor de x?
  - Quais são os operadores de identidade?

- Indentação
  Indentação é obrigatória em python.

- Condicionais
  - if/else/elif

    Exemplo:
      idade = int(input("Por favor, digite a sua idade: "))

      if idade <= 17:
          print("Idade menor que 18")
      elif idade >= 60:
          print("Idade maior que 60")
      else:
          print("Idade maior que 18 e menor que 60")

- Repetição
  - For
    numeros = [1, 2, 3, 4]

    for numero in numeros:
      print(numero)

    Ex:

    for num in range (0, 51, 5):
        print(num, end=" ")


  - While
    contador = 1

    while contador <= 4:
      print(contador)
      contador += 1

- Quiz
  - Quais são as estruturas condicionais disponíveis em Python?
  - Qual o comando utilizado para pular um ciclo de iteração dos comando for e while?
  - O comando while é usado para percorrer um objeto iterável. Faz sentido usar while quando sabemos o número exato de vezes que nosso bloco
 de código deve ser executado.
  - Range é uma função built-in do Python, ela é usada para produzir uma sequência de números inteiros.
  - Quais são as estruturas de repetição disponíveis em Python?
  - Qual o comando utilizado interromper o ciclo de iteração dos comando for e while?
  - Qual a função principal da indentação em um programa Python?

- String
  - .upper
  - .lower
  - .title

  Exemplo:
    curso = "PyThoN"
    print(curso.upper())   // PYTHON
    print(curso.lower())   // python
    print(curso.title())   // Python

  - .strip
  - .lstrip
  - .rstrip

  Exemplo:
    curso = "   Python   "
    print(curso.strip())    // "P"
    print(curso.lstrip())   // "P   "
    print(curso.rstrip())   // "   P"

 - .center
 - .join

 Exemplo:
   curso = "Python"
   print(curso.center(10, "#")) // ###pyth###
   print(".".join(curso))       // p..y..t..h

 - format.
 - f()

 Exemplo:
   nome = "Alice"
   idade = 30
   mensagem = "Olá, meu nome é {} e eu tenho {} anos.".format(nome, idade)
   print(mensagem)

   cor = "azul"
   animal = "gato"
   frase = "Eu gosto de {1} e a cor favorita dele é {0}. É um {1} {0}.".format(cor, animal)
   print(frase)

   +: frase = "Eu gosto de {} e a cor favorita dele é {}.".format(cor, animal)

 - Fatiamento
   var[num]
   var[:num]
   var[num:num:num]
   var[:]
   var[::-1]

  Exemplo:
    usuarios = "Rodolfo e Alice"
    usuarios[0]    // 'R'
    usuarios[:]    // 'Rodolfo e Alice'
    usuarios[::-1] // 'ecilA e oflodoR'

- String tripla
  """ """
  ''' '''

  Exemplo:
    mensagem = """
      O tempo é muito lento para os que esperam
        Muito rápido para os que têm medo
      Muito longo para os que lamentam
        Muito curto para os que festejam
      Mas, para os que amam, o tempo é eterno.
    """
    print(mensagem)

  - Quiz
    - Strings de múltiplas linhas são definidas informando 3 aspas simples ou duplas durante a atribuição. Elas podem ocupar várias linhas d
o código, e todos os espaços em branco são incluídos na string final
    - No programa: curso = "Python" print(f"Bem vindo ao curso de {curso.upper()}!") Qual será a mensagem exibida para o usuário?
    - texto = " Python ".lstrip(). Qual o valor de texto?
    - No programa: curso = "Python" print(curso[::-1]) Qual será a mensagem exibida para o usuário?
    - Qual método da classe string converte todas os caracteres para maiúsculo?
    - Utilizando fatiamento de sequências, como podemos acessar o primeiro caractere da variável curso = "Python"?
    - No programa: PI = 3.14159 print(f"Valor de PI: {PI:.2f}") Qual será a mensagem exibida para o usuário?

- Listas
  - Criando:
    frutas = ["Abacaxi", "Caju", "Manga"]
    print(frutas[0]) # Abacaxi
    print(frutas[1]) # Caju

  - Adicionando elementos
    - append

      frutas = ["Abacaxi", "Caju"]
      frutas.append("Banana")
      print(frutas)  # ["Abacaxi", "Caju", "Banana"]

    - insert

      frutas.insert(1, "Maçã")
      print(frutas)  # ["Abacaxi", "Maçã", "Caju", "Banana"]


 - Removendo elementos
   - pop

     frutas = ["Abacaxi", "Caju", "Manga"]
     frutas.remove("Caju")
     print(frutas)  # ["Abacaxi", "Manga"]

   - remove

     frutas.pop(0)
     print(frutas)  # ["Manga"]


- Outros
  - len

    frutas = ["Abacaxi", "Caju", "Manga"]
    print(len(frustas)) # 3

  - slice

    print(frutas[0:2])  # ["Abacaxi", "Caju"]

  - pertence

    print("Caju" in frutas)  # True

  - operacoes

    numeros = [10, 20, 30, 40]
    print(sum(numeros))  # 100
    print(max(numeros))  # 40

  - sort

    frutas = ["Manga", "Abacaxi", "Caju"]
    frutas.sort()
    print(frutas)  # ["Abacaxi", "Caju", "Manga"]

  - inverter

    frutas.reverse()
    print(frutas)  # ["Manga", "Caju", "Abacaxi"]


- Lista dentro de lista

  matriz = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
  ]


- Quiz
  - Como podemos recuperar o primeiro elemento da lista: frutas = ["maçã", "laranja", "uva", "pera"] ?
  - Como podemos recuperar o último elemento da lista: frutas = ["maçã", "laranja", "uva", "pera"]?
  - Qual será o valor da variável números ao executar o código abaixo: [n**2 if n > 6 else n for n in range(10) if n % 2 == 0] ?
  - Listas em Python podem armazenar de maneira sequencial qualquer tipo de objeto?



- Tuplas - Tuplas são Imutáveis

  frutas = ("Abacaxi", "Caju", "Manga")
  print(frutas[0])   # Abacaxi
  print(frutas[1])   # Caju
  print(frutas[-1])  # Manga (último)

  pessoa = ("João", 25, "Brasil")
  nome, idade, pais = pessoa
  print(nome)   # João
  print(idade)  # 25

  lista = [1, 2, 3]
  tupla = tuple(lista)
  print(tupla)  # (1, 2, 3)

- Quiz
  - Qual será o retorno do código a seguir: carros = ("gol") print(isinstance(carros, tuple))
  - Tuplas podem armazenar somente objetos imutáveis.
  - Qual a principal diferença entre tuplas e listas?
  - Considere a tupla: carros = ("gol", "celta", "palio",). Como podemos alterar o valor do primeiro elemento?

- Dicionarios
  pessoa = {
    "nome": "João",
    "idade": 25,
    "cidade": "São Paulo"
  }

  print(pessoa.get("nome"))      # João
  print(pessoa.get("profissao")) # None
  print(pessoa.get("profissao", "Não informado"))  # Não informado

  pessoa = {"nome": "Maria", "idade": 30}

  - Adicionar nova chave

  pessoa["profissao"] = "Engenheira"
  print(pessoa)  # {"nome": "Maria", "idade": 30, "profissao": "Engenheira"}


  pessoa = {"nome": "Pedro", "idade": 28, "cidade": "Rio"}

  - Remover chave específica

  del pessoa["cidade"]
  print(pessoa)  # {"nome": "Pedro", "idade": 28}

  pessoa = {"nome": "Ana", "idade": 27, "cidade": "Brasília"}

  - Percorrer chaves
  for chave in pessoa:
      print(chave)

  - Percorrer valores

  for valor in pessoa.values():
      print(valor)

  - Percorrer chaves e valores

  for chave, valor in pessoa.items():
      print(f"{chave}: {valor}")

- Quiz
  - Considere o dicionário: carro = {"marca": "Fiat", "modelo": "palio", "placa": "ABD-9826"}. Qual será o retorno do código: carro.get("mot
or")?
  - Os dados de um dicionários são acessados e modificados através de índices.
  - Um dicionário em Python é definido utilizando chaves { }, contendo pares de chave e valor separados por dois pontos (:).
  - Dado o dicionário: contatos = {"idioma": "pt_br", "pais": "Brasil" }, como podemos recuperar o país do contato?
