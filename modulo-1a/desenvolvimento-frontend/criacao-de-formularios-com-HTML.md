criacao-de-formularios-com-HTML

Formulários

```
<form>
  <label for="fname">First name:</label><br>
  <input type="text" id="fname" name="fname"><br>
  <label for="lname">Last name:</label><br>
  <input type="text" id="lname" name="lname">
</form>

```

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

Texto

```
<form>  
  <label for="fname">First name:</label><br>  
  <input type="text" id="fname" name="fname"><br>  
  <label for="lname">Last name:</label><br>  
  <input type="text" id="lname" name="lname">  
</form>
```

Radio - Opção

```
<form>
  <input type="radio" id="html" name="fav_language" value="HTML">
  <label for="html">HTML</label><br>
  <input type="radio" id="css" name="fav_language" value="CSS">
  <label for="css">CSS</label><br>
  <input type="radio" id="javascript" name="fav_language" value="JavaScript">
  <label for="javascript">JavaScript</label>
</form>
```


Checkbox

```
<form>
  <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike">
  <label for="vehicle1"> I have a bike</label><br>
  <input type="checkbox" id="vehicle2" name="vehicle2" value="Car">
  <label for="vehicle2"> I have a car</label><br>
  <input type="checkbox" id="vehicle3" name="vehicle3" value="Boat">
  <label for="vehicle3"> I have a boat</label>
</form>
```


Botão de submissão

```
<form action="/action_page.php">
  <label for="fname">First name:</label><br>
  <input type="text" id="fname" name="fname" value="John"><br>
  <label for="lname">Last name:</label><br>
  <input type="text" id="lname" name="lname" value="Doe"><br><br>
  <input type="submit" value="Submit">
</form>
```


# Problemas
01
​Qual é o HTML correto para fazer uma caixa de seleção?​​
input type="checkbox"

02
Em HTML, qual atributo é usado para especificar o que deve ser obrigatoriamente preenchido em um campo?
​required​​

03
​Qual o elemento do HTML5 é usado para agrupar elementos dentro de um formulário?​
fieldset

04
​Qual atributo do form define o destino dos dados?​​
​action​​

05
Qual a função do elemento HTML form?
Iniciar a construção de um formulário, atuando como um contêiner para diferentes tipos de campos de entrada de dados.​

06
Qual a diferença entre os métodos HTTP GET e POST no envio de dados de um formulário?
POST anexa os dados do formulário à URL, enquanto GET anexa os dados do formulário dentro do corpo da solicitação HTTP.​