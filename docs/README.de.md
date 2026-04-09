# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *Eine Retro-90er-Sci-Fi-Terminal-KI — sprich mit einer echten KI über eine Neon-CRT-Oberfläche.*
> *Angetrieben von Gemma 4 E2B. 100% offline. Mac-nativ.*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > Hallo, Mensch. Ich bin bereit für deine       ║
║    Eingabe.                                      ║
║                                                  ║
║  [SPACE] sprechen [ENTER] senden [ESC] beenden   ║
╚══════════════════════════════════════════════════╝
```

## Funktionen

- 🧠 **Gemma 4 E2B** — multimodales KI-Gehirn (Text + Audio + Vision), 140+ Sprachen
- 🎤 **Spracheingabe** — Push-to-Talk mit Live-Oszilloskop-Wellenform
- 🔊 **Sprachausgabe** — MMS-TTS mehrsprachige Sprachsynthese (Englisch, Thai usw.)
- 👁️ **Vision** — Bilder im Dateisystem über Gemmas Vision-Encoder analysieren
- 🔧 **Agentische Werkzeuge** — Dateien öffnen, lesen, Verzeichnisse auflisten, Befehle ausführen
- ⚠️ **Sicherheitsgenehmigung** — jede Werkzeugaktion erfordert explizite Benutzergenehmigung per Popup
- 🖥️ **Retro-CRT-UI** — neongrünes Terminal mit Boot-Sequenz, angetrieben von Ratatui
- 🔒 **100% offline** — keine Cloud, keine API-Schlüssel, keine Daten verlassen deinen Rechner
- 🍎 **Mac-nativ** — optimiert für Apple Silicon (M1+)

## Architektur

```
terminator (Rust-Binary)
├── ratatui       → Retro-CRT-Terminal-UI
├── cpal          → Mikrofonaufnahme (PCM 16kHz Mono)
└── Python-Brücke (Subprocess, JSON-Protokoll)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (Text + Audio + Vision, 1 Modell)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha usw.)
```

### Datenfluss

```
🎤 Sprache → cpal → Gemma 4 (transkribieren) → Gemma 4 (denken/Werkzeuge) → MMS-TTS 🔊
⌨️ Text →                                      Gemma 4 (denken/Werkzeuge) → MMS-TTS 🔊
                                              ↓
                                    [Werkzeugaufruf erkannt]
                                              ↓
                                    ⚠ WARNUNG-Popup [Y/N]
                                              ↓
                                    Ausführen (öffnen/lesen/auflisten/ausführen/Vision)
                                              ↓
                                    Ergebnis → Gemma 4 → Antwort
```

## Voraussetzungen

- macOS auf Apple Silicon (M1+)
- Rust 1.75+
- Python 3.11+
- ~5 GB Festplatte (Modellgewichte)
- ~4 GB RAM (Inferenz)

## Schnellstart

```bash
# 1. Klonen
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Python-Abhängigkeiten installieren
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. Modell herunterladen (nur beim ersten Start, ~5 GB)
python3 scripts/download_model.py

# 4. Bauen und ausführen
cargo build --release
./target/release/terminator
```

## Steuerung

| Taste     | Aktion                                |
|-----------|---------------------------------------|
| `Enter`   | Eingegebene Nachricht senden          |
| `Tab`     | Sprach-/Textmodus umschalten          |
| `Space`   | Tippen zum Starten/Stoppen der Aufnahme (Sprache) |
| `Y`       | Werkzeugaktion genehmigen             |
| `N`       | Werkzeugaktion ablehnen               |
| `Esc`     | Beenden                               |

## Werkzeuge (Agentisch)

TERMINATOR kann über Werkzeugaufrufe mit deinem System interagieren. Jede Aktion erfordert eine explizite Genehmigung über ein Warn-Popup.

| Werkzeug          | Beschreibung                         |
|-------------------|--------------------------------------|
| `open_file`       | Datei/Ordner in Standard-App öffnen  |
| `read_file`       | Textdateiinhalt lesen und zurückgeben |
| `list_directory`  | Dateien in einem Verzeichnis auflisten |
| `run_command`     | Shell-Befehl ausführen               |
| `analyze_image`   | Bild mit Gemma 4 Vision analysieren  |

Beispiel: *„Dateien in meinen Downloads auflisten"* → Genehmigungs-Popup → führt `ls` aus → KI fasst Ergebnisse zusammen.

## Projektstruktur

```
terminator/
├── Cargo.toml              # Rust-Abhängigkeiten
├── requirements.txt        # Python-Abhängigkeiten
├── src/
│   ├── main.rs             # Einstiegspunkt, Ereignisschleife
│   ├── app.rs              # Zustandsmaschine, Werkzeugausführung
│   ├── ui.rs               # Ratatui-TUI-Rendering, Genehmigungs-Popup
│   ├── audio.rs            # Mikrofonaufnahme via cpal, Resampling
│   ├── bridge.rs           # Python-Subprocess-Brücke (JSON-Protokoll)
│   └── theme.rs            # CRT/Neon-visuelles Thema
├── scripts/
│   ├── download_model.py   # Modell-Downloader
│   └── inference.py        # Gemma 4 Inferenz + Funktionsaufrufe + TTS
├── tests/
│   ├── test_bridge.rs      # Brückenprotokoll-Tests
│   └── test_audio.rs       # Audio-Pipeline-Tests
└── docs/
    └── SDLC.md             # Vollständige SDLC-Dokumentation
```

## Ressourcenverbrauch

Gemessen auf Apple M5 Pro (48 GB):

| Komponente        | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4,2 GB  |
| MMS-TTS           | Burst   | ~145 MB  |
| **Gesamt**        |         | **~4,3 GB** |

*\*CPU steigt nur während der Inferenz, sonst im Leerlauf. GPU (Metal) wird zur Beschleunigung verwendet.*

## Modelle

| Modell | Rolle | Größe | Sprachen |
|--------|-------|-------|----------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | Gehirn (Text + Audio + Vision) | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | Sprachausgabe | ~145 MB/Sprache | 1100+ |

## Lizenz

Apache 2.0
