# [ADS] [DevOps-I] Acessando "de dentro" e "de fora”

**host:** sistema operacional que segura todos os containers que ficam em cima dele e é reponsável por distribuir recursos da máquina local como CPU, memória. 

Temos que entender que acessar em container é uma palavra vaga, você tem que especificar sobre o que está falando acessar a porta? Acessar o sistema operacional? 

Caso queira acessar uma porta, precisa antes passar a porta que nem vimos:

```powershell
# comandos + caminho
# -v /C/docker/volumes/nginx: = caminho do volume
# -p 9180:80 = define a porta
# nginx:lasted = usa a versão lasted a última
# -d = roda em background

docker run --name Nginx2 -v /C/docker/volumes/nginx:/usr/share/nginx/html -p 9180:80 -d nginx: lasted
```

Agora se desejamos acessa o sistema o operacional que definimos no dockerfile, seguimos assim:

```powershell
# comandos + caminho
# -v /C/docker/volumes/nginx: = caminho do volume
# -p 9180:80 = define a porta
# nginx:lasted = usa a versão lasted a última
# -d = roda em background

docker run --name Nginx2 -v /C/docker/volumes/nginx:/usr/share/nginx/html -p 9180:80 -d nginx: lasted
docker exec -it Ngix /bin/bash

# exec = executar
# -it = iteração
# Ngix = com o que vai interagir
# /bin/bash = terminal linux 

```

**Portas:**  

- **Físicas:** são as postas presentes na sua máquina para criar uma conexão, Exemplo: rede, p2 ..
- **Virtuais:** são aquelas que usamos para criar uma conexão de forma virtual. Exemplo: acessar o google.

**Portas em Docker:**  containers são isolados do host e de outros containers, significa que, por padrão, um container não pode ser acessado diretamente de fora. 

As portas virtuais são uma maneira de expor as portas que um container está usando para o mundo externo, permitindo que outros serviços ou usuários acessem a aplicação que está rodando dentro do container.

```docker
docker run -p 80:8080 minha_imagem
```