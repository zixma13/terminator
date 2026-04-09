# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *레트로 90년대 SF 터미널 AI — 네온 CRT 인터페이스로 진짜 AI와 대화*
> *Gemma 4 E2B 기반. 100% 오프라인. Mac 네이티브.*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > 안녕하세요, 인간. 입력을 기다리고 있습니다.      ║
║                                                  ║
║  [SPACE] 말하기  [ENTER] 전송  [ESC] 종료         ║
╚══════════════════════════════════════════════════╝
```

## 기능

- 🧠 **Gemma 4 E2B** — 멀티모달 AI 브레인 (텍스트 + 오디오 + 비전), 140개 이상 언어 지원
- 🎤 **음성 입력** — 푸시투톡, 실시간 오실로스코프 파형 표시
- 🔊 **음성 출력** — MMS-TTS 다국어 음성 합성 (영어, 태국어 등)
- 👁️ **비전** — Gemma 비전 인코더로 파일 시스템의 이미지 분석
- 🔧 **에이전트 도구** — 파일 열기, 읽기, 디렉토리 목록, 명령 실행
- ⚠️ **보안 승인** — 모든 도구 작업은 팝업을 통한 사용자 명시적 승인 필요
- 🖥️ **레트로 CRT UI** — 부팅 시퀀스가 있는 네온 그린 터미널, Ratatui 기반
- 🔒 **100% 오프라인** — 클라우드 없음, API 키 불필요, 데이터가 기기를 떠나지 않음
- 🍎 **Mac 네이티브** — Apple Silicon (M1+) 최적화

## 아키텍처

```
terminator (Rust 바이너리)
├── ratatui       → 레트로 CRT 터미널 UI
├── cpal          → 마이크 캡처 (PCM 16kHz mono)
└── Python 브리지 (subprocess, JSON 프로토콜)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (텍스트 + 오디오 + 비전, 1개 모델)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha 등)
```

### 데이터 흐름

```
🎤 음성 → cpal → Gemma 4 (전사) → Gemma 4 (사고/도구) → MMS-TTS 🔊
⌨️ 텍스트 →                        Gemma 4 (사고/도구) → MMS-TTS 🔊
                                              ↓
                                    [도구 호출 감지]
                                              ↓
                                    ⚠ 경고 팝업 [Y/N]
                                              ↓
                                    실행 (열기/읽기/목록/실행/비전)
                                              ↓
                                    결과 → Gemma 4 → 응답
```

## 요구 사항

- Apple Silicon (M1+)의 macOS
- Rust 1.75+
- Python 3.11+
- ~5GB 디스크 (모델 가중치)
- ~4GB RAM (추론)

## 빠른 시작

```bash
# 1. 클론
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Python 의존성 설치
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. 모델 다운로드 (최초 실행 시에만, ~5GB)
python3 scripts/download_model.py

# 4. 빌드 및 실행
cargo build --release
./target/release/terminator
```

## 조작법

| 키        | 동작                                  |
|-----------|---------------------------------------|
| `Enter`   | 입력한 메시지 전송                      |
| `Tab`     | 음성/텍스트 모드 전환                   |
| `Space`   | 탭하여 녹음 시작/중지 (음성 모드)        |
| `Y`       | 도구 작업 승인                         |
| `N`       | 도구 작업 거부                         |
| `Esc`     | 종료                                  |

## 도구 (에이전트)

TERMINATOR는 도구 호출을 통해 시스템과 상호작용할 수 있습니다. 모든 작업은 경고 팝업을 통한 명시적 승인이 필요합니다.

| 도구              | 설명                                  |
|-------------------|---------------------------------------|
| `open_file`       | 기본 앱으로 파일/폴더 열기              |
| `read_file`       | 텍스트 파일 내용 읽기 및 반환           |
| `list_directory`  | 디렉토리의 파일 목록 표시               |
| `run_command`     | 셸 명령 실행                           |
| `analyze_image`   | Gemma 4 비전으로 이미지 분석           |

예시: *"다운로드 폴더의 파일 목록"* → 승인 팝업 → `ls` 실행 → AI가 결과 요약

## 프로젝트 구조

```
terminator/
├── Cargo.toml              # Rust 의존성
├── requirements.txt        # Python 의존성
├── src/
│   ├── main.rs             # 진입점, 이벤트 루프
│   ├── app.rs              # 상태 머신, 도구 실행
│   ├── ui.rs               # Ratatui TUI 렌더링, 승인 팝업
│   ├── audio.rs            # cpal 마이크 캡처, 리샘플링
│   ├── bridge.rs           # Python 서브프로세스 브리지 (JSON 프로토콜)
│   └── theme.rs            # CRT/네온 비주얼 테마
├── scripts/
│   ├── download_model.py   # 모델 다운로더
│   └── inference.py        # Gemma 4 추론 + 함수 호출 + TTS
├── tests/
│   ├── test_bridge.rs      # 브리지 프로토콜 테스트
│   └── test_audio.rs       # 오디오 파이프라인 테스트
└── docs/
    └── SDLC.md             # 전체 SDLC 문서
```

## 리소스 사용량

Apple M5 Pro (48GB)에서 측정:

| 컴포넌트          | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4.2 GB  |
| MMS-TTS           | burst   | ~145 MB  |
| **합계**          |         | **~4.3 GB** |

*\*CPU는 추론 중에만 급증, 유휴 시 저부하. GPU (Metal)로 가속.*

## 모델

| 모델 | 역할 | 크기 | 언어 |
|------|------|------|------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | 브레인 (텍스트 + 오디오 + 비전) | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | 음성 출력 | ~145 MB/언어 | 1100+ |

## 라이선스

Apache 2.0
