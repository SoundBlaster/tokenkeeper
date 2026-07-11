# Tokenkeeper Workplan

Source requirements: [`SPECS/PRD.md`](PRD.md).

## Phase 1: Foundation

#### P1-T1: Bootstrap Rust Project ✅ Complete
- **Description:** Create the minimal Rust binary crate, module skeleton, local quality gates, and baseline CLI entry point.
- **Priority:** P0
- **Dependencies:** None
- **Parallelizable:** no
- **Acceptance Criteria:**
  - `Cargo.toml`, `Cargo.lock`, `src/`, and `tests/` exist
  - `tokenkeeper --help` runs without filesystem mutations
  - `cargo test --all-targets --all-features` passes
  - `cargo clippy --all-targets --all-features -- -D warnings` passes
  - `cargo fmt --all --check` passes

#### P1-T2: Research Agent and Integration Storage Locations ✅ Complete
- **Description:** Verify filesystem and Keychain storage behavior for Codex, Claude Code, OpenCode, Cursor, MCP servers, and credential-bearing utility configs. GitHub Copilot is optional.
- **Priority:** P0
- **Dependencies:** None
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - Every claimed location records product/version, platform, source, and verification date
  - File-based credentials and integrity-sensitive configs are classified separately
  - MCP server and utility configs that may contain credentials are identified and classified as `CredentialConfig`
  - Keychain-only or unknown storage is documented without a false coverage claim
  - No real credential content enters documentation or fixtures

#### P1-T3: Implement Profile Schema and Resolver ✅ Complete
- **Description:** Add the data-only embedded profile model, registry validation, semantic roots, and bounded path resolution.
- **Priority:** P0
- **Dependencies:** P1-T1, P1-T2
- **Parallelizable:** no
- **Acceptance Criteria:**
  - Auditing core contains no agent-specific branching
  - Invalid IDs, `..`, absolute profile paths, duplicate entries, and unbounded traversal are rejected
  - Resolution is anchored to the current user's trusted Home
  - Intermediate and target symlinks are never followed silently

## Phase 2: Audit Core

#### P2-T1: Implement Unix Metadata Policies ✅ Complete
- **Description:** Inspect node type, UID/GID, mode bits, ancestors, and symlinks without reading target contents.
- **Priority:** P0
- **Dependencies:** P1-T3
- **Parallelizable:** no
- **Acceptance Criteria:**
  - `SecretFile`, `CredentialConfig`, `PrivateDirectory`, `TrustedConfig`, and `ExecutableConfig` policies match the PRD matrix
  - Component-wise inspection stops on unexpected symlinks
  - Missing optional, missing required, access denied, and unsafe permissions remain distinct outcomes
  - Tests cover safe, unsafe, malformed, and incomplete filesystem states

#### P2-T2: Implement CLI Reporting and Guidance ✅ Complete
- **Description:** Add profile selection, custom path checks, stable findings, summaries, exit codes, and conservative remediation output.
- **Priority:** P0
- **Dependencies:** P2-T1
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - `check`, repeatable `--profile`, custom `--path/--policy`, and `profiles` follow the PRD contract
  - Results distinguish `PASS`, `FINDING`, `UNKNOWN`, and `SKIP`
  - Exit codes are `0` clean, `1` findings, and `2` incomplete/error
  - Spaces, quotes, control characters, and shell metacharacters cannot inject commands or terminal output
  - Tokenkeeper never executes remediation

#### P2-T3: Implement macOS ACL Backend ✅ Complete
- **Description:** Inspect macOS extended ACL and map effective or conservatively unknown non-owner access into policy results.
- **Priority:** P0
- **Dependencies:** P2-T1
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - Non-owner allow entries incompatible with policy produce a finding
  - Inherited and ambiguous allow/deny ordering never produce a false `PASS`
  - ACL read failures produce `UNKNOWN/INCOMPLETE`
  - Any required `unsafe`/FFI is isolated, documented, and covered by macOS integration tests

## Phase 3: Profiles and Hardening

