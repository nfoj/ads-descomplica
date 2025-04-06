# [ADS] [Monitoramento] Introdução ao Prometheus

O Prometheus é um sistema de monitoramento e coleta de métricas de código aberto, amplamente utilizado em ambientes de microsserviços e nuvem. Ele foi desenvolvido originalmente pela SoundCloud e se tornou parte da Cloud Native Computing Foundation (CNCF). A seguir, apresento uma introdução ao Prometheus, abordando suas principais características, arquitetura, casos de uso e benefícios.

É uma ferramenta de monitoramento que coleta e armazena métricas em formato de séries temporais. Ele permite que os usuários consultem essas métricas e as visualizem em dashboards ou as utilize para alertas. O Prometheus é particularmente popular em ambientes de contêineres e microsserviços, onde a dinâmica do sistema é frequentemente complexa e em rápida mudança.

Para coletar dados das aplicações e sistemas utilizando o Prometheus, existem algumas formas:

**1. Pull (Coleta por solicitação)**

a. O sistema faz requisições periódicas a endpoints expostos pelas aplicações para coletar métricas.

b. Vantagens: Oferece simplicidade na configuração e melhor controle das coletas de dados.

c. Desvantagens: Pode não capturar eventos em tempo real se o intervalo de coleta for longo.

**2. Push (Coleta por envio)**

a. Aplicações enviam métricas diretamente para o sistema de monitoramento.

b. Vantagens: Captura imediata de eventos e mudanças e útil para aplicações que não podem expor endpoints.

c. Desvantagens: Pode complicar a coleta de dados em cenários de alta disponibilidade.

**3. Logs**

a. Coleta dados de logs gerados pelas aplicações e sistemas.

b. Vantagens: Captura de detalhes ricos sobre eventos, erros e comportamento da aplicação.

c. Desvantagens: Pode complicar a coleta de dados, especialmente em cenários de alta disponibilidade.

**4. Traces**

a. Coletar dados de rastreamento distribuído para entender como as requisições fluem através de sistemas e serviços.

b. Vantagens: Fornece visibilidade em sistemas complexos e ajuda a identificar gargalos.

c. Desvantagens: Pode exigir configuração adicional e ferramentas específicas.

Coletar métricas é fundamental para o monitoramento e a gestão eficaz de sistemas, aplicações e serviços. Algumas razões que destacam a importância dessa prática são:

**1. Monitoramento de Desempenho**

Identificação de Problemas: Métricas fornecem insights sobre o desempenho do sistema, permitindo identificar gargalos, falhas e degradação no serviço em tempo real.

Análise de Tendências: A coleta contínua de métricas permite a análise de tendências ao longo do tempo, ajudando a prever futuros problemas de desempenho e a realizar ajustes proativos.

**2. Tomada de Decisão Baseada em Dados**

Fundamentação para Decisões: Métricas ajudam as equipes a tomar decisões informadas sobre escalabilidade, alocação de recursos e planejamento de capacidade, reduzindo a dependência de suposições.

Avaliação de Mudanças: Ao coletar métricas antes e depois de alterações no sistema (como atualizações de software ou mudanças na infraestrutura), as equipes podem avaliar o impacto das mudanças.

**3. Otimização de Recursos**

Eficiência Operacional: Métricas permitem que as organizações identifiquem áreas onde os recursos estão sendo subutilizados ou super utilizados, possibilitando uma alocação mais eficiente.

Redução de Custos: Através da análise de métricas, as empresas podem reduzir custos operacionais ao eliminar desperdícios e otimizar o uso de infraestrutura.

**4.  Experiência do Usuário**

Melhoria na Satisfação do Cliente: A coleta de métricas relacionadas ao desempenho da aplicação, como tempo de resposta e taxas de erro, permite que as equipes identifiquem e resolvam problemas que afetam a experiência do usuário.

Feedback Contínuo: Métricas ajudam a captar feedback contínuo sobre o uso e a eficácia das aplicações, permitindo ajustes baseados nas necessidades dos usuários.

**5. Compliance e Segurança**

Conformidade com Regulamentações: A coleta de métricas pode ajudar as organizações a atenderem a requisitos regulatórios, mantendo registros de desempenho e segurança.

Identificação de Ameaças: Métricas de segurança permitem a detecção precoce de atividades suspeitas e potenciais brechas de segurança.

**6. Facilidade de Diagnóstico e Resolução de Problemas**

Diagnóstico Rápido: Quando ocorrem falhas, as métricas ajudam as equipes a diagnosticar rapidamente a causa raiz do problema, reduzindo o tempo de inatividade e melhorando a eficiência na resolução.

Análise de Logs: Métricas podem ser correlacionadas com logs de eventos para fornecer uma visão holística do que está acontecendo em um sistema.

O uso de métricas e rótulos (ou labels) é fundamental para o monitoramento eficaz de sistemas e aplicações. Eles permitem a coleta, a organização e a análise de dados de maneira estruturada e significativa.

**1. Métricas**

Métricas são medidas quantitativas que descrevem o desempenho ou a saúde de um sistema, aplicação ou serviço.

**2. Rótulos (Labels)**

Rótulos são pares chave-valor que fornecem contexto adicional sobre as métricas coletadas. Eles permitem que as métricas sejam categorizadas e filtradas de forma mais eficaz.

Alertas e regras de monitoramento são componentes cruciais para garantir a disponibilidade e o desempenho de sistemas e aplicações. Eles ajudam a identificar problemas em tempo real e a tomar ações corretivas rapidamente. Vamos explorar o que são alertas, como funcionam as regras de alerta e como implementá-las efetivamente.

**1. Alertas**

Alertas são notificações que são geradas quando certas condições ou limites pré-definidos são atingidos ou excedidos. Eles permitem que as equipes identifiquem problemas, como degradação de desempenho, falhas de serviços ou interrupções, antes que esses problemas impactem os usuários finais.

**2. Regras**

As regras definem as condições sob as quais os alertas são gerados e como eles devem ser tratados. 

A estrutura de uma regra de um alerta geralmente inclui:

- **Nome do Alerta**: um identificador descritivo para o alerta.
- **Condições**: a lógica que determina quando o alerta deve ser disparado.
- **Intervalo de Avaliação**: a frequência com que a condição deve ser avaliada.
- **Notificações**: os canais para os quais as notificações devem ser enviadas (e-mail, Slack, etc.).