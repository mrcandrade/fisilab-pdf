# Modificações do FisiLab PDF em relação ao PdfCraft

O **FisiLab PDF** (nome curto **PDF**) é a versão do [PdfCraft](https://github.com/storytold/pdfcraft) (versão 0.8.0)
disponibilizada pelo **FisiLab — LARFIC · UFRGS** em parceria com o **CTA — Centro de Tecnologia
Acadêmica**, como parte da Suíte FisiLab, para democratizar o acesso à tecnologia entre os alunos da UFRGS.
O código continua sob a licença MIT ou Apache 2.0, à escolha de quem o usa. Este arquivo lista o
que foi alterado e por quê.

## 1. Nome e marca

- O nome exibido passou de **PdfCraft** para **PDF** em todo lugar que a pessoa vê: título e
  ícone da janela, tela inicial, menus ("Sobre o PDF"), diálogos, avisos, mensagens de erro,
  janela "Sobre", texto de ajuda e versão da linha de comando, descrições do servidor MCP e os
  catálogos de tradução (`crates/ui-egui/src/i18n/*.tsv`, chaves e traduções juntas).
- O nome **ArtCraft**, o logotipo e o símbolo (`docs/brand/`) são marcas registradas que a licença
  do projeto não cobre e que versões modificadas devem remover (veja o NOTICE original). Foram
  apagados `docs/brand/`, a captura de tela `docs/images/pdfcraft-tools.png` (que mostrava o
  símbolo), as funções que os desenhavam na interface e as entradas correspondentes em
  `ATTRIBUTION.toml`. No lugar do símbolo aparece o ícone do PDF.
- O crédito em texto ao PdfCraft e ao ArtCraft Team foi mantido e ampliado (janela Sobre, tela
  inicial, NOTICE, README).

## 2. Identidade visual da Suíte FisiLab

- Novo crate `crates/fisilab-brand` (compartilhado pelos apps da suíte): paleta, tela de abertura
  com a logo do CTA e a janela "Sobre a Suíte FisiLab".
- **Paleta original do PdfCraft**, sem cores próprias da suíte. Os dois temas do PdfCraft ganharam
  os nomes **"Noite"** (escuro) e **"Dia"** (claro, o padrão), listados em *Exibir → Tema de
  exibição* e em Preferências → Aparência (`crates/ui-egui/src/theme.rs`). O nome do tema
  aceita também `noite`/`dia` (e `night`/`day`) em `--theme` e `ui.set`.
- "Usar configuração do sistema" alterna entre Noite e Dia conforme o sistema.
- O português do Brasil é o idioma da primeira abertura (definido em `apps/pdfcraft/src/main.rs`
  só quando não há preferência salva).
- Tela de abertura (cerca de 3 s, pode ser pulada com um clique ou tecla; `FISILAB_NO_SPLASH=1`
  a desliga), chamada em `apps/pdfcraft/src/main.rs`.
- Novo item **Ajuda → Sobre a Suíte FisiLab…** (comando `help.fisilab`) e um botão "Suíte
  FisiLab" na barra de abas, no lugar do botão do Discord. A janela "Sobre o PDF" original,
  que credita os contribuidores do PdfCraft, foi mantida.
- Ícone do aplicativo: `assets/fisilab/fisilab-pdf.ico` no executável e `fisilab-pdf-256.png` na janela.

## 3. Links da comunidade e serviços externos

- Os links "Join the ArtCraft Discord", "PdfCraft web page" e "ArtCraft website"
  (`crates/engine/src/links.rs`, menu Ajuda, tela inicial, janela Sobre e `--version` da CLI)
  foram trocados por: **Site da Suíte FisiLab**, **Manifesto pelas tecnologias livres**,
  **Projeto original (PdfCraft)** e **Reportar um problema**
  (`https://github.com/mrcandrade/fisilab-pdf/issues`).
- **Verificar atualizações** (só quando a pessoa pede; não há verificação automática) consulta as
  versões de `github.com/mrcandrade/fisilab-pdf`, não mais as do PdfCraft.
- O PdfCraft não tem telemetria nem envio de relatórios de erro; nada foi encontrado que
  contatasse a infraestrutura do ArtCraft além dos links acima. Os modelos de OCR continuam sendo
  baixados, apenas na hora de empacotar (`cargo xtask models`), do servidor público do projeto ocrs.

## 4. Executáveis, identificadores e pastas

- Executáveis: `fisilab-pdf.exe` (aplicativo) e `fisilab-pdf-cli.exe` (linha de comando), por `[[bin]]`
  nos crates `apps/pdfcraft` e `apps/pdfcraft-cli`; os nomes dos pacotes Rust não mudaram.
- Recursos do Windows (VERSIONINFO): ProductName PDF, descrição, empresa FisiLab — LARFIC/UFRGS
  e direitos autorais com crédito ao PdfCraft.
- Identificador do aplicativo `br.ufrgs.fisilab.pdf` (antes `ai.storyteller.pdfcraft`).
- Pastas próprias, para não colidir com uma instalação do PdfCraft nem com outro programa (nunca
  uma pasta chamada só "PDF"): configurações em `%APPDATA%\FisiLab PDF\data` (macOS
  `~/Library/Application Support/FisiLab PDF`, Linux `~/.local/share/fisilab-pdf`); recuperação,
  área de transferência e canal de instância única em `%LOCALAPPDATA%\FisiLab PDF` (macOS
  `~/Library/Application Support/FisiLab PDF/Recovery` e `~/Library/Caches/FisiLab PDF/Clipboard`,
  Linux `~/.local/share/fisilab-pdf/recovery` e `~/.cache/fisilab-pdf/clipboard`); modo portátil em
  `FisiLabPDFData` (marcadores `portable.txt` ou `FisiLabPDF.portable`). A migração automática das
  pastas do antigo nome PrintCraft foi removida.
- Metadados gravados em PDFs novos (Producer, autor padrão de comentários, nome do aplicativo
  nas assinaturas) dizem FisiLab PDF.
- Onde o nome curto "PDF" seria confundido com o formato de arquivo (avisos, mensagens de erro,
  atualização disponível, ferramentas e instruções do servidor MCP, User-Agent), o programa é
  chamado de **FisiLab PDF**; título da janela, tela inicial, Sobre e menus continuam "PDF".

## 5. Idioma

- Uma instalação nova abre em **português do Brasil** (`pdfcraft_ui_egui::DEFAULT_LANGUAGE`); a
  escolha salva em Preferências (outro idioma ou "Automático") continua valendo.
- Os rótulos novos da suíte (links, temas, "Sobre a Suíte FisiLab…") estão em português em todos
  os catálogos.

## 6. Outros

- Removida a pasta `.github/` (integração contínua, lançamentos e modelos do projeto original).
- `NOTICE` ganhou um cabeçalho do FisiLab PDF; o texto original foi mantido abaixo dele.
- O `README.md` original virou `README.upstream.md`; o novo `README.md` está em português.
- `packaging/windows/portable.txt` foi traduzido e aponta para as novas pastas.
- O arquivo `.desktop` do Linux passou a se chamar `br.ufrgs.fisilab.pdf.desktop`.

## Segurança e privacidade

- *Ajuda › Verificar atualizações* agora consulta o `manifest.json` da última versão da Suíte
  (`https://github.com/mrcandrade/larfic/releases/latest/download/manifest.json`), em vez de uma
  página de versões do `fisilab-pdf` que não publica os instaladores. Lê só a entrada `fisilab-pdf`,
  compara com a versão em execução e oferece apenas `https://github.com/mrcandrade/larfic/releases/latest`
  para baixar (nada vindo da resposta é aberto). Continua só sob ação explícita do usuário, somente
  HTTPS (inclusive nos redirecionamentos), com limite de 10 s e de tamanho da resposta.

## O que não mudou

- Todo o motor de PDF, a renderização, a edição, os formatos de arquivo e as ferramentas.
- Nomes de crates, módulos e identificadores internos (`pdfcraft_*`, `PdfCraftApp`), variáveis
  de ambiente (`PDFCRAFT_*`), URIs `pdfcraft://` do MCP e marcadores gravados dentro dos PDFs
  (por exemplo `%PdfCraft`), para manter a compatibilidade.
- Comentários de código, a documentação em `docs/`, o `ROADMAP.md`, `AGENTS.md`, `CLAUDE.md` e o
  histórico do projeto original.
- `LICENSE-MIT`, `LICENSE-APACHE` e `ATTRIBUTION.md` (este é gerado a partir de `ATTRIBUTION.toml`
  e não foi regenerado).
- Os scripts de empacotamento de macOS, Linux e FreeBSD e o MSI do Windows
  (`packaging/windows/*.wxs`, `package.ps1`) continuam com os nomes do PdfCraft; a suíte usa o seu
  próprio instalador.
- As ferramentas de desenvolvimento em `xtask/` (capturas de tela, PDF de demonstração).