#### P3-T1: Add Built-in Agent and Integration Profiles ✅ Complete
- **Description:** Add validated embedded profiles for the required agents, MCP integrations, and credential-bearing utility configs using the researched evidence.
- **Priority:** P0
- **Dependencies:** P1-T2, P1-T3, P2-T1, P2-T3
- **Parallelizable:** no
- **Acceptance Criteria:**
  - `tokenkeeper profiles` lists Codex, Claude Code, OpenCode, Cursor, and MCP/integration profiles; GitHub Copilot is optional
  - Every location has an appropriate policy and bounded selector
  - Known credential-bearing configs use `CredentialConfig` without reading their contents
  - Optional, version-dependent, Keychain, and unsupported storage are reported honestly
  - Profile fixtures contain no secrets

#### P3-T2: Complete Security Integration Tests ✅ Complete
- **Description:** Validate the full CLI against adversarial filesystem trees and all PRD acceptance scenarios.
- **Priority:** P0
- **Dependencies:** P2-T2, P2-T3, P3-T1
- **Parallelizable:** no
- **Acceptance Criteria:**
  - All `SPECS/PRD.md` acceptance scenarios pass on macOS
  - Tests cover symlink escape/loops, writable ancestors, ACL ambiguity, odd filenames, and traversal limits
  - Tests demonstrate no target-content reads, mutations, network access, or remediation subprocesses
  - Malformed input produces no panic

## Phase 4: Distribution

#### P4-T1: Publish User and Security Documentation ✅ Complete
- **Description:** Document installation, supported scope, output interpretation, limitations, and manual remediation workflow.
- **Priority:** P1
- **Dependencies:** P3-T2
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - README contains install and minimal usage examples
  - Security documentation explains same-UID, root, Keychain, backup/sync, and point-in-time limitations
  - Supported profile/version evidence is discoverable
  - Example output contains no real usernames or credentials

#### P4-T2: Add Homebrew Tap Distribution ✅ Complete
- **Description:** Create the versioned release contract and Homebrew formula for installation from a maintainer-owned tap.
- **Priority:** P1
- **Dependencies:** P4-T1
- **Parallelizable:** no
- **Acceptance Criteria:**
  - Project license, SemVer tag, immutable source archive, and SHA-256 checksum are published consistently
  - `brew install <owner>/homebrew-tap/tokenkeeper` succeeds from a clean environment
  - Formula builds from source and passes `brew audit` and `brew test`
  - Upgrade and uninstall behavior is validated on supported macOS architectures
  - Formula has no `post_install`, daemon setup, implicit Home scan, or credential mutation
  - README documents install, update, upgrade, and uninstall commands

#### P4-T3: Evaluate Linux Support ✅ Complete
- **Description:** Reuse the Unix core on Linux, evaluate ACL semantics, and decide whether Linux can be declared supported.
- **Priority:** P2
- **Dependencies:** P3-T2
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - Build and core policy tests run in Linux CI
  - Linux ACL behavior is implemented and tested or explicitly reported unsupported
  - Documentation distinguishes supported behavior from best-effort compilation

## Phase 5: Post-release Hardening

#### P5-T1: Audit v0.1.0 Release ✅ Complete
- **Description:** Perform a formal post-release traceability, security, code-quality, verification, and distribution audit of `v0.1.0`; preserve the review evidence and convert every actionable finding into a tracked follow-up task.
- **Priority:** P0
- **Dependencies:** P4-T1, P4-T2, P4-T3
- **Parallelizable:** no
- **Acceptance Criteria:**
  - A standalone `v0.1.0` review records PRD/Workplan coverage, reproducible findings, release-lineage evidence, quality-gate results, and remediation priorities
  - Findings are classified by severity and reference exact source/spec locations
  - Every actionable finding is represented by an atomic follow-up task with dependencies and verifiable acceptance criteria
  - Validation records the repository state without changing production behavior
  - The review and task artifacts complete the full FLOW archive and review lifecycle

#### P5-T2: Correct Native macOS ACL Evaluation ✅ Complete
- **Description:** Replace synthetic ACL assumptions with native macOS ACL acquisition/evaluation and make every backend error explicitly incomplete. Covers `TK-REV-003` and `TK-REV-004`.
- **Priority:** P0
- **Dependencies:** P5-T1
- **Parallelizable:** no
- **Outputs / Artifacts:** `src/acl.rs`, native macOS ACL integration fixtures, ACL error-path tests
- **Acceptance Criteria:**
  - A native non-owner read/write allow produces the policy-appropriate finding
  - Native header, qualifier, flag, permission, inherited, and allow/deny representations are handled conservatively without synthetic-only grammar
  - `acl_get_file`/conversion failures, unsupported filesystems, permission denial, allocation errors, and target disappearance produce `UNKNOWN/INCOMPLETE`, never `NotPresent` or `PASS`
  - ACL allocations and qualifiers are released on all paths and unsafe FFI remains isolated

