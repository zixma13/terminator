# Review Notes

## Consistency Check ✅

- Model variants consistent across `bridge.rs` MODELS, `download_model.py` MODELS, `inference.py` resolve_model(), and README
- Bridge protocol types match between all docs and source code
- Tool names and parameters consistent across components and interfaces docs
- State machine documented correctly including new Loading progress behavior

## Completeness Check

### Well-Documented ✅
- Multi-model selection flow (picker → bridge → Python)
- Download status detection (ready / downloading / not started)
- All 5 tools with path handling gotchas
- State machine with all 7 states
- Bridge protocol both directions
- Boot sequence with fake progress bar

### Gaps ⚠️

| Area | Gap | Severity |
|------|-----|----------|
| Error recovery | What happens when Python crashes mid-conversation | Low |
| TTS language mapping | Which language codes map to which MMS-TTS models | Low |
| Model comparison | No benchmarks or quality comparison between E2B/E4B/26B-A4B/31B | Low |
| Testing | Only 2 test files; no integration tests for model picker or multi-model flow | Medium |

## Recommendations

1. Add integration tests for the bridge with mock Python process
2. Document TTS language detection logic and supported languages
3. Consider adding model quality/speed benchmarks to README
