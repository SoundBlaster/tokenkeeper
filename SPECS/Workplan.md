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

#### P2-T3: Implement macOS ACL Backend **INPROGRESS**
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

#### P3-T1: Add Built-in Agent and Integration Profiles
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

#### P3-T2: Complete Security Integration Tests
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

#### P4-T1: Publish User and Security Documentation
- **Description:** Document installation, supported scope, output interpretation, limitations, and manual remediation workflow.
- **Priority:** P1
- **Dependencies:** P3-T2
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - README contains install and minimal usage examples
  - Security documentation explains same-UID, root, Keychain, backup/sync, and point-in-time limitations
  - Supported profile/version evidence is discoverable
  - Example output contains no real usernames or credentials

#### P4-T2: Add Homebrew Tap Distribution
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

#### P4-T3: Evaluate Linux Support
- **Description:** Reuse the Unix core on Linux, evaluate ACL semantics, and decide whether Linux can be declared supported.
- **Priority:** P2
- **Dependencies:** P3-T2
- **Parallelizable:** yes
- **Acceptance Criteria:**
  - Build and core policy tests run in Linux CI
  - Linux ACL behavior is implemented and tested or explicitly reported unsupported
  - Documentation distinguishes supported behavior from best-effort compilation

## Task Status Legend

- **Not Started** — task is available or waiting on dependencies
- **INPROGRESS** — task is selected and being executed
- **✅ Complete** — task and validation artifacts are archived
