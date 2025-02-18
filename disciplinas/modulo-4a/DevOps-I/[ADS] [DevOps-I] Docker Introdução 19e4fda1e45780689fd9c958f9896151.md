# [ADS] [DevOps-I] Docker: Introdução

**Docker:**  código aberto que permite automatizar a implantação e o gerenciamento de aplicações em containers.

**Note:** "caixas" padronizadas onde você coloca tudo o que seu aplicativo precisa para rodar e essas caixas podem ser movidas entre diferentes ambientes. 

**Docker Hub:** repositório online de imagens Docker.

**Vantagens do Docker:**

- **Portabilidade:** os containers podem ser executados em qualquer ambiente que tenha o Docker instalado, seja Linux, Windows ou macOS;
- **Consistência:** os containers garantem que a aplicação rode sempre da mesma forma, independentemente do ambiente em que for executada;
- **Eficiência:** os containers são mais leves e consomem menos recursos do que máquinas virtuais, pois compartilham o kernel do sistema operacional do host;
- **Escalabilidade:** é fácil criar e destruir containers;
- **Agilidade:** acelera o processo de desenvolvimento e implantação de aplicações, permitindo que as equipes entreguem software mais rápido.

**Docker Local: q**uando você instala o Docker no seu próprio computador. Nesse caso, o Docker Engine (que executa os containers) e o Docker Client (que envia comandos para o Engine) estão rodando na sua máquina.

**Docker Remoto:**  envolve utilizar um servidor remoto para hospedar o Docker Engine. Nesse caso, o Docker Client está no seu computador, mas os containers são executados em um servidor separado, que pode ser uma máquina virtual na nuvem, um servidor físico dedicado ou outro computador na sua rede.