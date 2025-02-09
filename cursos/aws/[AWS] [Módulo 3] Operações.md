# [AWS] [Módulo 3] Operações

```markdown
# [AWS] [Módulo 3] Operações

## Cloud Trail

- Registra e monitora todas as ações de um usuário; 
- Fornece histórico de eventos da conta da aws;
- Idetifica: quem acessou, quando acessou, por onde acessou;
- Realiza análise de segurança.

Funcionalidades: 
	- Cloudtrail events: registro de atividades da conta
		- Event history: histórico de atividades até 90 dias
		- Couldtrail trail: cria uma trilha que é monitorada que pode arquivar, analizar e responder por recursos, além de possuir um período maior que 90 dias.
		- Manager events: informações sobre operações realizadas nos recursos da conta
		- Data events: informações sobre os dados dos recursos
		- Insigts events: usado para identificar atividades incomuns associada a chaadas de API Write

Couldtrail trail: quando criamos uma trilha conseguimos ter controle sobre os eventos que estão acontecendo em todas as regiões e não apenas em uma delas, além de possuir um período maior que 90 dias.
	- Criando uma trilha:
		- Acesse trail
		- Name:
		- Selecione aonde vai armazenar
		- Critpograph: sim ou não
		- IAM Role: ela já está pré-criada e basta colocar um nome
		- Selecione os eventos que deseja ter informações: 
		- API: 

## Cloud Watch

- Usado para monitorar o ambiente e gerar alertas caso necessário
- Os dados relacionados ao monitoramento do sistema são através de métricas e se mantem armazenado por 15 meses.

- Anomaly Detection Metric: você pode ativar para gerar métricas e saber se seu serviço está saindo da média por exemplo.
- Alarmes: os alarmes podem ser criados para determinadas métricas
	- Ok: dentro do limite definido
	- Alarm: atingio o limite definido e passou
	- Insufficient_data: não a dados suficientes ou o alarme acabou de ser criado

- Períodos: intervalo de tempo para avaliar a métrica
	- Tempo: você define o tempo que ele vai avaliar (1h, 3h, 6h ...)
	- Períodos: quantas vezes dentro desse tempo ele vai avaliar
	- datapoints: quantas vezes ele ultrapassou o definido com base nos períodos

## Trusted Advisor

- Inspeciona o ambiente e faz recomendações de melhorias e como economizar, otimizar e melhorar a segurança.
	- Dependendo do seu nível de suporte você terá apenas 6 opções disponíveis.

- Ele faz um scam para investigar com base nas boas práticas e assim ele consegue fazer recomendações.

- Categorias: 
	- Otimização de Custos;
	- Performace;
	- Segurança;
	- Tolerância a falhas;
	- Limites de Serviço.
	

## Limits

- As informações referentes aos limites da sua conta estão disponíveis para que possa acessar e verificar se está dentro das suas necessidades. 

## Elastic Beanstalk

- Serviço gerenciado pela AWS voltado para aplicativos WEB

- Usuário Gerencia: 
	- Seu código
	
- AWS Gerencia: 
	- Servidor HTTP
	- Servidor de aplicativos
	- Intérprete de Linguagem
	- Sistema Operacional
	- Host

- Linguagens suportadas: java, php, node, .net, python, docker ... 

```