# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *复古90年代科幻终端AI — 通过霓虹CRT界面与真正的AI对话*
> *由Gemma 4 E2B驱动。100%离线。Mac原生。*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > 你好，人类。我已准备好接收你的输入。             ║
║                                                  ║
║  [SPACE] 说话  [ENTER] 发送  [ESC] 退出           ║
╚══════════════════════════════════════════════════╝
```

## 功能

- 🧠 **Gemma 4 E2B** — 多模态AI大脑（文本+音频+视觉），支持140+种语言
- 🎤 **语音输入** — 按住说话，实时示波器波形显示
- 🔊 **语音输出** — MMS-TTS多语言语音合成（英语、泰语等）
- 👁️ **视觉** — 通过Gemma视觉编码器分析文件系统中的图像
- 🔧 **代理工具** — 打开文件、读取文件、列出目录、运行命令
- ⚠️ **安全审批** — 每个工具操作都需要通过弹窗获得用户明确批准
- 🖥️ **复古CRT界面** — 霓虹绿终端带启动序列，由Ratatui驱动
- 🔒 **100%离线** — 无云端、无API密钥、数据不离开你的设备
- 🍎 **Mac原生** — 针对Apple Silicon（M1+）优化

## 架构

```
terminator (Rust二进制文件)
├── ratatui       → 复古CRT终端UI
├── cpal          → 麦克风采集（PCM 16kHz mono）
└── Python桥接 (subprocess, JSON协议)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (文本+音频+视觉，1个模型)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha等)
```

### 数据流

```
🎤 语音 → cpal → Gemma 4 (转录) → Gemma 4 (思考/工具) → MMS-TTS 🔊
⌨️ 文本 →                          Gemma 4 (思考/工具) → MMS-TTS 🔊
                                              ↓
                                    [检测到工具调用]
                                              ↓
                                    ⚠ 警告弹窗 [Y/N]
                                              ↓
                                    执行 (打开/读取/列出/运行/视觉)
                                              ↓
                                    结果 → Gemma 4 → 响应
```

## 系统要求

- 搭载Apple Silicon（M1+）的macOS
- Rust 1.75+
- Python 3.11+
- ~5GB磁盘空间（模型权重）
- ~4GB内存（推理）

## 快速开始

```bash
# 1. 克隆
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. 安装Python依赖
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. 下载模型（仅首次运行，~5GB）
python3 scripts/download_model.py

# 4. 构建并运行
cargo build --release
./target/release/terminator
```

## 控制

| 按键      | 操作                                |
|-----------|-------------------------------------|
| `Enter`   | 发送输入的消息                       |
| `Tab`     | 切换语音/文本模式                    |
| `Space`   | 点击开始/停止录音（语音模式）         |
| `Y`       | 批准工具操作                         |
| `N`       | 拒绝工具操作                         |
| `Esc`     | 退出                                |

## 工具（代理）

TERMINATOR可以通过工具调用与你的系统交互。每个操作都需要通过警告弹窗获得明确批准。

| 工具              | 描述                                |
|-------------------|-------------------------------------|
| `open_file`       | 用默认应用打开文件/文件夹            |
| `read_file`       | 读取并返回文本文件内容               |
| `list_directory`  | 列出目录中的文件                     |
| `run_command`     | 执行shell命令                       |
| `analyze_image`   | 使用Gemma 4视觉分析图像             |

示例：*"列出我的下载文件夹"* → 审批弹窗 → 执行`ls` → AI总结结果

## 项目结构

```
terminator/
├── Cargo.toml              # Rust依赖
├── requirements.txt        # Python依赖
├── src/
│   ├── main.rs             # 入口点，事件循环
│   ├── app.rs              # 状态机，工具执行
│   ├── ui.rs               # Ratatui TUI渲染，审批弹窗
│   ├── audio.rs            # cpal麦克风采集，重采样
│   ├── bridge.rs           # Python子进程桥接（JSON协议）
│   └── theme.rs            # CRT/霓虹视觉主题
├── scripts/
│   ├── download_model.py   # 模型下载器
│   └── inference.py        # Gemma 4推理+函数调用+TTS
├── tests/
│   ├── test_bridge.rs      # 桥接协议测试
│   └── test_audio.rs       # 音频管道测试
└── docs/
    └── SDLC.md             # 完整SDLC文档
```

## 资源使用

在Apple M5 Pro（48GB）上测量：

| 组件              | CPU     | RAM      |
|-------------------|---------|----------|
| Rust TUI          | < 1%    | ~22 MB   |
| Python/Gemma 4    | ~78%*   | ~4.2 GB  |
| MMS-TTS           | burst   | ~145 MB  |
| **总计**          |         | **~4.3 GB** |

*\*CPU仅在推理期间飙升，空闲时低负载。GPU（Metal）用于加速。*

## 模型

| 模型 | 角色 | 大小 | 语言 |
|------|------|------|------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | 大脑（文本+音频+视觉） | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | 语音输出 | ~145 MB/语言 | 1100+ |

## 许可证

Apache 2.0
