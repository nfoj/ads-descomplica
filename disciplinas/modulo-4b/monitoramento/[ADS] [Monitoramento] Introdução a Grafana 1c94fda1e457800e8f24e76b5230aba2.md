# [ADS] [Monitoramento] Introdução a Grafana

Grafana é uma plataforma de visualização de dados de código aberto que permite criar dashboards interativos e dinâmicos, ideal para monitoramento e análise em tempo real. É amplamente utilizada por equipes de TI, engenheiros e analistas para visualizar dados de diversas fontes de maneira intuitiva e personalizável.

É uma plataforma amplamente utilizada, pois se conecta a diversas fontes de dados, como Prometheus, InfluxDB, Elasticsearch, MySQL, PostgreSQL, entre outras. Isso possibilita a integração de dados de diferentes sistemas em um único dashboard. O Grafana também possui dashboards personalizáveis com vários tipos de visualizações que auxiliam na visualização para análise dos dados.

Outro aspecto relevante do Grafana é a possibilidade de configurar alerta com base em condições específicas, o que torna possível a detecção precoce de problemas. Além disso, a ferramenta suporta plugins para novas fontes de dados, novos tipos de dados e visualizações, o que aumenta a flexibilidade e torna-a uma ferramenta altamente eficaz no monitoramento de aplicações.

Quando falamos sobre a criação de alerta, é importante entender que eles são mecanismos que detectam quando uma condição específica em um sistema atinge um estado que requer atenção. Esses estados podem ser baseados em métricas, eventos ou thresholds (limites) predefinidos. Alguns tipos comuns de alerta que podemos citar são:

1. **Threshold Alerts**: Alerta que dispara quando uma métrica excede ou fica abaixo de um limite específico. Por exemplo, se a utilização da CPU ultrapassar 90%, um alerta pode ser acionado.

2. **Anomaly Detection**: Alerta que utiliza algoritmos para detectar comportamentos incomuns nas métricas em comparação com o histórico. Isso é útil para identificar problemas inesperados.

3. **Status Change Alerts**: Alerta que notifica quando há mudanças em um status, como um serviço que fica fora do ar ou um sistema que muda de um estado saudável para um estado de erro.

Alerta e notificações são componentes cruciais em sistemas de monitoramento de software. Eles desempenham um papel essencial na identificação e comunicação de problemas em tempo real, permitindo que equipes de TI ou administradores de sistemas tomem medidas rápidas para corrigir falhas e minimizar o impacto de eventuais problemas.

**1. Alerta**

Alerta são sinais ou notificações automáticas geradas por um sistema de monitoramento quando um comportamento anômalo ou anormalidade é detectado. Eles servem para indicar que algo fora do padrão está ocorrendo e que atenção imediata pode ser necessária. Alerta pode ser configurados com base em parâmetros específicos, como:

- **Erros de sistema**: Falhas de software ou hardware que afetam o funcionamento normal.
- **Desempenho abaixo do esperado**: Como altos tempos de resposta, uso excessivo de CPU ou memória, e falhas de rede.
- **Limites excedidos**: Por exemplo, limites de tráfego de rede, número de requisições ou espaço em disco.
- **Falhas de segurança**: Tentativas de invasão, acessos não autorizados, ou vulnerabilidades detectadas.

Esses alertas podem ser configurados para diferentes níveis de gravidade, como:

- **Críticos**: Indicam falhas graves que podem interromper o funcionamento do sistema ou serviço.
- **Avisos**: Indicam condições que podem levar a problemas, mas ainda não causam falhas graves.
- **Informações**: Relatam o estado atual do sistema sem indicar um problema, mas ajudam a fornecer contexto.

Quando um alerta é disparado, ele pode acionar ações automáticas, como o reinício de processos, a ativação de backups ou o ajuste de configurações, ou ainda acionar equipes responsáveis por solucionar o problema.

**2. Notificações**

Notificações são mensagens enviadas para informar os responsáveis sobre a ocorrência de alerta ou eventos importantes que demandam atenção. Embora os alertas sejam, em sua essência, os sinais do problema, as notificações são o meio pelo qual esses sinais são comunicados aos usuários ou administradores do sistema.

As notificações podem ser enviadas de várias maneiras, como:

- **E-mail**: Método comum para enviar notificações a administradores de sistemas ou equipes de suporte.
- **SMS**: Pode ser usado para alertar sobre problemas críticos que precisam de uma resposta imediata.
- **Push notifications**: Mensagens enviadas para dispositivos móveis ou para aplicativos de desktop em tempo real.
- **Sistemas de mensagens instantâneas**: Como Slack ou Microsoft Teams, que são usados para enviar alerta de forma colaborativa e em tempo real.
- **Painéis de controle**: Alguns sistemas de monitoramento oferecem notificações visíveis em interfaces gráficas em tempo real, com informações sobre o estado dos sistemas monitorados.

As notificações podem ser configuradas para fornecer detalhes sobre a natureza do problema, o impacto potencial, e instruções de ação, dependendo da gravidade do alerta. Isso permite que as equipes de TI possam responder de maneira rápida e eficiente a incidentes.

**Diferença entre Alerta e Notificações**

Alerta são os eventos gerados automaticamente pelo sistema de monitoramento quando detectam condições anormais ou falhas. Eles são uma maneira de identificar que algo precisa de atenção.

Notificações são as mensagens que comunicam esses alertas para os responsáveis, para que possam tomar as devidas providências. Elas são um meio de alertar as partes interessadas sobre o estado do sistema ou sobre algum problema.

Importância de Alerta e Notificações no Monitoramento de Software

- **Proatividade**: Permitem uma resposta rápida a problemas antes que eles afetem gravemente o sistema ou o serviço, ajudando a evitar downtime e a reduzir riscos.
- **Eficiência**: A automação desses processos evita que a equipe precise monitorar manualmente todos os aspectos do sistema, economizando tempo e recursos.
- **Melhora na Tomada de Decisão**: Com notificações detalhadas, as equipes podem agir com mais precisão, sabendo exatamente o que está ocorrendo no sistema.
- **Respostas Rápidas**: Em sistemas de alta criticidade, como bancos, e-commerce e plataformas de saúde, alerta e notificações são vitais para garantir que a operação continue funcionando sem interrupções, minimizando perdas e danos.