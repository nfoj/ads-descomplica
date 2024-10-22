Link - HTML

```
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="styles.css">
</head>
<body>

<h1>This is a heading</h1>
<p>This is a paragraph.</p>

</body>
</html>

```


Estilo

```

body {
  background-color: powderblue;
}

h1 {
  color: blue;
}

p {
  color: red;
}

```


Syntax

h1 > Seletor
color > propriedade
blue > valor

h1 {
	color: blue;
}


```

// Seletor universal *

* {
  text-align: center;
  color: blue;
}

```

```

# Id

<p id="exemplo">Texto</p>

#exemplo {
  text-align: center;
  color: red;
}

```

...

# Problemas

01
​Qual é a sintaxe CSS correta para deixar todos os elementos p em negrito?
p {font-weight:bold;}

02
Qual destes códigos CSS exibe hiperlinks sem sublinhado?
a {text-decoration:none;}

03
Como você seleciona um elemento com id 'demo' em um arquivo .css?
#demo

04
Como você seleciona elementos com o nome de classe 'test'?
.test

05
Dado o código CSS para estilizar o título da página:

p {
 text-align: center;
 font-style: italic;
 font-size: 44px;
 text-shadow: 3px 3px 3px #0f0f0f;
 color: white;
 font-weight: bold;
}

Analise as seguintes afirmações:

I. O título da página recebeu as seguintes formatações: alinhamento centralizado, estilo itálico, tamanho da fonte de 44px, sombreamento, cor branca e texto em negrito.

II. A tabela recebeu estilos diferentes para as linhas pares e ímpares, com as pares recebendo a cor de fundo #e9e9e9 e as ímpares recebendo a cor de fundo #bdbdbd.

III. O cabeçalho da tabela recebeu a cor de fundo cinza e cor de texto branco.

Assinale a alternativa correta:
Apenas a afirmativa I está correta.​


06
Considerando o seguinte código CSS para estilizar uma lista não ordenada:

ul {
    list-style: none;
    padding: 25px 50px 70px 0px;
    background: gray;
}
 
ul li {
    float: left;
    padding: 10px;
    color: white;                
}

Analise as seguintes afirmações:

I. Os marcadores padrão da lista foram removidos.
II. Foi adicionado um espaçamento entre os itens da lista
III. A lista foi disposta na horizontal.
IV. A cor de fundo aplicada à lista foi cinza e a cor do texto foi branca.

Assinale a alternativa correta:
Todas as afirmativas estão corretas.​