# Review Notes

## Consistency Check ✅

- All documentation files reference the same state machine states consistently
- Bridge protocol types match between `interfaces.md`, `architecture.md`, and source code
- Tool names and parameters are consistent across `components.md` and `interfaces.md`
- No conflicting information found across documents

## Completeness Check

### Well-Documented Areas ✅
- State machine transitions and all 7 states
- Bridge JSON protocol (both directions)
- Tool calling interface and approval flow
- Audio capture pipeline
- All Rust and Python components
- Dependency stack

### Gaps Identified ⚠️

| Area | Gap | Severity |
|------|-----|----------|
| Error handling | No documentation of how Python errors propagate to UI beyond `Response::Error` | Low |
| Configuration | No config file or environment variables documented (hardcoded values like `MAX_RECORD_SECS`, truncation limits) | Low |
| TTS language support | `detect_lang()` logic not fully documented — unclear which languages trigger which MMS-TTS model | Low |
| Testing | Only 2 test files with basic tests; no integration test coverage documented | Medium |
| Vision workflow | `analyze_image` tool passes args to Python but the full round-trip (image loading, vision inference, response) is less documented than other tools | Low |
| Deployment | No CI/CD, Docker, or distribution documentation | Low (expected for a local-only app) |

## Recommendations

1. **Add integration tests** for the full Rust ↔ Python bridge flow (mock Python process)
2. **Document hardcoded constants** like `MAX_RECORD_SECS` (28s), truncation limits (2000 chars, 50 entries)
3. **Add error recovery documentation** — what happens when Python crashes mid-conversation
4. **Document TTS language mapping** — which language codes map to which MMS-TTS models
