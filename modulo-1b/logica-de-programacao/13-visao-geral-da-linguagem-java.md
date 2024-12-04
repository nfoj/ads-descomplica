- Programa Java .js 
- Compilador transforando .js em .class 
- Bytecode é conjunto de instruções que são excutadas em uma máquina virtual java (JVM) 
- Tempo de execução a JVM lê e interpreta aquivos .class e executa as instruções
- JVM interpreta bytecode
- JVM é o núcleo de "gravação única, execução em qualquer local"

```

// Comentários

class MeuPrimeiroPrograma
{
	public static void main (String entrada []) {

		// declaração
		int idade = 40;
		char genero = 'F';
		double peso = 50.7;
		String nome = "Fulano";
		
		System.out.println("Eu sou " + nome + "meu peso é " + ...);
		System.exit(0);
	}
}


```


# Problemas

01
​Para desenvolver um programa Java, utilizamos um editor de texto que pode ser bloco de notas ou notepad++. Para salvar um programa na linguagem Java, precisamos utilizar qual extensão?
​.java


02
​Para compilar um programa Java pelo Prompt de Comando, se o nome do programa é Prog.java, o comando para compilar esse programa Java é:
​javac Prog.java


03
​Uma vez que o programa Java Prog.java foi compilado, um arquivo .class é gerado. Com qual nome é gerado esse arquivo .class?
​com o nome da classe .class


04
​Para executar um arquivo de extensão .class gerado pela compilação de um programa Java pelo Prompt de Comando, se o nome da classe é Prog, o comando para executar esse arquivo Prog.class gerado é:
​java Prog


05
​Para a entrada de dados em um programa Java pelo Prompt de Comando, utilizando as informações passadas pelo usuário, você deve utilizar:
​O vetor entrada[] que é um argumento passado como parâmetro no void main


06
​A saída de resultados de um programa Java pode ser apresentado no Prompt de Comandos. Para isso, o programador deve utilizar um método do Java na hora que está escrevendo o programa Java. Qual é este comando de saída de resultados ou envio de mensagens para o usuário pelo Prompt de Comando?
​System.out.println(“mensagem”);