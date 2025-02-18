# [ADS] [DevOps-I] Construindo nossa imagem

**Dockerfile:** é um arquivo de texto que contém instruções para construir uma imagem Docker. 

**Note:** pensa nisso como um script! 📄

**Definições:**

- **Imagem base:** qual imagem existente sua nova imagem será construída;
- **Comandos:** quais comandos serão executados dentro do container durante a construção da imagem;
- **Arquivos:** quais arquivos serão copiados para dentro da imagem;
- **Portas:** quais portas o container irá expor;
- **Usuário:** qual usuário será usado para executar os comandos dentro do container;
- **Outras configurações:** variáveis de ambiente, volumes, diretórios de trabalho, etc.

**Instruções:**

- **FROM:** define a imagem base.
    - Ex: `FROM ubuntu:latest` (começa com a imagem Ubuntu mais recente).
- **RUN:** executa um comando dentro do container durante a construção da imagem.
    - Ex: `RUN apt-get update && apt-get install -y nginx` (instala o Nginx).
- **COPY:** copia arquivos do host para dentro da imagem.
    - Ex: `COPY . /app` (copia todos os arquivos do diretório atual para o diretório `/app` dentro da imagem).
- **ADD:** similar ao COPY, mas pode descompactar arquivos e baixar arquivos remotos.
- **WORKDIR:** define o diretório de trabalho dentro do container.
    - Ex: `WORKDIR /app` (define o diretório `/app` como o diretório de trabalho).
- **EXPOSE:** expõe uma porta do container para o host.
    - Ex: `EXPOSE 80` (expõe a porta 80).
- **CMD:** define o comando que será executado quando o container for iniciado.
    - Ex: `CMD ["nginx", "-g", "daemon off;"]` (inicia o Nginx).
- **ENTRYPOINT:** semelhante ao CMD, mas com algumas diferenças em relação à forma como os comandos são executados.
- **ENV:** define variáveis de ambiente.
    - Ex: `ENV MY_VAR=my_value`
- **VOLUME:** cria um ponto de montagem para volumes.
- **USER:** define o usuário que será usado para executar os comandos.

```docker
FROM python:3.9

WORKDIR /app

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .

EXPOSE 5000

CMD ["python", "app.py"]
```

**Construir a imagem: `docker build`** 

- Ex: `docker build -t minha-imagem .` (constrói a imagem e a nomeia `minha-imagem`).

**Executar um container:** **`docker run`** 

- Ex: `docker run minha-imagem` (executa um container a partir da imagem `minha-imagem`).

**Layered system:** sistema em camadas, ou arquitetura em camadas.

**Note:** pense que o sistema de camadas no docker é cada coisinha que você adiciona ao dockerfile, cada comando é uma nova camada, logo: 

- FROM ubuntu:lastest > Camada
- RUN apt-get update > Camada
- RUN apt-get install -y nginx > Camada
- COPY meu_app /var/www/html > Camada

```docker
FROM ubuntu:latest
RUN apt-get update
RUN apt-get install -y nginx
COPY meu_app /var/www/html
```

**Logo:** 4 Camada e quanto mais camadas, mais pesado seu container fica!