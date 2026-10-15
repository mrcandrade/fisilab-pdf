# Janela do DMG (macOS)

Dois arquivos prontos, copiados por `package.sh` para dentro da imagem; nada aqui é gerado na compilação.

| Arquivo | O que é |
|---|---|
| `background.tiff` | Fundo da janela do Finder (1x e 2x em um TIFF HiDPI). Vai para `.background/background.tiff`. |
| `dmg-layout.DS_Store` | Ajustes de visualização do Finder (tamanho da janela, posição dos ícones, fundo). Vai para `.DS_Store` na imagem. |

Ambos são gerados por `suite-fisilab/scripts/gerar_mac.py`, no repositório
[mrcandrade/larfic](https://github.com/mrcandrade/larfic). Não edite à mão: gere de novo por lá.

O layout espera estes nomes exatos:

- volume: `PDF` (sem versão; o Finder localiza o fundo pelo nome do volume);
- aplicativo: `PDF.app`;
- atalho para a pasta de aplicativos: `Aplicativos`.
