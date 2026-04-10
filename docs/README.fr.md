# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *Une IA terminal rétro sci-fi des années 90 — discutez avec une vraie IA via une interface CRT néon.*
> *Propulsé par Gemma 4 E2B. 100% hors ligne. Natif Mac.*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > Bonjour, humain. Je suis prêt pour vos        ║
║    instructions.                                 ║
║                                                  ║
║  [SPACE] parler  [ENTER] envoyer  [ESC] quitter  ║
╚══════════════════════════════════════════════════╝
```

## Fonctionnalités

- 🧠 **Gemma 4 E2B** — cerveau IA multimodal (texte + audio + vision), 140+ langues
- 🎤 **Entrée vocale** — appuyez pour parler avec affichage oscilloscope en direct
- 🔊 **Sortie vocale** — synthèse vocale multilingue MMS-TTS (anglais, thaï, etc.)
- 👁️ **Vision** — analyse d'images sur votre système de fichiers via l'encodeur vision de Gemma
- 🔧 **Outils agentiques** — ouvrir des fichiers, lire des fichiers, lister des répertoires, exécuter des commandes
- ⚠️ **Approbation de sécurité** — chaque action d'outil nécessite l'approbation explicite de l'utilisateur via popup
- 🖥️ **Interface terminale rétro** — terminal néon vert avec séquence de démarrage, propulsé par Ratatui
- 🔒 **100% hors ligne** — pas de cloud, pas de clés API, aucune donnée ne quitte votre machine
- 🍎 **Natif Mac** — optimisé pour Apple Silicon (M1+)

## Architecture

```
terminator (binaire Rust)
├── ratatui       → interface terminal CRT rétro
├── cpal          → capture microphone (PCM 16kHz mono)
└── Pont Python (subprocess, protocole JSON)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (texte + audio + vision, 1 modèle)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha, etc.)
```

### Flux de données

```
🎤 Voix → cpal → Gemma 4 (transcrire) → Gemma 4 (réfléchir/outils) → MMS-TTS 🔊
⌨️ Texte →                               Gemma 4 (réfléchir/outils) → MMS-TTS 🔊
                                              ↓
                                    [appel d'outil détecté]
                                              ↓
                                    ⚠ Popup d'AVERTISSEMENT [Y/N]
                                              ↓
                                    Exécuter (ouvrir/lire/lister/exécuter/vision)
                                              ↓
                                    Résultat → Gemma 4 → réponse
```

## Prérequis

- macOS sur Apple Silicon (M1+)
- Rust 1.75+
- Python 3.11+
- ~5 Go disque (poids du modèle)
- ~4 Go RAM (inférence)

## Démarrage rapide

```bash
# 1. Cloner
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Installer les dépendances Python
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. Télécharger le modèle (première exécution uniquement, ~5 Go)
python3 scripts/download_model.py

# 4. Compiler et exécuter
cargo build --release
./target/release/terminator
```

## Contrôles

| Touche    | Action                                |
|-----------|---------------------------------------|
| `Enter`   | Envoyer le message tapé               |
| `Tab`     | Basculer mode voix/texte              |
| `Space`   | Appuyer pour démarrer/arrêter l'enregistrement (voix) |
| `Y`       | Approuver l'action de l'outil         |
| `N`       | Rejeter l'action de l'outil           |
| `Esc`     | Quitter                               |

## Outils (Agentiques)

TERMINATOR peut interagir avec votre système via des appels d'outils. Chaque action nécessite une approbation explicite via un popup d'avertissement.

| Outil             | Description                          |
|-------------------|--------------------------------------|
| `open_file`       | Ouvrir fichier/dossier dans l'app par défaut |
| `read_file`       | Lire et retourner le contenu d'un fichier texte |
| `list_directory`  | Lister les fichiers d'un répertoire  |
| `run_command`     | Exécuter une commande shell          |
| `analyze_image`   | Analyser une image avec Gemma 4 vision |

Exemple : *« Lister les fichiers dans mes Téléchargements »* → popup d'approbation → exécute `ls` → l'IA résume les résultats.

## Structure du projet

```
terminator/
├── Cargo.toml              # Dépendances Rust
├── requirements.txt        # Dépendances Python
├── src/
│   ├── main.rs             # Point d'entrée, boucle d'événements
│   ├── app.rs              # Machine à états, exécution d'outils
│   ├── ui.rs               # Rendu TUI Ratatui, popup d'approbation
│   ├── audio.rs            # Capture micro via cpal, rééchantillonnage
│   ├── bridge.rs           # Pont subprocess Python (protocole JSON)
│   └── theme.rs            # Thème visuel CRT/néon
├── scripts/
│   ├── download_model.py   # Téléchargeur de modèle
│   └── inference.py        # Inférence Gemma 4 + appels de fonctions + TTS
├── tests/
│   ├── test_bridge.rs      # Tests du protocole pont
│   └── test_audio.rs       # Tests du pipeline audio
└── docs/
    └── SDLC.md             # Documentation SDLC complète
```

## Utilisation des ressources

Mesuré sur Apple M5 Pro (48 Go) :

| Composant         | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 Mo   |
| Python/Gemma 4    | ~78%*   | ~4,2 Go  |
| MMS-TTS           | burst   | ~145 Mo  |
| **Total**         |         | **~4,3 Go** |

*\*Le CPU monte en charge uniquement pendant l'inférence, inactif sinon. GPU (Metal) utilisé pour l'accélération.*

## Modèles

| Modèle | Rôle | Taille | Langues |
|--------|------|--------|---------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | Cerveau (texte + audio + vision) | ~5 Go | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | Sortie vocale | ~145 Mo/langue | 1100+ |

## Licence

Apache 2.0
