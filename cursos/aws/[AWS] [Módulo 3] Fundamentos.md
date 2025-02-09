# [AWS] [Módulo 3] Fundamentos

```markdown
# [AWS] [Módulo 3] Fundamentos

## Modelo de Pagamento
- Pagamentos sobre demanda
 - Só paga pelo usar
 - Paga menos ao reserva
 - Paga menos quando usar mais a medida que aws cresce
 - Paga apenas pelo serviço que você consome

- Reserva de Instância (1 a 3 anos prometo que uso) - Desconto ate 75%
	- Os pagamentos podem ser adiantados (Auri)!
	- Adiantamento Parcial (Puri)
	- Sem adiantamento (Nuri)
	- Se você não reserva você usa sob demanda (Padrão)

- Serviços sem custo
	- alguns serviços não tem custo, mas quando começar a usar ou colocar coisa para funcionar eles vão ter custo.

- Nível Gratuito
	- Verificar as regras, boa parte delas é teste (1 ano), mas temos limites.

- Calculadora TCO
	- Estimativa de custos
	- Tem a versão mensal

- Serviço de Custo
	- AWS Billing and Cost Management > Ele indica o custo e os próximos valores
	- As tags são muito importantes aqui na hora de gerar relatórios

- Budgets
	- Aqui você consegue limitar um valor x por mes e acompanhar

- Suporte
	- Básico
	- Desenvolvedor
	- Comercial
	- Empresarial

```

```markdown
## AWS Cloudwatch

Serviço resposável por ajudar no monitoramento dos serviçoes e aplicações.

- Alarme: vai notificar com base no que foi definido.

Exemplo:
	- Aplicação
		- CPUUltilization
			- 80% 60% 40% 25% 10% ...
			- Alarme: Se a métrica CPUUltilization > 50% por 5 minutos
				- Adiciona ação: Enviar uma notificação para a equipe.

```

" . 

```markdown
## AWS Autoscaling

Monitora os aplicativos e ajuda automaticamente a capacidade de manter uma performace constante e com menor custo possível

A capacidade aumenta e diminui conforme a necessidade

Vantagens:
	- Ajuda manter a disciponibilidade do aplicativo
	- Permite adicionar ou remover automativamente instâncias EC2 com base no que foi definido
	- Detecta instâncias danidicadas e aplicativos não integros e substitui automaticamente
	- Fornece opções de escalabilidade.
	
Você pode criar um group de auto scaling, assim permitindo maior controle.

```

Autoscaling 

![image.png](image.png)

Melhorando a arquitetura

![image.png](image%201.png)

```markdown
## AWS Autoscaling

- Primeiro cria uma AMI (imagem) 
	- Você pode criar uma instância como modelo, configurada e depois criar a imagem a parti dela.
	- Configurações mais detalhadas não entram aqui! Fica a critério da equiep como fica o template.
	
	
- Criar um grupo autoscaling
	- Nome do Grupo: nome
	- Modelo de Execução: 
		- Defini as caracteristas do template e adiciona o template criado
	- Seleciona o grupo
	- Rede
		- VPC: Private (o objetivo é segurança)
	- Balanceador de Cargas
		- Seleciona o load balance
	- Metricas: você pode adicionar metricas para relatórios
	- Tamanho do grupo:
		- Mínimo: 2 (padrão)
		- Máximo: 4 (padrão)
	- Politíca de Escalabilidade
		- Configura com base na sua necessidade
	- Notificação: adicione se for necessário
	- Tags: recomendável colocar
	- Criar!
		
		

- Regras de Autoscaling
	- Detalhes: é tudo que está acontecendo no seu grupo
	- Escalabilidade automatica: já está normalmente predefinido
		- Ação programada: 
			- Nome:
			- Capacidade: 2 > 4 (Minimo) // 4 > 8 (Maximo)
			- Defini o dia e os horários (ideal definir 30 min antes)
			- Criar!
		- As regras podem ser criadas para aumentar ou diminuir a capacidade.
			
		
		
		
		
```

```markdown
## AWS Banco de Dados

- Não Gerenciados: cliente é responsável
- Gerenciados: aws é responsável

- Relacional: 
	- Linhas e colunas
	- Fixo
	- Usa SQL
	- Vertial
	- Exemplo:
		ISBN |   Titulo  | Autor | Formato 
		1203 | WITHERING | Jack  | Brochura
	
- Não relacional:
	- Chave valor,documento, grafo.
	- Dinâmico
	- Focado na coleta de documentos
	- Horizontal
	- Exemplo: 
		{
			ISBN: 321224556
			TITULO: "WITHERING"
			AUTOR: "JACK"
			FORMATO: "BROCHURA" 
		}

- Relacional
	- Facilidade de uso
	- Integridade de dados
	- Redução do armazenamento do dados
	- Linguagem comum de constulta
	- Exemplo:
		 Banco < > Servidor < > Usuário 

- Não Relacional
	- Flexibilidade
	- Escalabiliaded
	- Alta performance
	- APIs Altamente funcionais
	- Exemplo:
		Chave 1 > Valor 1
		Chave 2 > Valor 2
		Chave 3 > Valor 3
```

```markdown
## AWS RDS (Relational Database Service)

- A amazon RDS é um serviço gerenciado que configura e opera um bando de dados relacional na nuvem
- O armazenamento RDS permite que seu aplicativo proporcione perfomance, alta disponibilidade, segurança e compatibilidade
- O foco principal passa para os dados e a otimização do aplicativo

- Gerenciamento da AWS:
	- Instalação de patches do SO
	- Instalação de patches de softaware de banco de dados
	- Backup do banco
	- Disponibilidade
	- Escalabilidade
	- Energia e servidores em rack e pilha
	- Manutenção do servidor

- A amazon tem seu banco de dados relacional chamado: Amazon Aurora! Tem suas melhorias e que pode ser usado dependendo da sua necessidade.
- A amazon também tem  dynamodb para bancos não relacional, verificar caso seja necessário

- Criar
	- Cria grupo de sub-redes para seu banco de dados
	- Adiciona zonas: sub-redes
	- Adiciona sub-rede: privadas
	
- Cria banco de dados
	- Escolhe o Banco
	- Versão
	- Modelos: Produção, Desenvolvimento, Gratuito
	- Configuração:
		- Nome: 
		- Senha:
		- Instância: 
		- Armazenamento:
		- Escalabilidade automática: 
		- Provisionamento: escolhe o grupo de subrede
		- Acesso: privado!
		- Grupo de segurança: indicado criar um novo para o Banco
		- Zona de disponibilidade:
		- Porta:
		- Autenticação:
		- ... configurações de preferencia da sua necessidade.
		
Nota:
	- Se trabalha em empresa de auditória, normalmente ativa os logs.

```