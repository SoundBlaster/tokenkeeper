# P1-T1 — Bootstrap Rust Project

## Objective

Создать минимальный, compilable Rust binary crate для Tokenkeeper с reproducible local quality gates и безопасной CLI boundary. Эта задача подготавливает delivery skeleton, но не реализует filesystem audit, profiles, ACL или remediation.

## Scope and constraints

- Stable Rust и Cargo; edition 2021 для широкой совместимости.
- Binary package name: `tokenkeeper`, version `0.1.0`.
- Runtime dependencies отсутствуют, если они не нужны для smoke CLI.
- CLI не читает `$HOME`, не открывает target files, не запускает subprocess и не изменяет filesystem.
- Repository URL: `https://github.com/SoundBlaster/tokenkeeper`.
- Quality commands берутся из `.flow/params.yaml`.

## Deliverables

1. `Cargo.toml` и generated `Cargo.lock` с package metadata.
2. `src/main.rs` и минимальный `src/cli.rs` или эквивалентный модуль для `--help`, `--version` и unknown-argument handling.
3. `tests/cli_smoke.rs` с black-box tests через `CARGO_BIN_EXE_tokenkeeper`.
4. `.github/workflows/ci.yml` для `fmt --check`, tests и Clippy на stable Rust.
5. Короткий `README.md` с назначением проекта, ограничением metadata-only и командами разработки.
6. `SPECS/INPROGRESS/P1-T1_Validation_Report.md` с фактическими командами и результатами.

## User-visible contract

```text
tokenkeeper --help     -> exit 0, usage и краткое описание
tokenkeeper --version  -> exit 0, `tokenkeeper 0.1.0`
tokenkeeper --unknown  -> exit 2, error в stderr, без panic
```

Help/version paths не должны выполнять scan или требовать permissions к `$HOME`. Output не должен содержать usernames, tokens или environment secrets.

## Test-first execution plan

### Phase 1 — Failing acceptance tests

- Написать CLI smoke tests для help, version и unknown argument.
- Добавить assertions на exit code, stdout/stderr и отсутствие destructive side effects.
- Запустить `cargo test`, зафиксировать ожидаемый red state до production implementation.

### Phase 2 — Minimal implementation

- Создать package и CLI parser без agent-specific behavior.
- Реализовать только contract из User-visible contract.
- Добавить compile-time package version из Cargo metadata.

### Phase 3 — Delivery gates

- Добавить README и CI workflow.
- Запустить `cargo test --all-targets --all-features`.
- Запустить `cargo clippy --all-targets --all-features -- -D warnings`.
- Запустить `cargo fmt --all --check`.
- Записать результаты в validation report.

## Acceptance criteria

- `Cargo.toml`, `Cargo.lock`, `src/` и `tests/` существуют.
- `tokenkeeper --help` и `tokenkeeper --version` завершаются с code `0`.
- Unknown argument завершается с code `2` без panic.
- CLI smoke tests проходят и не требуют real agent config или credentials.
- Все три Cargo quality gates проходят локально.
- CI workflow запускает те же gates на stable Rust.
- README не обещает реализованный audit до следующих tasks.
- Изменения не читают и не меняют пользовательский `$HOME`.

## Out of scope

Profile registry, path resolver, Unix metadata, owner/mode policies, ACL backend, MCP locations, token detection, shell remediation, Homebrew formula и Linux support реализуются отдельными Workplan tasks.

## Notes

После выполнения обновить только task artifacts и status P1-T1. Следующим кандидатом остаётся P1-T2 — research agent and integration storage locations.
