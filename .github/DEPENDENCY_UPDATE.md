# Dependency Update - October 25, 2025

## Summary

Successfully updated all dependencies to their latest versions and adapted the code to work with the new APIs.

## Updated Dependencies

| Package | Old Version | New Version | Change |
|---------|-------------|-------------|--------|
| tokio | 1.41 | 1.48 | Minor update - async runtime improvements |
| tokio-tungstenite | 0.24 | 0.28 | Minor update - WebSocket library |
| rustyline | 14.0 | 17.0 | Major update - REPL/readline library |
| colored | 2.1 | 3.0 | Major update - Terminal coloring |

Other dependencies (serde, serde_json, clap, anyhow, url, dirs, futures-util) were already at their latest compatible versions.

## Code Changes Required

### 1. rustyline API Changes (v14 → v17)

**File:** `src/repl.rs`

The `highlight_char` method signature changed in rustyline 17.0:
- **Old:** `fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool`
- **New:** `fn highlight_char(&self, _line: &str, _pos: usize, _forced: CmdKind) -> bool`

**Changes Made:**
- Added `CmdKind` to imports: `use rustyline::highlight::{Highlighter, CmdKind};`
- Updated method signature to accept `CmdKind` instead of `bool`

### 2. tokio-tungstenite API Changes (v0.24 → v0.28)

**File:** `src/moonraker.rs`

The `Message::Text` variant now uses `Utf8Bytes` instead of `String` for better performance:
- **Old:** `Message::Text(String)`
- **New:** `Message::Text(Utf8Bytes)`

**Changes Made:**
- Line 51: Added `.to_string()` when forwarding messages: `read_tx.send(text.to_string())`
- Line 108: Added `.into()` when creating text messages: `Message::Text(message.to_string().into())`

### 3. colored API Changes (v2.1 → v3.0)

No code changes required - the API remained backward compatible for our use case.

## Verification

All changes have been verified:

✅ **Build:** `cargo build --release` - Success  
✅ **Tests:** `cargo test` - All passing  
✅ **Clippy:** `cargo clippy --release -- -D warnings` - No warnings  
✅ **Binary:** `./target/release/moonriver --help` and `--version` - Working correctly  

## Benefits of Updates

1. **tokio 1.48:** Performance improvements, bug fixes, better async runtime stability
2. **tokio-tungstenite 0.28:** More efficient WebSocket handling with `Utf8Bytes`
3. **rustyline 17.0:** Better REPL functionality, improved command history and editing
4. **colored 3.0:** More efficient terminal color handling

## Breaking Changes Handled

The updates included breaking API changes in:
- **rustyline:** Method signature change in `Highlighter` trait
- **tokio-tungstenite:** Type change for text messages

All breaking changes have been addressed with minimal code modifications that maintain the same functionality.

## Next Steps

1. Test the updated binary with a real Moonraker instance to ensure WebSocket communication works correctly
2. Consider creating a test release to verify all platforms build correctly with the new dependencies
3. Monitor for any user-reported issues after release

## Notes

- The `Utf8Bytes` type in tokio-tungstenite 0.28 is a more efficient representation of UTF-8 text that can be zero-copy in many cases
- The `CmdKind` parameter in rustyline 17.0 provides more context about command highlighting, though we continue to return `true` for all cases as before
- All changes are backward compatible in terms of functionality - users will not notice any difference in behavior
