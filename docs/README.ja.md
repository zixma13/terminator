# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *レトロ90年代SFターミナルAI — レトロターミナルインターフェースで本物のAIと会話*
> *Gemma 4 E2B搭載。100%オフライン。Macネイティブ。*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > こんにちは、人間。入力をお待ちしています。        ║
║                                                  ║
║  [SPACE] 話す  [ENTER] 送信  [ESC] 終了           ║
╚══════════════════════════════════════════════════╝
```

## 機能

- 🧠 **Gemma 4 E2B** — マルチモーダルAIブレイン（テキスト＋音声＋画像）、140以上の言語対応
- 🎤 **音声入力** — プッシュトゥトーク、ライブオシロスコープ波形表示
- 🔊 **音声出力** — MMS-TTS多言語音声合成（英語、タイ語など）
- 👁️ **ビジョン** — Gemmaビジョンエンコーダーでファイルシステム上の画像を分析
- 🔧 **エージェントツール** — ファイルを開く、読む、ディレクトリ一覧、コマンド実行
- ⚠️ **セキュリティ承認** — すべてのツール操作にポップアップでユーザー承認が必要
- 🖥️ **レトロterminal UI** — ブートシーケンス付きネオングリーンターミナル、Ratatui搭載
- 🔒 **100%オフライン** — クラウドなし、APIキー不要、データは外部に送信されません
- 🍎 **Macネイティブ** — Apple Silicon（M1+）に最適化

## アーキテクチャ

```
terminator (Rustバイナリ)
├── ratatui       → レトロターミナルUI
├── cpal          → マイク入力キャプチャ（PCM 16kHz mono）
└── Pythonブリッジ (subprocess, JSONプロトコル)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (テキスト＋音声＋画像、1モデル)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-thaなど)
```

### データフロー

```
🎤 音声 → cpal → Gemma 4 (文字起こし) → Gemma 4 (思考/ツール) → MMS-TTS 🔊
⌨️ テキスト →                            Gemma 4 (思考/ツール) → MMS-TTS 🔊
                                              ↓
                                    [ツール呼び出し検出]
                                              ↓
                                    ⚠ 警告ポップアップ [Y/N]
                                              ↓
                                    実行 (開く/読む/一覧/実行/ビジョン)
                                              ↓
                                    結果 → Gemma 4 → 応答
```

## 必要条件

- Apple Silicon（M1+）搭載のmacOS
- Rust 1.75+
- Python 3.11+
- ~5GBディスク（モデルウェイト）
- ~4GB RAM（推論）

## クイックスタート

```bash
# 1. クローン
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. Python依存関係のインストール
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. モデルのダウンロード（初回のみ、~5GB）
python3 scripts/download_model.py

# 4. ビルド＆実行
cargo build --release
./target/release/terminator
```

## 操作方法

| キー      | アクション                            |
|-----------|---------------------------------------|
| `Enter`   | 入力したメッセージを送信                |
| `Tab`     | 音声/テキストモードの切り替え           |
| `Space`   | タップで録音開始/停止（音声モード）      |
| `Y`       | ツール操作を承認                       |
| `N`       | ツール操作を拒否                       |
| `Esc`     | 終了                                  |

## ツール（エージェント）

TERMINATORはツール呼び出しでシステムと対話できます。すべての操作は警告ポップアップによる明示的な承認が必要です。

| ツール            | 説明                                  |
|-------------------|---------------------------------------|
| `open_file`       | デフォルトアプリでファイル/フォルダを開く |
| `read_file`       | テキストファイルの内容を読み取って返す    |
| `list_directory`  | ディレクトリ内のファイル一覧を表示       |
| `run_command`     | シェルコマンドを実行                    |
| `analyze_image`   | Gemma 4ビジョンで画像を分析            |

例: *「ダウンロードのファイルを一覧表示」* → 承認ポップアップ → `ls`実行 → AIが結果を要約

## プロジェクト構成

```
terminator/
├── Cargo.toml              # Rust依存関係
├── requirements.txt        # Python依存関係
├── src/
│   ├── main.rs             # エントリーポイント、イベントループ
│   ├── app.rs              # ステートマシン、ツール実行
│   ├── ui.rs               # Ratatui TUIレンダリング、承認ポップアップ
│   ├── audio.rs            # cpalによるマイクキャプチャ、リサンプリング
│   ├── bridge.rs           # Pythonサブプロセスブリッジ（JSONプロトコル）
│   └── theme.rs            # CRT/ネオンビジュアルテーマ
├── scripts/
│   ├── download_model.py   # モデルダウンローダー
│   └── inference.py        # Gemma 4推論＋関数呼び出し＋TTS
├── tests/
│   ├── test_bridge.rs      # ブリッジプロトコルテスト
│   └── test_audio.rs       # オーディオパイプラインテスト
└── docs/
    └── SDLC.md             # 完全なSDLCドキュメント
```

## リソース使用量

Apple M5 Pro（48GB）で計測:

| コンポーネント     | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4.2 GB  |
| MMS-TTS           | burst   | ~145 MB  |
| **合計**          |         | **~4.3 GB** |

*\*CPUは推論中のみスパイク、アイドル時は低負荷。GPU（Metal）でアクセラレーション。*

## モデル

| モデル | 役割 | サイズ | 言語 |
|--------|------|--------|------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | ブレイン（テキスト＋音声＋画像） | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | 音声出力 | ~145 MB/言語 | 1100+ |

## ライセンス

Apache 2.0
