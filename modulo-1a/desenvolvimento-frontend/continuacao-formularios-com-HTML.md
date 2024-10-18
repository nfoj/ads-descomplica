continuacao-formularios-com-HTML

Types

```
<input type="button">
<input type="checkbox">
<input type="color">
<input type="date">
<input type="datetime-local">
<input type="email">
<input type="file">
<input type="hidden">
<input type="image">
<input type="month">
<input type="number">
<input type="password">
<input type="radio">
<input type="range">
<input type="reset">
<input type="search">
<input type="submit">
<input type="tel">
<input type="text">
<input type="time">
<input type="url">
<input type="week">
```

Select 

```
<legend>Texto sobre seu formulário</legend>
<select>
	<option value="1x">1x sem juros</option>
	<option value="2x" selected>2x sem juros</option>
	<option value="3x">3x sem juros</option>
</select>
```

# Problema
01
Quais são as características de uma caixa de seleção ('checkbox') em um formulário HTML?
​Permite várias opções selecionadas

02
O que é um 'fieldset' em HTML?
​É um elemento que permite agrupar vários campos de entrada

03
Qual é a principal diferença entre um botão de rádio e uma caixa de seleção?
​Botões de rádio permitem apenas uma seleção, enquanto caixas de seleção permitem várias

04
Em um botão de rádio HTML, o que o 'value' representa?
​O valor que será enviado ao servidor se o botão for selecionado

05
Qual é a finalidade de uma lista suspensa em um formulário HTML?
​Exibir uma lista de opções das quais o usuário pode escolher uma

06
Imagine que você é um desenvolvedor web trabalhando para uma agência de viagens. Sua tarefa é criar um formulário para coletar informações sobre o tipo de serviço que um cliente deseja, a forma de pagamento escolhida e o prazo de pagamento. Este formulário deve ser facilmente navegável e deve fornecer ao usuário um conjunto claro de opções para selecionar.

Levando em consideração o trecho. Já temos grupos de campos para "Dados da Viagem", "Quais serviços deseja contratar?" e "Escolha a forma de pagamento". Agora, vamos adicionar outro grupo de campos onde o cliente poderá selecionar o “prazo de pagamento”, que tipo de campo de entrada do HTML5 seria mais apropriado para permitir ao cliente escolher o prazo de pagamento entre várias opções, garantindo que apenas uma opção possa ser selecionada?

​Lista suspensa ('select')


# Prática
Atividade Prática 6: Continuação: formulários com HTML
Objetivo: O objetivo desta atividade é permitir que os alunos apliquem os conceitos aprendidos na aula “Continuação: formulários com HTML”, criando um formulário de reserva de viagem mais avançado que incorpora diferentes tipos de campos de entrada, incluindo caixas de seleção, botões de rádio e listas suspensas.

```
<!DOCTYPE html>

<html lang="en">

<head>

    <meta charset="UTF-8">

    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <title>Reserva.html</title>

</head>

<body>

    <h1>Formulário de Reserva de Viagem</h1>

   

    <section>

        <form action="" autocomplete="on" method="get">    

            <fieldset>

            <legend>Informações da Viagem</legend>

                <label>Nome</label><br>

                <input type="text" name="nome" required><br><br>

           

                <label>Email</label><br>

                <input type="email" name="email" required> <br><br>



                <label>Date de Ida</label><br>

                <input type="date" name="dataida"> <br><br>



                <label>Data de Volta</label><br>

                <input type="date" name="datavolta">

            </fieldset>

           

            <fieldset>

            <legend>Serviços Desejados</legend>

                <select>

                    <option value="compra passagem">Compra de Passagem</option>

                    <option value="reserva hospedagem">Reserva de Hospedagem</option>

                    <option value="aluguel veículos">Aluguel de Veículos</option>

                </select>

            </fieldset>



            <fieldset>

            <legend>Forma de Pagamento</legend>

                <label>

                    <input type="radio" name="formaPagamento" value="cartaocredito">Cartão de Crédito

                </label>

                <br>

           

                <label>

                    <input type="radio" name="formaPagamento" value="boletobancario">Boleto Bancário

                </label>

                <br>



                <label>

                    <input type="radio" name="formaPagamento" value="pix">Pix

                </label>

            </fieldset>



            <fieldset>

            <legend>Prazo de Pagamento</legend>

                <select name="prazosdepagamento">

                    <option value="avista">À Vista</option>

                    <option value="cincovezes">5x sem juros</option>

                    <option value="dezvezes">10x com juros</option>

                </select>

            </fieldset>



            <fieldset>

            <legend>Restições e Necessidades</legend>

            <textarea rows="3"></textarea>

            </fieldset>

           

            <br>

            <input type="submit" value="Enviar">

        </form>

    </section>



</body>

</html>

```