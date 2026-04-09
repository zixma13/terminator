# TERMINATOR 🤖

🌍 **README translations:**
[🇬🇧 English](../README.md) · [🇹🇭 ไทย](README.th.md) · [🇯🇵 日本語](README.ja.md) · [🇨🇳 中文](README.zh.md) · [🇫🇷 Français](README.fr.md) · [🇪🇸 Español](README.es.md) · [🇰🇷 한국어](README.ko.md) · [🇩🇪 Deutsch](README.de.md) · [🇵🇹 Português](README.pt.md) · [🇷🇺 Русский](README.ru.md) · [🇮🇹 Italiano](README.it.md) · [🇮🇳 हिन्दी](README.hi.md) · [🇸🇦 العربية](README.ar.md)

> *AI เทอร์มินัลสไตล์ไซไฟยุค 90 — คุยกับ AI จริงผ่านหน้าจอ CRT นีออน*
> *ขับเคลื่อนโดย Gemma 4 E2B ออฟไลน์ 100% รองรับ Mac โดยเฉพาะ*

```
╔══════════════════════════════════════════════════╗
║  TERMINATOR OS v1.0.0                            ║
║  NEURAL CORE: GEMMA-4-E2B .............. ONLINE  ║
║  AUDIO SENSOR: ......................... ACTIVE   ║
║  LANGUAGES: 140+ ....................... READY    ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  > สวัสดี มนุษย์ ฉันพร้อมรับคำสั่งแล้ว            ║
║                                                  ║
║  [SPACE] พูด  [ENTER] ส่ง  [ESC] ออก             ║
╚══════════════════════════════════════════════════╝
```

## คุณสมบัติ

- 🧠 **Gemma 4 E2B** — สมอง AI มัลติโมดัล (ข้อความ + เสียง + ภาพ) รองรับ 140+ ภาษา
- 🎤 **อินพุตเสียง** — กดค้างเพื่อพูด พร้อมแสดงคลื่นเสียงแบบออสซิลโลสโคป
- 🔊 **เอาต์พุตเสียง** — สังเคราะห์เสียงพูดหลายภาษาด้วย MMS-TTS (อังกฤษ, ไทย ฯลฯ)
- 👁️ **วิชัน** — วิเคราะห์ภาพในระบบไฟล์ผ่าน Gemma vision encoder
- 🔧 **เครื่องมือเอเจนต์** — เปิดไฟล์, อ่านไฟล์, แสดงรายการไดเรกทอรี, รันคำสั่ง
- ⚠️ **การอนุมัติความปลอดภัย** — ทุกการกระทำต้องได้รับการอนุมัติจากผู้ใช้ผ่านป๊อปอัป
- 🖥️ **UI แบบ CRT ย้อนยุค** — เทอร์มินัลนีออนเขียวพร้อมลำดับบูต ขับเคลื่อนโดย Ratatui
- 🔒 **ออฟไลน์ 100%** — ไม่มีคลาวด์ ไม่ต้องใช้ API key ข้อมูลไม่ออกจากเครื่อง
- 🍎 **Mac native** — ปรับแต่งสำหรับ Apple Silicon (M1+)

## สถาปัตยกรรม

```
terminator (Rust binary)
├── ratatui       → UI เทอร์มินัล CRT ย้อนยุค
├── cpal          → จับเสียงไมโครโฟน (PCM 16kHz mono)
└── Python bridge (subprocess, JSON protocol)
    ├── HuggingFace Transformers
    │   └── Gemma 4 E2B (ข้อความ + เสียง + ภาพ, 1 โมเดล)
    └── MMS-TTS (facebook/mms-tts-eng, mms-tts-tha ฯลฯ)
```

### การไหลของข้อมูล

```
🎤 เสียง → cpal → Gemma 4 (ถอดเสียง) → Gemma 4 (คิด/เครื่องมือ) → MMS-TTS 🔊
⌨️ ข้อความ →                             Gemma 4 (คิด/เครื่องมือ) → MMS-TTS 🔊
                                              ↓
                                    [ตรวจพบการเรียกเครื่องมือ]
                                              ↓
                                    ⚠ ป๊อปอัปเตือน [Y/N]
                                              ↓
                                    ดำเนินการ (เปิด/อ่าน/แสดง/รัน/วิชัน)
                                              ↓
                                    ผลลัพธ์ → Gemma 4 → คำตอบ
```

## ความต้องการระบบ

