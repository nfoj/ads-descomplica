Exemplo:

```

class Exemplo 
{
	public static void main(String entrada[])
	{
		int n1, n2, soma = 0;
		char op = 0;
		String msg = "", msgEnter = "Digite 1 para adição\nDigite 2 para soma";
		
		n1 = Integer.parseInt(JOptionPane.showinputDialog("Digite um numero inteiro"));

		n2 = Integer.parseInt(JOptionPane.showinputDialog("Digite um numero inteiro"));

		op = Integer.parseInt(JOptionPane.showinputDialog(msgEnter)).charAt(0);

		switch(op)
		{

			case '1';
			(
				if (n1%2==0 && n1%2==2){
					soma = n1 + n2
					msg = msg + "Soma = " + soma;
				}
				break;
			
			)

			case '2';
			(
				for(int i=1; i<=n2; i=i+1){
					soma = soma + n1
				}

				msg = msg + "Soma = " + soma;
				break;
			)
			default: JOptionPane.showMessageDialog(null, "Opção invalida!");
		}

		if (op >='1' && op <='3')
		{
			JOptionPane.showMessageDialog(null, msg);
		}
		System.exit(0);

	}	
}

```


# Problema

01
​Uma estrutura de decisão é utilizada quando apenas uma parte do programa deve ser executado de acordo com uma condição. A parte a ser executada é a que satisfaz determinada condição. Em relação à estrutura de decisão é correto afirmar que:
​Temos três tipos de estrutura de decisão, a simples, composta e encadeada


02
​Na estrutura de decisão composta, se a condição for verdadeira, os comandos são executados, caso contrário, outros comandos são executados. Qual das estruturas abaixo representa uma estrutura de decisão composta?

```

if (x > 0){

   System.out.println(x);

}

else {

   System.out.println(x+1);

}

```

03
​Utilizamos uma estrutura de repetição quando precisamos repetir por diversas vezes um mesmo conjunto de comandos. Numa estrutura de repetição é importante você garantir quando se inicia a repetição, a condição de parada e o comando de continuação na repetição. É um exemplo de uma estrutura de repetição do for:

​for (int i = 1; i<=10; i++){
​System.out.println(x);
​}


04
Utilizamos uma estrutura de repetição quando precisamos repetir por diversas vezes um mesmo conjunto de comandos. Numa estrutura de repetição é importante você garantir quando se inicia a repetição, a condição de parada e o comando de continuação na repetição. É um exemplo de uma estrutura de repetição do while:

Assinale a alternativa que contém, de cima para baixo, a sequência correta.
​int i = 1;
​while (i<=10){
​System.out.println(x);
​i++;
​}


05
​Utilizamos uma estrutura de repetição quando precisamos repetir por diversas vezes um mesmo conjunto de comandos. Numa estrutura de repetição é importante você garantir quando se inicia a repetição, a condição de parada e o comando de continuação na repetição. É um exemplo de uma estrutura de repetição do do/while:​

​int i = 1;
​do{
​System.out.println(x);
​i++;
​} while (i<=10);


06
Leia o trecho a seguir:
“Como o computador não é muito inteligente, as linguagens que entende têm uma sintaxe muito simples. Aprender uma linguagem de programação é fácil, e você rapidamente vai conseguir escrever programas bem úteis. Mas você conseguirá programar de verdade? Se surgir um problema mais complicado, você saberá resolvê-lo? Se seu programa não funcionar, você vai saber corrigi-lo? […] Programar bem exige disciplina e método.”

Fonte: RIBEIRO, J. A. Introdução à programação e aos algoritmos. 1. ed. Rio de Janeiro: LTC, 2019, p. 37.

Com base nessas informações e no conteúdo estudado acerca dos Comandos e Operadores do Java, analise os tipos de operadores abaixo e associe-os aos símbolos que pertencem a cada um deles.


```

1- Operador aritmético
2- Operador lógico
3- Operador relacional

( ) &&
( ) *=
( ) !=
( ) !
( ) %

```

Agora, assinale a alternativa que apresenta a sequência correta:
​2, 1, 3, 2, 1