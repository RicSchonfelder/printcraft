# PrintCraft — documentação em português do Brasil (pt-BR)

**O workbench de PDF; uma reimplementação open-source e clean-room do Adobe Acrobat, reconstruída em Rust puro.**

Feito em Rust puro, funciona nativamente em macOS, Windows e Linux e também no navegador via WebAssembly.

## Recursos

- Zoom profundo nítido: páginas grandes renderizam em tiles, texto legível em qualquer ampliação.
- Feito para sobreviver a arquivos ruins: cada página renderiza isolada e documentos danificados são reparados (0 crashes em 983 arquivos do corpus pdf.js).
- Layouts para cada tarefa: contínuo, página única, duas páginas, rotação, tela cheia e modo Leitura.
- Botão do Discord na barra de título: discord.gg/artcraft.

## Português do Brasil

Selecione o idioma no menu (a tela inicial permanece em inglês por design do upstream; menus e diálogos seguem o idioma escolhido).

PR #165 (tradução pt-BR) aberto no upstream (repositório renomeado para `storytold/pdfcraft`).

## A suíte ArtCraft

A ArtCraft é um conjunto de 7 aplicativos open-source que reimplementam, de forma clean-room e em Rust puro, as ferramentas de criação da Adobe — nativos para macOS, Windows e Linux, com a mesma interface no navegador via WebAssembly:

| Aplicativo | Propósito | Reimplementação de |
|---|---|---|
| PhotoCraft | Edição de imagens | Adobe Photoshop |
| FilmCraft | Edição de vídeo, cor e som | Adobe Premiere Pro |
| LightCraft | Biblioteca de fotos e revelação RAW | Adobe Lightroom |
| EffectCraft | Motion graphics e efeitos visuais | Adobe After Effects |
| PrintCraft | Workbench de PDF | Adobe Acrobat |
| DesignCraft | Layout de página e publicação | Adobe InDesign |
| VectorCraft | Ilustração vetorial | Adobe Illustrator |

- Site: <https://getartcraft.com> · Discord: <https://discord.gg/artcraft>

## Este fork

Adiciona **leitura desta documentação em português do Brasil** e, no código, a **tradução pt-BR da interface** — sem alterar nada do comportamento do aplicativo original.


## Instalar no Linux (x86_64)

Baixe o tarball da release e extraia (sem precisar de sudo):

```bash
wget https://github.com/storytold/pdfcraft/releases/download/v0.2.1/printcraft-0.2.1-linux-x86_64.tar.gz
mkdir -p ~/Programas/printcraft
tar -xzf printcraft-0.2.1-linux-x86_64.tar.gz -C ~/Programas/printcraft --strip-components=1
~/Programas/printcraft/bin/printcraft
```

> Consulte a página de releases do repositório upstream para a versão e o nome do asset atuais.

## Compilar do código

```bash
git clone https://github.com/storytold/<repositorio-upstream>.git
cd <repositorio-upstream>
# (opcional, para as fontes CJK do release) export CRAFT_FONTS_DIR=~/craft-fonts CRAFT_FONTS_REQUIRED=1
cargo build --release
```

## Comunidade

Suporte, feedback e novidades da suíte no Discord: <https://discord.gg/artcraft>.

---

Documentação original (inglês): [`README.md`](README.md).