- macOS บน Apple Silicon (M1+)
- Rust 1.75+
- Python 3.11+
- ~5GB พื้นที่ดิสก์ (น้ำหนักโมเดล)
- ~4GB RAM (การอนุมาน)

## เริ่มต้นอย่างรวดเร็ว

```bash
# 1. โคลน
git clone https://github.com/zixma13/terminator.git
cd terminator

# 2. ติดตั้ง Python dependencies
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 3. ดาวน์โหลดโมเดล (ครั้งแรกเท่านั้น ~5GB)
python3 scripts/download_model.py

# 4. บิลด์และรัน
cargo build --release
./target/release/terminator
```

## การควบคุม

| ปุ่ม      | การกระทำ                              |
|-----------|---------------------------------------|
| `Enter`   | ส่งข้อความที่พิมพ์                      |
| `Tab`     | สลับโหมดเสียง/ข้อความ                  |
| `Space`   | แตะเพื่อเริ่ม/หยุดบันทึกเสียง (โหมดเสียง) |
| `Y`       | อนุมัติการกระทำของเครื่องมือ             |
| `N`       | ปฏิเสธการกระทำของเครื่องมือ             |
| `Esc`     | ออก                                   |

## เครื่องมือ (เอเจนต์)

TERMINATOR สามารถโต้ตอบกับระบบของคุณผ่านการเรียกเครื่องมือ ทุกการกระทำต้องได้รับการอนุมัติผ่านป๊อปอัปเตือน

| เครื่องมือ        | คำอธิบาย                              |
|-------------------|---------------------------------------|
| `open_file`       | เปิดไฟล์/โฟลเดอร์ในแอปเริ่มต้น         |
| `read_file`       | อ่านและส่งคืนเนื้อหาไฟล์ข้อความ        |
| `list_directory`  | แสดงรายการไฟล์ในไดเรกทอรี             |
| `run_command`     | รันคำสั่งเชลล์                         |
| `analyze_image`   | วิเคราะห์ภาพด้วย Gemma 4 vision       |

ตัวอย่าง: *"แสดงไฟล์ใน Downloads"* → ป๊อปอัปอนุมัติ → รัน `ls` → AI สรุปผลลัพธ์

## โครงสร้างโปรเจกต์

```
terminator/
├── Cargo.toml              # Rust dependencies
├── requirements.txt        # Python dependencies
├── src/
│   ├── main.rs             # จุดเริ่มต้น, event loop
│   ├── app.rs              # State machine, การดำเนินการเครื่องมือ
│   ├── ui.rs               # Ratatui TUI rendering, ป๊อปอัปอนุมัติ
│   ├── audio.rs            # จับเสียงไมค์ผ่าน cpal, resampling
│   ├── bridge.rs           # Python subprocess bridge (JSON protocol)
│   └── theme.rs            # ธีม CRT/นีออน
├── scripts/
│   ├── download_model.py   # ตัวดาวน์โหลดโมเดล
│   └── inference.py        # Gemma 4 inference + function calling + TTS
├── tests/
│   ├── test_bridge.rs      # ทดสอบ bridge protocol
│   └── test_audio.rs       # ทดสอบ audio pipeline
└── docs/
    └── SDLC.md             # เอกสาร SDLC ฉบับเต็ม
```

## การใช้ทรัพยากร

วัดบน Apple M5 Pro (48GB):

| คอมโพเนนต์       | CPU     | RAM      |
|------------------|---------|----------|
| Rust TUI         | < 1%    | ~22 MB   |
| Python/Gemma 4   | ~78%*   | ~4.2 GB  |
| MMS-TTS          | burst   | ~145 MB  |
| **รวม**          |         | **~4.3 GB** |

*\*CPU พุ่งเฉพาะระหว่างการอนุมาน ไม่ทำงานเมื่อว่าง GPU (Metal) ใช้สำหรับเร่งความเร็ว*

## โมเดล

| โมเดล | บทบาท | ขนาด | ภาษา |
|-------|--------|------|------|
| [Gemma 4 E2B](https://huggingface.co/google/gemma-4-E2B-it) | สมอง (ข้อความ + เสียง + ภาพ) | ~5 GB | 140+ |
| [MMS-TTS](https://huggingface.co/facebook/mms-tts-eng) | เอาต์พุตเสียง | ~145 MB/ภาษา | 1100+ |

## สัญญาอนุญาต

Apache 2.0
