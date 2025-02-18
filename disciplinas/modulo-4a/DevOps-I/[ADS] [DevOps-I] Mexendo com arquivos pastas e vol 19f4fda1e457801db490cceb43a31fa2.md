# [ADS] [DevOps-I] Mexendo com arquivos/pastas e volumes

Quando você cria uma imagem Docker, você define o sistema de arquivos que estará dentro do container.

- **COPY:** copia arquivos e pastas do host para o container;
- **ADD:** similar ao copy, mas com funcionalidades extras, como descompactar arquivos `.tar` e baixar arquivos remotos.

```powershell
# comandos + caminho
# -v /C/docker/volumes/nginx: = caminho do volume
# -p 9180:80 = define a porta
# nginx:lasted = usa a versão lasted a última
# -d = roda em background

docker run --name Nginx2 -v /C/docker/volumes/nginx:/usr/share/nginx/html -p 9180:80 -d nginx: lasted
```

**Volumes no Docker:** são mecanismos para persistir dados gerados ou usados por um container. 

**Notes:** "pastas compartilhadas" entre o container e o host (seu computador ou servidor).  Os dados armazenados nessas pastas permanecem intactos, mesmo que o container seja excluído.

```powershell
# comandos + caminho
# -p 9180:80 = define a porta
# -d nginx: lasted = usa a versão lasted a última!
docker run --name Nginx2 -v /C/docker/volumes/nginx:/usr/share/nginx/html -p 9180:80 -d nginx: lasted

```