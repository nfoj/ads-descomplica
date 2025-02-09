# [AWS] [Módulo 2] Fundamentos

```markdown
# [AWS] [Módulo 2] Fundamentos

## S3

- Serviço de Armazenamento da AWS:
	- Armazena quantidades ilimitadas de dados estruturados;
	- Arquivos de dados são armazenados como objetos em um bucket;
	- O tamanho máximo de cada objeto é 5TB;
	- Todo objeto tem sua própria URL acessível por REST (Namespace Universal);
	- Todos os objetos possui: chave, id, valor, metadados e sub-recursos.

- Durabilidade
	- Os dados não serão perdidos ou posui uma chance pequena.
	
- Disponibilidade
	- Permite que os dados sejam acessados de qualquer lugar e a qualquer momento.

- Escalabilidade
	- Espaço quase ilimitado para novos objetos

- Segurança
	- Auto controle de acesso e formas de proteçõa de dados.

- Performace
 - Compativel com vários padrões.
 
 
 
 - Controle de Acesso:
	 - Padrão: somente o usuário pode acessar
	 - Público: qualquer pessoa pode acessar os dados (Sites)
	 - Restrito: esse acesso é configurado de forma a que, o dono dos dados defina quem pode ou não acessar esse dados. 
	 
```

Controle de Acesso: (1.)

![image.png](image.png)

```markdown
# [AWS] [Módulo 2] Fundamentos

## S3

- Controle de Acesso:
	- Bloquei de acesso público;
	- Política de IAM;
	- Política de bucket;
	- Lista de controle de acesso (ACLS);
	- Ponto de Acesso (Permissões);
	- URLs de acesso;
	
- Criptografia
	- Sem;
	- Lado do Servidor(Amazon e reponsável);
	- Lado do Cliente(Cliente é responsável);
	
- Armazenamento
	- S3 Standard: acesso com frequência;
	- S3 Standard IA: acessados com pouca frequência;
	- S3 One Zone IA: não críticos e acessados com pouca frequência;
	- Amazon S3 Glacier ou Deep Archive: frequência de acesso muito baixa.
	
- Ciclo de Vida: quanto mais acessado é um dado, mais caro é o serviço, 
logo o ideal é criar um ciclo de vida e a cada determinado período ele saia
de uma classe e vá para outra
	S3 Standard > S3 Standard IA > S3 Glacier

	
```

```markdown
# [AWS] [Módulo 2] Fundamentos

## EC2

- Launch Instance Wizard (9 Etapas)
	- AMI
	- Tipo de Instância
	- Configuração de Rede
	- Função do IAM
	- Dados de usuário
	- Opção de Armazenamento
	- Tags (Organizar)
	- Grupo de segurança
	- Par de chaves 

- Tags 
	- Chave: Name
	- Valor: servidor-web

- Grupo de segurança
	- Pode criar um por instância ou ter vários para a mesma instância
	- Regras
		- Tipo: SSH 
		- Protocolo: TCP
		- Intervalo de Portas: 22
		- Origem: Personalizado (192.168.0.0/0 ...)

- Exemplo: Apache
	- Regras
		- Tipo: HTTP 
		- Protocolo: TCP
		- Intervalo de Portas: 80
		- Origem: Personalizado (0.0.0.0/0)
	
- Par de chaves
	- Verificação final e a criação de 2 chaves (1 Privada e 1 Pública) para acesso.

Imporante!
- Ao criar uma instância EC2 você terá ao final um IP público variável, logo toda vez que a instância reiniciar, seu IP muda.
- Lembre-se de criar um IP Elástico ou usar um IP existente, depois basta associar a sua instância e agora terá um IP público fixo.

- Replicar uma AMI
	- Ao clicar com botão direito numa instância, você pode criar uma imagem (Cópia) igual a instância pronta.
	

### Load Balancer

- Distribui o tráfego de entrada do aplicativo ou da rede entre vários detinos em uma única zona de disponibilidade. Em outras palavras, ele verifica se ta funcionando, se tiver ele manda, se não ele não manda.
- Ele automaticamente vai escalando a medida das requisições vão chegando.

- Tipos:
	- Application load balance: balanceamento de carga http e https; 
	- Network load balancer: balanceamento de carga de tráfego TCP, UDP, TLS, alta performace (recomendado quando as cargas são muito altas).
	

- Criando: Target Group
	- Crie um Target Group (Grupo de Servidores)
		- Instance, IP addresses ...
		- Target group name: name
		- Protocolo: HTTP, HTTPS ... 
		- Port: 80, 22 ...
		- VPC: seleciona
		- Health checks: / (endereço para teste)
			- Port: Traffic port (defina se vai ser a padrão usada acima(80) ou outra)
			- Healthy threshold: 5 (numero de vezes que ele vai bate na porta e receber resposta)
			- Unhealthy threshold: 2 (numero de vezes que ele bate e não obtem resposta) 
			- timeout: 5 (tempo de intervalo a cada pergunta)
			- Interval: 30 (quanto quanto tempo ele pergunta)
			- Code: 200 (retorna tudo ok!)
			- Confirma!
				- Seleciona as instâncias;
				- Cria o target group
			
	
- Criando: Load balance
	- Load Balance Name: name
	- Scheme: internet facing (publico) ou internal (privado)
	- IP address type: IPV4 
	- VPC: escolhe
	- Mappings: selecione suas zonas de disponibilidade (lembra de marca a publica)
	- Security groups: recomendavel criar um novo. (configura normal)
	- Listeners and routing: seleciona o grupo de instância
	- Tag: add tag
	- Create!

Nota:

Status de Códigos HTTP:
- 1XX: Informações
- 2XX: requisição deu certo
- 200: OK
- 201: CREATED (alguma coisa foi criada)
- 202: ACCEPTED (deu tudo certo, mas está em processamento)
- 204: deu certo, mas não vou falar nada!
- 3XX: redirecionamento (o que você procurou, não está aqui em ...)
- 301: esse link foi redirecionado (estava no A, mas vai ser redirecionado para B)
- 302: redirecionamento temporário (fui ali e já volto) 
- 4XX: ERRO (cliente fazendo besteira)
- 400: Bad Request (cliente fazendo besteira)
- 401: Unauthorized (cliente tentando acessar algo que não está cadastrado!)
- 403: Forbidden (cliente logado, tentando acessar uma área que não pode!)
- 404: página não encontrada
- 405: method not allowed (Erro no metodo get // post - Subir algo)
- 5XX: ERRO (servidor fazendo besteira)
- 500: erro interno do servidor (seu site ta com erro)
- 501: erro not implement yet (nao foi implementado - estou fazendo ainda)
- 502: bad gateway
- 503: Service Unavaiable (em manutenção) 

```

![image.png](image%201.png)

```markdown

## Arquitetura mais segura

- Gere as AMIS de cada instância
	- Botão direito > Criar imagem > Configura e Cria imagem.
	
- Provisiona Intância
	- Vai na imagem e cria (Não tem como mover uma instância, você pode apenas criar e deletar)
	- Criar 2 instâncias privadas (o primeiro exemplo fou público, mas o ideal e privado)

- Criar o target group ou adicionar as 2 privadas (cria é melhor)
	- instances
	- protocolo HTTP
	- ... 
	- adiciona as instâncias privadas
	
- Load balance
	- Edit
	- muda para o private
	- atualiza! 
	
```