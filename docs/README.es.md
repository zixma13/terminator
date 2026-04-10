# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *Una IA de terminal retro sci-fi de los 90 — habla con una IA real a través de una interfaz CRT de neón.*
> *Impulsado por Gemma 4 E2B. 100% sin conexión. Nativo de Mac.*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > Hola, humano. Estoy listo para tu entrada.    ║
║                                                  ║
║  [SPACE] hablar  [ENTER] enviar  [ESC] salir     ║
╚══════════════════════════════════════════════════╝
```

## Características

- 🧠 **Gemma 4 E2B** — cerebro IA multimodal (texto + audio + visión), 140+ idiomas
- 🎤 **Entrada de voz** — pulsa para hablar con forma de onda de osciloscopio en vivo
- 🔊 **Salida de voz** — síntesis de voz multilingüe MMS-TTS (inglés, tailandés, etc.)
- 👁️ **Visión** — analiza imágenes en tu sistema de archivos con el codificador de visión de Gemma
- 🔧 **Herramientas agénticas** — abrir archivos, leer archivos, listar directorios, ejecutar comandos
- ⚠️ **Aprobación de seguridad** — cada acción de herramienta requiere aprobación explícita del usuario mediante popup
- 🖥️ **Interfaz terminal retro** — terminal verde neón con secuencia de arranque, impulsado por Ratatui
- 🔒 **100% sin conexión** — sin nube, sin claves API, los datos no salen de tu máquina
- 🍎 **Nativo de Mac** — optimizado para Apple Silicon (M1+)

## Arquitectura

```
terminator (binario Rust)
├── ratatui       → interfaz terminal CRT retro
├── cpal          → captura de micrófono (PCM 16kHz mono)
└── Puente Python (subprocess, protocolo JSON)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (texto + audio + visión, 1 modelo)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha, etc.)
```

### Flujo de datos

```
🎤 Voz → cpal → Gemma 4 (transcribir) → Gemma 4 (pensar/herramientas) → MMS-TTS 🔊
⌨️ Texto →                               Gemma 4 (pensar/herramientas) → MMS-TTS 🔊
                                              ↓
                                    [llamada a herramienta detectada]
                                              ↓
                                    ⚠ Popup de ADVERTENCIA [Y/N]
                                              ↓
                                    Ejecutar (abrir/leer/listar/ejecutar/visión)
                                              ↓
                                    Resultado → Gemma 4 → respuesta
```

## Requisitos

- macOS en Apple Silicon (M1+)
- Rust 1.75+
- Python 3.11+
- ~5GB de disco (pesos del modelo)
- ~4GB de RAM (inferencia)

## Inicio rápido

```bash
# 1. Clonar
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Instalar dependencias de Python
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. Descargar modelo (solo la primera vez, ~5GB)
python3 scripts/download_model.py

# 4. Compilar y ejecutar
cargo build --release
./target/release/terminator
```

## Controles

| Tecla     | Acción                                |
|-----------|---------------------------------------|
| `Enter`   | Enviar mensaje escrito                |
| `Tab`     | Alternar modo voz/texto              |
| `Space`   | Tocar para iniciar/detener grabación (voz) |
| `Y`       | Aprobar acción de herramienta         |
| `N`       | Rechazar acción de herramienta        |
| `Esc`     | Salir                                 |

## Herramientas (Agénticas)

TERMINATOR puede interactuar con tu sistema mediante llamadas a herramientas. Cada acción requiere aprobación explícita a través de un popup de advertencia.

| Herramienta       | Descripción                          |
|-------------------|--------------------------------------|
| `open_file`       | Abrir archivo/carpeta en la app predeterminada |
| `read_file`       | Leer y devolver contenido de archivo de texto |
| `list_directory`  | Listar archivos en un directorio     |
| `run_command`     | Ejecutar un comando de shell         |
| `analyze_image`   | Analizar imagen con Gemma 4 vision   |

Ejemplo: *"Listar archivos en mis Descargas"* → popup de aprobación → ejecuta `ls` → la IA resume los resultados.

## Estructura del proyecto

```
terminator/
├── Cargo.toml              # Dependencias Rust
├── requirements.txt        # Dependencias Python
├── src/
│   ├── main.rs             # Punto de entrada, bucle de eventos
│   ├── app.rs              # Máquina de estados, ejecución de herramientas
│   ├── ui.rs               # Renderizado TUI Ratatui, popup de aprobación
│   ├── audio.rs            # Captura de micrófono via cpal, remuestreo
│   ├── bridge.rs           # Puente subprocess Python (protocolo JSON)
│   └── theme.rs            # Tema visual CRT/neón
├── scripts/
│   ├── download_model.py   # Descargador de modelos
│   └── inference.py        # Inferencia Gemma 4 + llamadas a funciones + TTS
├── tests/
│   ├── test_bridge.rs      # Tests del protocolo puente
│   └── test_audio.rs       # Tests del pipeline de audio
└── docs/
    └── SDLC.md             # Documentación SDLC completa
```

## Uso de recursos

Medido en Apple M5 Pro (48GB):

| Componente        | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4,2 GB  |
| MMS-TTS           | burst   | ~145 MB  |
| **Total**         |         | **~4,3 GB** |

*\*El CPU solo se dispara durante la inferencia, inactivo en otros momentos. GPU (Metal) usado para aceleración.*

## Modelos

| Modelo | Rol | Tamaño | Idiomas |
|--------|-----|--------|---------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | Cerebro (texto + audio + visión) | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | Salida de voz | ~145 MB/idioma | 1100+ |

## Licencia

Apache 2.0
