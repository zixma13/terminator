# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *Uma IA de terminal retrô sci-fi dos anos 90 — converse com uma IA real através de uma interface CRT neon.*
> *Alimentado por Gemma 4 E2B. 100% offline. Nativo para Mac.*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > Olá, humano. Estou pronto para sua entrada.   ║
║                                                  ║
║  [SPACE] falar  [ENTER] enviar  [ESC] sair       ║
╚══════════════════════════════════════════════════╝
```

## Funcionalidades

- 🧠 **Gemma 4 E2B** — cérebro IA multimodal (texto + áudio + visão), 140+ idiomas
- 🎤 **Entrada de voz** — pressione para falar com forma de onda de osciloscópio ao vivo
- 🔊 **Saída de voz** — síntese de fala multilíngue MMS-TTS (inglês, tailandês, etc.)
- 👁️ **Visão** — analise imagens no seu sistema de arquivos via codificador de visão do Gemma
- 🔧 **Ferramentas agênticas** — abrir arquivos, ler arquivos, listar diretórios, executar comandos
- ⚠️ **Aprovação de segurança** — cada ação de ferramenta requer aprovação explícita do usuário via popup
- 🖥️ **Interface CRT retrô** — terminal verde neon com sequência de boot, alimentado por Ratatui
- 🔒 **100% offline** — sem nuvem, sem chaves de API, nenhum dado sai da sua máquina
- 🍎 **Nativo para Mac** — otimizado para Apple Silicon (M1+)

## Arquitetura

```
terminator (binário Rust)
├── ratatui       → interface terminal CRT retrô
├── cpal          → captura de microfone (PCM 16kHz mono)
└── Ponte Python (subprocess, protocolo JSON)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (texto + áudio + visão, 1 modelo)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha, etc.)
```

### Fluxo de dados

```
🎤 Voz → cpal → Gemma 4 (transcrever) → Gemma 4 (pensar/ferramentas) → MMS-TTS 🔊
⌨️ Texto →                               Gemma 4 (pensar/ferramentas) → MMS-TTS 🔊
                                              ↓
                                    [chamada de ferramenta detectada]
                                              ↓
                                    ⚠ Popup de AVISO [Y/N]
                                              ↓
                                    Executar (abrir/ler/listar/executar/visão)
                                              ↓
                                    Resultado → Gemma 4 → resposta
```

## Requisitos

- macOS em Apple Silicon (M1+)
- Rust 1.75+
- Python 3.11+
- ~5GB de disco (pesos do modelo)
- ~4GB de RAM (inferência)

## Início rápido

```bash
# 1. Clonar
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Instalar dependências Python
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. Baixar modelo (apenas na primeira execução, ~5GB)
python3 scripts/download_model.py

# 4. Compilar e executar
cargo build --release
./target/release/terminator
```

## Controles

| Tecla     | Ação                                  |
|-----------|---------------------------------------|
| `Enter`   | Enviar mensagem digitada              |
| `Tab`     | Alternar modo voz/texto              |
| `Space`   | Tocar para iniciar/parar gravação (voz) |
| `Y`       | Aprovar ação da ferramenta            |
| `N`       | Rejeitar ação da ferramenta           |
| `Esc`     | Sair                                  |

## Ferramentas (Agênticas)

TERMINATOR pode interagir com seu sistema via chamadas de ferramentas. Cada ação requer aprovação explícita através de um popup de aviso.

| Ferramenta        | Descrição                            |
|-------------------|--------------------------------------|
| `open_file`       | Abrir arquivo/pasta no app padrão    |
| `read_file`       | Ler e retornar conteúdo de arquivo texto |
| `list_directory`  | Listar arquivos em um diretório      |
| `run_command`     | Executar um comando shell            |
| `analyze_image`   | Analisar imagem com Gemma 4 vision   |

Exemplo: *"Listar arquivos nos meus Downloads"* → popup de aprovação → executa `ls` → IA resume os resultados.

## Estrutura do projeto

```
terminator/
├── Cargo.toml              # Dependências Rust
├── requirements.txt        # Dependências Python
├── src/
│   ├── main.rs             # Ponto de entrada, loop de eventos
│   ├── app.rs              # Máquina de estados, execução de ferramentas
│   ├── ui.rs               # Renderização TUI Ratatui, popup de aprovação
│   ├── audio.rs            # Captura de microfone via cpal, reamostragem
│   ├── bridge.rs           # Ponte subprocess Python (protocolo JSON)
│   └── theme.rs            # Tema visual CRT/neon
├── scripts/
│   ├── download_model.py   # Baixador de modelos
│   └── inference.py        # Inferência Gemma 4 + chamadas de função + TTS
├── tests/
│   ├── test_bridge.rs      # Testes do protocolo ponte
│   └── test_audio.rs       # Testes do pipeline de áudio
└── docs/
    └── SDLC.md             # Documentação SDLC completa
```

## Uso de recursos

Medido no Apple M5 Pro (48GB):

| Componente        | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4,2 GB  |
| MMS-TTS           | burst   | ~145 MB  |
| **Total**         |         | **~4,3 GB** |

*\*CPU sobe apenas durante a inferência, ocioso caso contrário. GPU (Metal) usado para aceleração.*

## Modelos

| Modelo | Função | Tamanho | Idiomas |
|--------|--------|---------|---------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | Cérebro (texto + áudio + visão) | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | Saída de voz | ~145 MB/idioma | 1100+ |

## Licença

Apache 2.0
