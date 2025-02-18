# [ADS] [DevOps-I] Eficiência operacional em entrega de software

**Processo:** 

- **Build:**  transforma o código fonte da sua aplicação em um artefato executável;
- **Deploy:**  pega o construído na etapa anterior e o coloca em um ambiente onde ele possa ser executado;
- **Run:** é o momento em que a aplicação está efetivamente em funcionamento.

**SCM (Software Configuration Management):** é um conjunto de práticas e ferramentas que permitem controlar e organizar as mudanças no código fonte de um software ao longo do tempo.

**Note:** o bom e velho git!

Merger:  é quando mistura todo código que todo mundo fez e reza! 🙏

**Gitlab:** conjunto abrangente de ferramentas para gerenciar todo o ciclo de vida do desenvolvimento de software.

**Note:** irmão do github! 🐈 + 🐙

**Principais Funcionalidades:**

- **Repositórios:** hospedagem para repositórios Git;
- **Versões:** acompanhar cada alteração feita no código;
- **Issues:** criar, acompanhar e gerenciar issues (problemas, bugs, solicitações de funcionalidades);
- **Integração Contínua e Entrega Contínua (CI/CD):** automatizar o processo de build, teste e deploy da sua aplicação;
- **Pipelines:** fluxos de trabalho complexos para o seu CI/CD, com várias etapas e condições;
- **Colaboração: f**erramentas para facilitar a colaboração entre os membros da equipe;
- **Segurança:** autenticação de dois fatores, controle de acesso e análise de vulnerabilidades.

 **Pipelines:** sequência de etapas ou estágios que são executados automaticamente.

- **Build:** Compilar o código fonte, gerar executáveis ou imagens de container.
- **Test:** Executar testes automatizados (unitários, de integração, etc.) para garantir a qualidade do código.
- **Deploy:** Implantar a aplicação em um ambiente (desenvolvimento, teste, produção).
- **Análise:** Realizar análises estáticas de código, verificar vulnerabilidades de segurança.
- **Notificação:** Enviar notificações sobre o status do pipeline (sucesso, falha).

**Note: pense pipeline como um script > uma receita de bolo que você faz e um robozinho vai reproduzir!**  🦊 + 🤖

> Atividade prática
> 

O primeiro passo é entender que ao longo dos anos o processo de desenvolvimento de software foi sendo alterado conforme as necessidades que foram aparecendo dentro do modelo vigente, logo se percebeu que a forma anterior de desenvolvimento estava dificultando a entrega dos projetos e isso se deu por diversos fatores, irei citar alguns deles aqui:

**Detecção precoce de erros:** com uma integração frequente e com testes automatizados, podemos identificar problemas logo no inicio do desenvolvimento, evitando que se acumulem, logo mesmo que o projeto cresça significativamente, dificilmente teremos problemas maiores no futuro.

**Melhoria continua na qualidade do código:** o código sempre estará funcionando e as melhorias podem seguir sendo implementadas a medida que vai sendo produzido, pois a medida que vai sendo entregue, podemos avaliar, testar e melhorar o que está escrito sem impactar negativamente o que outros desenvolvedores da equipe está fazendo.

**Aumento na agilidade:** entrega continua permite entregas menores, mais eficientes e mais rápidas do projeto final, mesmo que o projeto não esteja 100% completo, muitas vezes podemos pegar o que está pronto e colocar em testes de produção, pois o código está sendo escrito continuamente e melhorado continuamente.

**Redução dos riscos:** o código está sendo entregue em pequenos pedaços, logo precocemente os problemas maiores vão ser encontrados e resolvidos, evitando no final do projeto, atrasos e custos adicionais.

**Melhora na colaboração:** compartilhar o que está sendo escrito sempre foi um problema, seja pois docs, via e-mail ou ferramenta, leva-se um certo tempo para receber e entender o que foi compartilhado, logo quando se melhora esse processo, mais facilmente conseguimos seguir dando continuidade aquilo que estava sendo produzido inicialmente por outro colaborador. 

**Cliente:** a produção de software é um produto, logo por vezes o cliente precisa ter acesso ao que foi produzido para entender que está sendo feito e para ter acesso aquilo que está pagando, logo ele é um das pessoas que pode mais facilmente o que está sendo entregue continuamente.

Se a algo importante a se pensar é, que todas essas melhorias são frutos dos problemas que foram sendo encontrados ao longo dos anos de desenvolvimento de software que foram sendo criadas soluções que foram sendo implementadas e compartilhadas por diversas pessoas, para que pudessem ser aplicadas no dia a dia do desenvolvimento, pois não só facilitam o trabalho, mas melhoram em muito om processo de chegar na reta final do projeto independente do tamanho que ele possua.