#### P5-T3: Inspect the Complete Anchored Ancestor Chain ✅ Complete
- **Description:** Evaluate every component from canonical Home through the target for owner, node type, Unix mode, symlink, metadata completeness, and replacement-capable ACL rights. Covers `TK-REV-001` and `TK-REV-010`.
- **Priority:** P0
- **Dependencies:** P5-T2
- **Parallelizable:** no
- **Outputs / Artifacts:** anchored resolver/inspector component model, ancestor ACL policy, adversarial filesystem tests
- **Acceptance Criteria:**
  - Parent ACL grants such as `add_file`, `add_subdirectory`, `delete_child`, search, or write cannot produce a clean result
  - Foreign-owned, non-directory, unreadable, or mutable ancestors and an unsafe trusted Home are findings or incomplete as appropriate
  - Semantic-root components are walked no-follow from Home; a symlink in `Library/Application Support` or another root cannot be followed before diagnosis
  - Known target findings remain visible when another component is incomplete

#### P5-T4: Compose Confidentiality and Integrity Policies ✅ Complete
- **Description:** Replace the single mutually exclusive policy choice with composable security requirements and correct all credential-bearing built-in locations. Covers `TK-REV-002` and the policy-model portion of `TK-REV-018`.
- **Priority:** P0
- **Dependencies:** P5-T1
- **Parallelizable:** yes
- **Outputs / Artifacts:** policy requirement model, corrected Codex/AWS profiles, policy matrix tests
- **Acceptance Criteria:**
  - Credential-bearing executable configs enforce both owner-only confidentiality and executable-config integrity
  - Codex `config.toml` and AWS `config` mode `0644` cannot pass when declared credential-bearing
  - Node, confidentiality, integrity, ancestor, and ACL requirements compose without profile-specific branching
  - Every built-in location has one explicit, tested effective requirement set

#### P5-T5: Resolve Canonical User Home and Elevated Invocation ✅ Complete
- **Description:** Derive the audited Home from the OS account identity, compare environment state safely, and define root/sudo behavior. Covers `TK-REV-006`.
- **Priority:** P0
- **Dependencies:** P5-T1
- **Parallelizable:** yes
- **Outputs / Artifacts:** UID/account Home resolver, elevated-invocation policy, CLI integration tests and documentation
- **Acceptance Criteria:**
  - Home is resolved from the intended UID through the OS account database rather than trusted from `$HOME`
  - `$HOME` mismatch produces an explicit warning while the canonical Home remains the audited scope
  - Arbitrary same-UID directories cannot produce a misleading all-`SKIP` clean audit
  - root, `sudo`, missing account records, non-absolute Home, and owner mismatch have deterministic exit-2 behavior or an explicitly documented safe target-user flow

#### P5-T6: Make Bounded Traversal Fail-closed and Resource-bounded ✅ Complete
- **Description:** Enforce depth and entry budgets without partial success or unbounded directory enumeration. Covers `TK-REV-007` and `TK-REV-011`.
- **Priority:** P0
- **Dependencies:** P5-T1
- **Parallelizable:** yes
- **Outputs / Artifacts:** bounded traversal budget implementation, completeness results, depth/entry/large-directory tests
- **Acceptance Criteria:**
  - Detecting descendants beyond `max_depth` produces `INCOMPLETE`, not a shortened success
  - Directory enumeration consumes at most the remaining entry budget plus one detection entry
  - Symlink/error branches do not erase prior scope and cannot convert unvisited siblings into a complete exit code
  - Time and memory scale with declared budgets rather than total directory size

