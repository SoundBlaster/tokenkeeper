## REVIEW REPORT — Tokenkeeper v0.1.0 Release

**Scope:** `origin/main..HEAD` plus the audited `v0.1.0` tag and current `main` behavior  
**Files:** 6 branch files; production source and tests reviewed as audit evidence  
**Primary artifact:** `docs/v0.1.0-post-release-review.md`

### Summary Verdict

- [ ] Approve
- [ ] Approve with comments
- [ ] Request changes
- [x] Block

The audit documentation is complete and internally consistent, but the `v0.1.0` release cannot receive acceptance closure. Two native macOS scenarios produce unsafe false `PASS`, ACL acquisition errors can also become clean state, and the published tag does not identify the current reviewed source tree.

### Critical Issues

- **[Blocker] TK-REV-001 — ancestor ACL false `PASS`:** target-only ACL evaluation misses replacement-capable rights on parent directories. Implement complete component observations and native ancestor ACL tests.
- **[Blocker] TK-REV-002 — credential policy false `PASS`:** Codex and AWS configs classified as credential-bearing are evaluated only as `ExecutableConfig`, allowing foreign read. Apply credential confidentiality and introduce composable policy requirements.
- **[High] TK-REV-003/TK-REV-004 — macOS ACL backend:** native ACL text is not parsed, while `acl_get_file()` errors are treated as absence. Replace synthetic grammar assumptions and preserve every backend failure as incomplete.
- **[High] TK-REV-005 — report contract:** stable rule IDs, severity, current/expected state, risk, manual guidance, unknown reasons, and full checked scope are missing.
- **[High] TK-REV-006/TK-REV-007 — trusted scope and traversal completeness:** `$HOME` is authoritative and depth-limited discovery returns partial success.
- **[High] TK-REV-008/TK-REV-009 — release/distribution:** `v0.1.0` is not an ancestor of `main`, identical versions represent different source trees, and complete clean Homebrew lifecycle evidence is absent from CI.

### Secondary Issues

- **[Medium] TK-REV-010/TK-REV-011:** semantic-root symlinks may be followed before diagnosis and entry limits do not bound enumeration memory.
- **[Medium] TK-REV-012/TK-REV-013:** CLI selector combinations, platform selection, availability output, XDG behavior, and Linux per-result completeness are ambiguous or hardcoded.
- **[Medium] TK-REV-014/TK-REV-015:** per-location evidence, exact built-in fixtures, runtime registry validation, native security integration, access-denied, Home/root, ESC-byte, no-read, and golden CLI tests are incomplete.
- **[Medium] TK-REV-016/TK-REV-017:** coverage is below threshold, CI lacks release gates, and terminal sanitization does not cover all user-controlled CLI errors.
- **[Low] TK-REV-018:** Cargo metadata, PRD status/open questions, optional-scope semantics, and authoritative release metadata require reconciliation.

### Architectural Notes

- A single `Policy` cannot express independent confidentiality and integrity requirements.
- A mutually exclusive `InspectionResult` cannot retain a known finding together with incomplete ACL coverage.
- Path-string reinspection separates resolver checks from metadata/ACL evaluation and weakens component anchoring.
- Per-profile evidence cannot represent location-specific platform, version, source, and availability state.
- Release identity must be derived from one reviewed main-line commit before tag, archive, and Formula publication.

### Tests

- Configured FLOW gates pass: 39 tests, Clippy with `-D warnings`, and format check.
- Native reproductions confirm target ACL `UNKNOWN`, ancestor ACL false `PASS`, credential-policy false `PASS`, and arbitrary `$HOME` clean exit.
- Coverage completed with rustup stable: 71.08% lines, below the 80% REVIEW threshold.
- Homebrew style, audit-by-name, and minimal formula test pass; clean source reinstall, upgrade, and uninstall were not run.
- Missing tests and release gates are enumerated in the primary artifact and must become tracked follow-up work.

### Next Steps

1. Run FOLLOW-UP and create atomic Phase 5 tasks for every audit finding.
2. Execute false-`PASS` fixes before reporting improvements or release work.
3. Add native/macOS and golden integration gates before successor release validation.
4. Keep `v0.1.0` immutable; publish a new version only after the complete acceptance matrix passes from reviewed `main` lineage.
