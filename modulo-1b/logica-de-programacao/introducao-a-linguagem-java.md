- Exemplo 01

```

int Num;
double numr;
char Caracter;

Num = Integer.parseInt(entrada[0]);
Numr = Double.parseDouble(entrada[1]);
Caracter = (entrada[2]).cahrAt(0);

soma = (double)Num + Numr;

System.out.prinln((double)Num + " + " + Numr + " = " + soma + " sinal " + Caractere);

System.exit(0);

```


- Exemplo 02

```

class program {
	public static void main (String entrada[]) {

	 // Declaração
    int n1, n2;
    String msg = "";
    double raiz, pot;

    // Verificar se há argumentos suficientes
    if (entrada.length < 2) {
	    System.out.println("Por favor, forneça dois números como entrada.");
        return;
    }

    // Input
    n1 = Integer.parseInt(entrada[0]);
    n2 = Integer.parseInt(entrada[1]);

    // Evitar divisão por zero
    if (n2 == 0) {
	    System.out.println("Erro: Divisão por zero.");
        return;
    }

    // Cálculos
    raiz = Math.sqrt(n1); // Raiz quadrada de n1
    pot = Math.pow(n1, n2); // Potência de n1 elevado a n2

    // Concatenando os resultados
    msg += "Soma: " + (n1 + n2) + "\n";
    msg += "Subtração: " + (n1 - n2) + "\n";
    msg += "Multiplicação: " + (n1 * n2) + "\n";
    msg += "Divisão: " + ((double)n1 / n2) + "\n";  // Divisão com resultado em ponto flutuante
    msg += "Raiz quadrada de " + n1 + ": " + raiz + "\n";
    msg += "Potência de " + n1 + " elevado a " + n2 + ": " + pot + "\n";

    // Exibir resultado
    System.out.println(msg);

	}
}

```


# Problema

01
​Para converter uma entrada de dados do usuário que chega no programa Java como String, para o tipo inteiro antes da realização do processamento desses dados, utiliza-se o método:
​Integer.parseInt()


02
​Para converter uma entrada de dados do usuário que chega no programa Java como String, para o tipo real antes da realização do processamento desses dados, utiliza-se o método:
​Double.parseDouble()


03
​Para converter uma entrada de dados do usuário que chega no programa Java como String, para o tipo caracter, antes da realização do processamento desses dados, utiliza-se o método:
().charAt(0)


04
Considere o seguinte programa Java.

class Ex04
{
public static void main (String entrada[])
{
int n1, n2;
double x;
n1 = Integer.parseInt(entrada[0]);
n2 = Integer.parseInt(entrada[1]);
x = n1 / n2;
System.out.println(x);
System.exit(0);
}
}

Ao executar realizar a execução deste programa no Prompt de Comando, java Ex04 1 4, o valor que será apresentado ao usuário pelo comando System.out.println(x); será:
​0.0


05
​Para ter uma interação com o usuário, para receber informações para o processamento de dados no programa Java, o comando que pode ser utilizado e que está no biblioteca javax.swing.* do Java é:
​JOptionPane.showInputDialog


06
​Para ter uma interação com o usuário, para enviar uma mensagem com o resultado do processamento de dados no programa Java, o comando que pode ser utilizado e que está no biblioteca javax.swing.* do Java é:
​JOptionPane.showMessageDialog