#### P5-T7: Implement the Structured Finding and Report Contract ✅ Complete
- **Description:** Implement the full FR-05 result/report model with stable rules, completeness, human guidance, and checked scope. Covers `TK-REV-005` and report-related parts of `TK-REV-012`/`TK-REV-018`.
- **Priority:** P0
- **Dependencies:** P5-T3, P5-T4, P5-T5, P5-T6
- **Parallelizable:** no
- **Outputs / Artifacts:** structured finding model, report renderer, rule catalog, golden output and exit-code tests
- **Acceptance Criteria:**
  - Every finding exposes profile, absolute path, stable rule ID, severity, current state, expected state, risk, and remediation or manual-guidance reason
  - `UNKNOWN`, access denial, missing required scope, and backend failures render their sanitized cause
  - Summary records canonical Home, selected profiles/paths, checked targets, skipped targets, and incomplete branches
  - Known findings and incomplete checks can coexist without losing evidence; zero checked targets cannot claim a complete clean audit
  - Modes render in conventional octal and output remains stable under golden tests

#### P5-T8: Make CLI Scope Selection and Errors Unambiguous ✅ Complete
- **Description:** Define selector composition, duplicate handling, zero-scope behavior, and control-safe rendering for all user-controlled CLI input. Covers `TK-REV-012` and `TK-REV-017`.
- **Priority:** P1
- **Dependencies:** P5-T7
- **Parallelizable:** no
- **Outputs / Artifacts:** CLI validation rules, safe display helper, end-to-end command tests
- **Acceptance Criteria:**
  - `--profile` and `--path/--policy` are either explicitly mutually exclusive or both executed and represented in scope
  - Unknown/duplicate profiles and repeated path/policy flags have deterministic documented behavior
  - ANSI ESC, C0 controls, non-UTF-8 values, quotes, leading dashes, and newlines cannot alter terminal semantics in success or error output
  - All-optional/all-skipped execution follows the documented completeness and exit-code contract

#### P5-T9: Implement Platform-aware Profile Availability ✅ Complete
- **Description:** Derive profile selection, semantic roots, availability, and incomplete-platform results from actual runtime platform metadata. Covers `TK-REV-013`.
- **Priority:** P1
- **Dependencies:** P5-T5, P5-T7
- **Parallelizable:** yes
- **Outputs / Artifacts:** platform/availability evaluator, XDG root policy, profiles output tests, Linux completeness tests
- **Acceptance Criteria:**
  - `profiles` prints each declared platform and current availability rather than a hardcoded label
  - Runtime selects the current platform instead of always checking `Platform::MacOs`
  - A safe `XDG_CONFIG_HOME` inside canonical Home is handled according to documented semantics; an external value is rejected or reported incomplete
  - Unsupported Linux ACL coverage is incomplete at the per-result level and no target line claims an unconditional `PASS`

#### P5-T10: Complete Per-location Profile Evidence and Validation
- **Description:** Record and validate versioned provenance, availability, and exact fixtures for every built-in location. Covers `TK-REV-014`.
- **Priority:** P1
- **Dependencies:** P5-T4, P5-T9
- **Parallelizable:** no
- **Outputs / Artifacts:** per-location evidence schema, researched version/source mapping, complete built-in fixtures, runtime registry validation
- **Acceptance Criteria:**
  - Every location records product/version context, platform, upstream source, verification date, evidence state, policy, optionality, and selector bounds
  - Tests freeze the complete required location inventory and reject duplicate locations, invalid platforms, missing evidence, and invalid requirement combinations
  - Embedded registry validation runs before auditing and cannot silently accept invalid built-ins
  - Project-scoped/Keychain/unknown locations are represented without false coverage claims

#### P5-T11: Build the Native Security Acceptance Suite
- **Description:** Implement the missing PRD security, golden CLI, and non-operation tests as end-to-end acceptance gates. Covers `TK-REV-015`.
- **Priority:** P0
- **Dependencies:** P5-T2, P5-T3, P5-T4, P5-T5, P5-T6, P5-T7, P5-T8, P5-T9, P5-T10
- **Parallelizable:** no
- **Outputs / Artifacts:** macOS integration suite, golden reports, profile fixtures, read/network/subprocess enforcement tests
- **Acceptance Criteria:**
  - Native target and ancestor ACL allow/deny/inherited/error cases run on macOS CI
  - Access denied, foreign owner, semantic-root symlink, depth/entry breach, Home mismatch, root/sudo, ESC/non-UTF-8, and zero-scope cases have end-to-end exit assertions
  - Golden tests cover every status, rule field, remediation suppression, and checked-scope summary
  - Tests demonstrate that target contents are never opened, target metadata is not mutated, network is not used, and remediation subprocesses are never launched
  - Every PRD acceptance scenario maps to at least one named test

