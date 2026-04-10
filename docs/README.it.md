# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *Un'IA terminale retro sci-fi anni '90 — parla con una vera IA attraverso un'interfaccia CRT al neon.*
> *Alimentato da Gemma 4 E2B. 100% offline. Nativo per Mac.*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > Ciao, umano. Sono pronto per il tuo input.    ║
║                                                  ║
║  [SPACE] parlare  [ENTER] inviare  [ESC] uscire  ║
╚══════════════════════════════════════════════════╝
```

## Funzionalità

- 🧠 **Gemma 4 E2B** — cervello IA multimodale (testo + audio + visione), 140+ lingue
- 🎤 **Input vocale** — premi per parlare con forma d'onda oscilloscopio in tempo reale
- 🔊 **Output vocale** — sintesi vocale multilingue MMS-TTS (inglese, thailandese, ecc.)
- 👁️ **Visione** — analizza immagini nel tuo filesystem tramite l'encoder visivo di Gemma
- 🔧 **Strumenti agentici** — aprire file, leggere file, elencare directory, eseguire comandi
- ⚠️ **Approvazione di sicurezza** — ogni azione richiede l'approvazione esplicita dell'utente tramite popup
- 🖥️ **Interfaccia terminale retro** — terminale verde neon con sequenza di avvio, alimentato da Ratatui
- 🔒 **100% offline** — nessun cloud, nessuna chiave API, nessun dato lascia la tua macchina
- 🍎 **Nativo per Mac** — ottimizzato per Apple Silicon (M1+)

## Architettura

```
terminator (binario Rust)
├── ratatui       → interfaccia terminale CRT retro
├── cpal          → cattura microfono (PCM 16kHz mono)
└── Ponte Python (subprocess, protocollo JSON)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (testo + audio + visione, 1 modello)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha, ecc.)
```

### Flusso dati

```
🎤 Voce → cpal → Gemma 4 (trascrivere) → Gemma 4 (pensare/strumenti) → MMS-TTS 🔊
⌨️ Testo →                                Gemma 4 (pensare/strumenti) → MMS-TTS 🔊
                                              ↓
                                    [chiamata strumento rilevata]
                                              ↓
                                    ⚠ Popup di AVVISO [Y/N]
                                              ↓
                                    Eseguire (aprire/leggere/elencare/eseguire/visione)
                                              ↓
                                    Risultato → Gemma 4 → risposta
```

## Requisiti

- macOS su Apple Silicon (M1+)
- Rust 1.75+
- Python 3.11+
- ~5 GB disco (pesi del modello)
- ~4 GB RAM (inferenza)

## Avvio rapido

```bash
# 1. Clonare
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Installare le dipendenze Python
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. Scaricare il modello (solo la prima volta, ~5 GB)
python3 scripts/download_model.py

# 4. Compilare ed eseguire
cargo build --release
./target/release/terminator
```

## Controlli

| Tasto     | Azione                                |
|-----------|---------------------------------------|
| `Enter`   | Inviare il messaggio digitato         |
| `Tab`     | Alternare modalità voce/testo        |
| `Space`   | Toccare per avviare/fermare la registrazione (voce) |
| `Y`       | Approvare l'azione dello strumento    |
| `N`       | Rifiutare l'azione dello strumento    |
| `Esc`     | Uscire                                |

## Strumenti (Agentici)

TERMINATOR può interagire con il tuo sistema tramite chiamate a strumenti. Ogni azione richiede un'approvazione esplicita tramite popup di avviso.

| Strumento         | Descrizione                          |
|-------------------|--------------------------------------|
| `open_file`       | Aprire file/cartella nell'app predefinita |
| `read_file`       | Leggere e restituire il contenuto di un file di testo |
| `list_directory`  | Elencare i file in una directory     |
| `run_command`     | Eseguire un comando shell            |
| `analyze_image`   | Analizzare un'immagine con Gemma 4 vision |

Esempio: *"Elenca i file nei miei Download"* → popup di approvazione → esegue `ls` → l'IA riassume i risultati.

## Struttura del progetto

```
terminator/
├── Cargo.toml              # Dipendenze Rust
├── requirements.txt        # Dipendenze Python
├── src/
│   ├── main.rs             # Punto di ingresso, ciclo eventi
│   ├── app.rs              # Macchina a stati, esecuzione strumenti
│   ├── ui.rs               # Rendering TUI Ratatui, popup di approvazione
│   ├── audio.rs            # Cattura microfono via cpal, ricampionamento
│   ├── bridge.rs           # Ponte subprocess Python (protocollo JSON)
│   └── theme.rs            # Tema visivo CRT/neon
├── scripts/
│   ├── download_model.py   # Scaricatore di modelli
│   └── inference.py        # Inferenza Gemma 4 + chiamate a funzioni + TTS
├── tests/
│   ├── test_bridge.rs      # Test del protocollo ponte
│   └── test_audio.rs       # Test della pipeline audio
└── docs/
    └── SDLC.md             # Documentazione SDLC completa
```

## Utilizzo risorse

Misurato su Apple M5 Pro (48 GB):

| Componente        | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4,2 GB  |
| MMS-TTS           | burst   | ~145 MB  |
| **Totale**        |         | **~4,3 GB** |

*\*La CPU sale solo durante l'inferenza, inattiva altrimenti. GPU (Metal) usata per l'accelerazione.*

## Modelli

| Modello | Ruolo | Dimensione | Lingue |
|---------|-------|------------|--------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | Cervello (testo + audio + visione) | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | Output vocale | ~145 MB/lingua | 1100+ |

## Licenza

Apache 2.0
