## REVIEW REPORT — P5-T2 Native ACL Evaluation

**Scope:** `origin/main..HEAD`
**Files:** 7

### Summary Verdict

- [x] Approve with comments
- [ ] Approve
- [ ] Request changes
- [ ] Block

### Critical Issues

None found in the task-owned implementation. Native FFI remains macOS-gated and errors are fail-closed.

### Secondary Issues

- [Medium] Repository line coverage is 72.23%, below the 80% workflow default. This is not introduced as a regression claim for this task and is tracked by P5-T12 (`SPECS/Workplan.md`).
- [Low] Native syscall behavior cannot be simulated on Linux; the current macOS runner compiled and exercised the acquisition path, while parser fixtures provide deterministic grammar coverage.

### Architectural Notes

- `src/acl.rs` now accepts native `acl_to_text` entry ordering and qualifiers, but unknown permissions/flags remain incomplete rather than being silently ignored.
- A null `acl_get_file` result is treated as no ACL only for the platform no-ACL errno or the observed no-ACL `ENOENT` on an existing path; an absent target remains `Unknown`.
- ACL ownership and policy composition remain separate concerns for P5-T3/P5-T4.

### Tests

- `cargo test --all-targets --all-features` — PASS.
- `cargo clippy --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all --check` — PASS.
- Coverage command — 72.23% total lines, below 80% default; follow-up P5-T12.
- `git diff --check` — PASS after removing report whitespace.

### Next Steps

1. Continue with P5-T3, which depends on reliable ACL evaluation and covers complete anchored ancestor inspection.
2. Keep P5-T12 open for repository-wide coverage enforcement and CI reproducibility.

No new task is required from this review beyond the already tracked P5-T12 coverage follow-up.