#### P5-T12: Enforce Coverage and Reproducible Rust CI
- **Description:** Add coverage and supply-chain/reproducibility gates for the Rust project. Covers the Rust/CI portions of `TK-REV-016` and metadata portion of `TK-REV-018`.
- **Priority:** P1
- **Dependencies:** P5-T11
- **Parallelizable:** yes
- **Outputs / Artifacts:** CI coverage job, pinned toolchain/actions, locked Cargo gates, package metadata
- **Acceptance Criteria:**
  - Line coverage is at least 80% and critical security modules/entrypoints have meaningful branch execution
  - CI runs format, test, Clippy, explicit check/build, and coverage with `--locked` where applicable
  - Rust version is pinned/documented, GitHub Actions are commit-SHA pinned, and cache/toolchain behavior is reproducible
  - `Cargo.toml` declares the MIT license, supported Rust version, and release metadata consistently with `LICENSE`

#### P5-T13: Automate the Homebrew Release Lifecycle
- **Description:** Validate source installation and the full supported Homebrew lifecycle on clean macOS environments. Covers `TK-REV-009` and the Homebrew portion of `TK-REV-016`.
- **Priority:** P1
- **Dependencies:** P5-T11
- **Parallelizable:** yes
- **Outputs / Artifacts:** Homebrew CI workflow, clean install/test/upgrade/uninstall evidence, release checklist
- **Acceptance Criteria:**
  - CI runs formula style/audit, clean source install, `brew test`, upgrade, uninstall, and post-uninstall config-preservation checks
  - Intel and Apple Silicon coverage is documented or gaps are explicitly non-supported
  - Formula has no post-install scan/service/mutation and tests require no real Home or credentials
  - Validation artifacts identify exact release tag, archive SHA-256, Formula commit, Cargo version, and installed binary version

#### P5-T14: Reconcile the Specification Lifecycle
- **Description:** Finalize PRD status, resolve stale open questions, and define authoritative semantics for incomplete versus unknown, all-optional scope, policy composition, and release metadata. Covers the specification portion of `TK-REV-018`.
- **Priority:** P1
- **Dependencies:** P5-T1
- **Parallelizable:** yes
- **Outputs / Artifacts:** updated PRD decisions/status, traceability matrix, release acceptance checklist
- **Acceptance Criteria:**
  - PRD status and version reflect the post-release hardening state
  - Resolved license, tap owner, release, root/sudo, selector, JSON/SARIF, and policy-precedence questions are closed or explicitly deferred with owners/tasks
  - `UNKNOWN`, `INCOMPLETE`, zero checked targets, optional-only scope, and point-in-time verdict semantics are unambiguous
  - Canonical PRD criteria cannot be weakened by task-local acceptance without an explicit PRD change

#### P5-T15: Publish a Traceable Successor Release
- **Description:** Release the completed hardening work from reviewed main-line history without moving `v0.1.0`. Covers `TK-REV-008`.
- **Priority:** P0
- **Dependencies:** P5-T7, P5-T8, P5-T9, P5-T10, P5-T11, P5-T12, P5-T13, P5-T14
- **Parallelizable:** no
- **Outputs / Artifacts:** version bump, immutable signed tag, GitHub release, source checksum, updated tap Formula and release validation report
- **Acceptance Criteria:**
  - Existing `v0.1.0` tag remains unchanged and documented as superseded
  - Successor tag is an ancestor of `main` and identifies the exact reviewed tree
  - Cargo version, binary `--version`, tag, release archive, Formula URL/version/checksum, and documentation agree
  - Full PRD acceptance, native security suite, coverage, Rust CI, and Homebrew lifecycle gates pass before publication
  - Post-publication install retrieves the successor behavior and no two different source trees share its version

## Task Status Legend

- **Not Started** — task is available or waiting on dependencies
- **INPROGRESS** — task is selected and being executed
- **✅ Complete** — task and validation artifacts are archived
