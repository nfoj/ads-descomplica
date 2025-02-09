# [AWS] [Módulo 1] Operações

```markdown
# [AWS] [Módulo 1] Operações

## AWS IAM (Identity and acess management)

- Controla do acesso individual e de grupo e seus recursos na aws
- Integração a outros serviços
- Gerenciamento de identidade federado
- permissões granulares
- suporte à autentificação multifator

- Usuário Raiz (Root!)
	- Cuidado:
		- Crie um usuário de acesso
		- Bloquei as credenciais do usuário raiz
		- Amplie o acesso a contas com esse usuário raiz uma boa e criar grupos e dar permissões a esses grupos
		
		
- Divisão:
	- Usuário do IAM
	- Grupos do IAM
	- Políticas do IAM
	- Funções do IAM (Roles)

- Usuário do IAM: criado a parti do Root
	- A mesma conta pode possuir vários usuários
	- O user possui credenciais de acesso e tem atribuido a ele permissões para que possa trabalhar.

		
- Grupos: são vários usuários que podem ser configurados juntos. 
	- O mesmo usuário pode pertencer a vários grupos, mas não existe grupo dentro de grupo.
	- As permissões ao ser aplicadas em um grupo valem para todos os usuários desse grupo. 	

- Políticas: gerencia as permissões para usuários do IAM, grupos e roles. (Resummindo: Regras que vão ser associadas a um usuário ou a grupos.)
	- Gerenciadas: essas regras estão prontas e foram feitas pela AWS e você as utiliza tanto em usuários como em grupos. 	
	- Inline: regras criadas pelo próprio usuário e gerenciadas por ele, podem ser usadas tanto em usuários quanto em grupos.

Exemplo: Efeito (Permitindo ou negando), ação (ele pode fazer o que?), recurso (ele vai fazer isso aonde) e condição (quais condições ele deve cumprir para aplicar a regra)

- A campos que são obrigatórios e outros que não, logo tem que entender quais se encaixam na sua necessidade.
- A AWS possui um Gerador de Politicas pa IAM
	- Type: tipo de política
	- Effect: permitido ou negado
	- Service: seleciona o serviço
	- Actions: seleciona a ação 
	- ARN: selecione o que mais lhe interessa ou caso seja tudo *.

- Funções(Roles): uma role é parecida com um usuário IAM, logo possui policies com permissões, mas é temporária.
	- É como se fosse uma chave de acesso temporária, tem regra associadas a ela e pode ser usada por um usuário ou grupo ou por um serviço de forma temporária. 
	- Pode-se associar uma role a um serviço AWS:
		- Uma role que permite EC2 acessar o S3. 
		- Recomendável usar roles a credenciais a uma instância. 

## AWS CLI (Acesso via terminal)

- Shell (Bash, ZSH, ...)
- Cadatrado na AWS
- Criar uma conta de Usuário IAM
- Criar um ID de chave de acesso e a chave de acesso secreta
- Instalar o AWS CLI
- Configurar com as credenciais de usuário IAM

- Depois de instalar para seu sistema, se faz necessário ir no usuário administrador e gerar um Access key e uma Secret Key
	- As informações de acesso e secret key não devem ser compartilhadas, pois elas que dão ao usuário o poder de fazer o que estiver permitido.

- Configuração: aws configure ou import .csv (basta acessar a aws)
	- Preencher os dados: 
		- AWS Access key:
		- AWS Secret Key:
		- Region: coloca a região que está trabalhando
		- Format: formato que prefere (.json, yaml, text)
	
Note: aws configure --profile name-user

Ao usar --profile você estará acessando a conta através de um usuário especifico.

A lista de comandos pode ser vista na página da AWS.
	- Estrutura: aws [serviço] [comando] [detalhes] informação
	- Exemplo: aws iam create-user user-name roberto

```