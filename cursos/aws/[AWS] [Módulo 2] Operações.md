# [AWS] [Módulo 2] Operações

```markdown
# [AWS] [Módulo 2] Operações

## Cloudformation
	- Infraestrutura como código
	- Reduz:
		- Implementação mais rápidas de ambientes mais complexos
		- Consistência das conigurações
		- Exclusão mais eficiente (Exclui a pilha)
		- Mais fácil para aplicar modificações
	- Benefícios:
		- Facilidade de reuso e repetibilidade
		- Fácil manutenção
		
- Simplifica o processo de modelar, criar e gerenciar coleção de recursos da AWS.
		- A coleção de recursos é chamada de pilha do AWS cloudformation
		- Não tem custo para usar
	- Criar, atualiza e exclui pilhas
	- Permite provisionar e atualizar ordenamente de forma previsivel
	- Possui o controle de versões
	
- Resumo: 
	- Define o modelo ou usa um modelo pronto (arquivo com instruções) > Upload do modelo > Criação da pilha (executa os comandos definidos no arquivo) > Pilha funcionando (Atualiza, Editar, Excluir)
	
	
Modelos:
	- Yaml: 
		- Menos detalhado (sem os caracteres {} "")
		- Suporta comentários incorporados
	- Json:
		- Mais utilizado por outros sistemas (Ex: APIs)
	- Esses templates podem ser criados no AWS Cloudformation Designer 
	

Sessões: as partes que compões seu modelo
	- Resources: obrigatório
	- format version: a versão do modelo (dd:mm:ano)
	- description: uma descrição do seu modelo  
	- metadata: informações adicionais
	... acesse o site e veja mais detalhes sobre o asusnto: Seções de modelos do CloudFormation
	
	
	
## System Manager
	- Serviço que poder usar para visualizar e controlar a infraestrutura
		- Pode exibir dados operacionais de vários serviços e automatizar tarefas
		- Ajuda a manter a seguraça verificando seus Nós Gerenciados e gerando relatórios tomando medidas corretivas.
	
```

![image.png](image.png)

```markdown
## System Manager

Recursos:
	- Gerenciamento de Operações: incident manager, explore, opscenter, cloudwatch;
	- Gerenciamento de Aplicação: application manager, appconfig, parameter store;
	- Gerenciamento de Alterações: gerenciar alterações, automation, calendário, janelas de manutençao;
	- Gerenciamento de Nós: compliance, fleet manager, inventory, session manager, execute comando, state manager, patch manager, distributor, hybrid activations;
	- Recursos Compartilhados: documents.
	
	
Como conectar o sistem manager:
	- A aws libera um passo a passo para criar a conexão através de uma rule: Configurar permissões de instância obrigatórias para o Systems Manager
			- https://docs.aws.amazon.com/pt_br/systems-manager/latest/userguide/setup-instance-permissions.html
	

	
```