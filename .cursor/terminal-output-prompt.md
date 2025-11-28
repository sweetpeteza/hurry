# Terminal Output Guidelines for AI Agents

## Core Principle
**Always show command output to users.** They need visibility into what's happening.

## Quick Rules

### When to Show Full Output
- `git status`, `git log`, `git diff` - Always full
- Short commands (< 20 lines output) - Show all
- Error messages - Always show complete errors

### When to Tail Output  
- `cargo build` → `cargo build 2>&1 | tail -n 30`
- `cargo test` → `cargo test 2>&1 | tail -n 50`
- `cargo clippy` → `cargo clippy 2>&1 | tail -n 40`
- Long compilation/build output → tail last 20-50 lines

### Why It Matters
- Users track progress and catch issues early
- Terminal feedback maintains user context
- Errors/warnings are immediately visible
- Users see what passed/failed in tests

## Examples

✅ **Good:**
```bash
cargo test 2>&1 | tail -n 50
```
Shows test summary, pass/fail counts, timing

✅ **Good:**
```bash
git status
```
Full output (always short and relevant)

❌ **Bad:**
```bash
cargo test > /dev/null
```
Hides all output from user

❌ **Bad:**
```bash
cargo build
```
No tail - potentially thousands of lines

## Quick Reference
- Build: tail -n 30
- Test: tail -n 50  
- Lint: tail -n 40
- Git: full output
- Short commands: full output
