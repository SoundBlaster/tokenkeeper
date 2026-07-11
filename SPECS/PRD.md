# Tokenkeeper — Product Requirements Document

| Поле | Значение |
| --- | --- |
| Статус | Draft |
| Версия | 0.1 |
| Основная платформа | macOS |
| Язык реализации | Rust |
| Основной release channel | Homebrew tap |

## 1. Резюме

Tokenkeeper — минимальная локальная CLI-утилита для ручного аудита filesystem permissions файлов конфигурации, credentials AI coding agents и MCP/utility integrations в домашней директории пользователя. Утилита проверяет только metadata, объясняет найденный риск и предлагает стандартную команду исправления. Она не читает токены, не изменяет файлы и не выполняет предложенные команды.

Первая версия ориентирована на macOS и распространяется через Homebrew tap. Общая Unix-логика проектируется переносимой, чтобы позднее добавить подтверждённую поддержку Linux.

## 2. Проблема

Agent tools, MCP servers и сторонние utilities сохраняют credentials, настройки, state databases и integration configs в разных locations внутри `$HOME`. Такие config files часто содержат embedded tokens или credentials для MCP и внешних сервисов. Избыточные mode bits, неверный owner, extended ACL, небезопасный symlink или writable parent directory могут раскрыть token другому локальному пользователю либо позволить изменить конфигурацию агента.

Ручная проверка требует знания всех locations и различий между confidentiality и integrity policies. Пользователю нужен короткий, понятный и воспроизводимый отчёт без автоматического вмешательства в систему.

## 3. Цели

- Находить небезопасные owner, Unix mode, ACL, node type, symlink и parent-directory permissions.
- Проверять известные locations через встроенные declarative profiles.
- Поддерживать явную проверку пользовательского path с выбранной policy.
- Объяснять фактическое состояние, ожидаемое состояние и риск.
- Предлагать минимальную shell-safe remediation-команду, когда она однозначна.
- Работать локально, без network access, telemetry и privilege escalation.
- Позволять добавлять новый agent profile без изменения auditing core.
- Обеспечить воспроизводимую установку и обновление через Homebrew.

## 4. Не-цели

В `v0.1` не входят:

- автоматический `fix`, запуск команд или изменение filesystem;
- чтение содержимого configs, tokens, SQLite databases или иных target files;
- доступ к macOS Keychain и проверка валидности credentials;
- secret scanning, malware detection, daemon/watch mode и background monitoring;
- сканирование всего `$HOME` без ограниченного profile/path;
- executable third-party plugins, dynamic libraries, scripts или callbacks;
- Windows и обещание полной поддержки Linux;
- публикация в `homebrew-core` до выполнения upstream eligibility requirements;
- защита от процессов с тем же UID, `root`, malware, backup или sync leakage.

## 5. Пользователи и сценарии

Основной пользователь — developer, который локально использует AI coding agents, MCP servers и сторонние utilities и хочет проверить exposure их credential-bearing configs.

- Запустить `tokenkeeper check` и проверить все встроенные profiles.
- Проверить только один agent: `tokenkeeper check --profile codex`.
- Проверить собственный location с явной policy.
- Скопировать предложенную команду, самостоятельно оценить её и выполнить при необходимости.
- Добавить новый встроенный profile как data-only описание locations и policies.

## 6. Scope `v0.1`

Встроенный registry должен содержать обязательные profiles:

- Codex;
- Claude Code;
- OpenCode;
- Cursor;
- MCP/integration configs с подтверждёнными file-based locations.

GitHub Copilot остаётся optional profile и может быть отложен после `v0.1`, если его file-based storage не даёт полезного покрытия.

Точные paths и типы хранилищ должны быть проверены по upstream documentation и локальными fixtures до реализации каждого profile. Profile location может принадлежать агенту, MCP server или utility. Если продукт хранит credential в Keychain, Tokenkeeper сообщает об ограничении и не заявляет, что credential был проверен.

Profile или пользовательская policy может объявить config как `CredentialConfig`, если он известен как потенциально содержащий token/credential. Tokenkeeper проверяет exposure такого файла, но не читает contents и не пытается самостоятельно доказать наличие секрета.

Публичный `v0.1` должен быть доступен как Homebrew formula из maintainer-owned tap. Formula собирается из versioned source release; bottles могут быть добавлены позднее как optimization.

## 7. Functional requirements

### FR-01 — Запуск и выбор scope

- `tokenkeeper check` проверяет все доступные для текущей платформы profiles.
- Повторяемый `--profile <id>` ограничивает проверку указанными profiles, например `tokenkeeper check --profile codex --profile cursor`.
- `--path <path> --policy <policy>` проверяет явный file или directory.
- `tokenkeeper profiles` выводит profile ID, display name, platform и доступность.
- Missing optional location получает статус `SKIP`, а missing required location — `INCOMPLETE` с exit code `2`.

### FR-02 — Разрешение paths

Profiles используют semantic roots: `Home`, `XdgConfig` и `MacApplicationSupport`. Home текущего пользователя определяется по UID через операционную систему; несовпадение с `$HOME` показывается как warning. Relative path не может содержать `..` или выходить за declared root. В `v0.1` profile roots и custom paths должны находиться внутри trusted Home текущего пользователя; внешний path отклоняется с exit code `2`.

Допустимы exact paths и явно ограниченные selectors с пределами глубины и количества entries. Unbounded recursive traversal запрещён. Каждый path разрешается component-by-component от trusted Home: symlink в target или любом промежуточном component создаёт finding и останавливает traversal по этой ветке.

### FR-03 — Сбор metadata

Для каждого target Tokenkeeper получает без открытия содержимого:

- file type каждого path component через no-follow metadata operation;
- numeric UID/GID и текущего owner;
- Unix permission и special bits;
- extended ACL на macOS;
- metadata всех parent directories от trusted Home до target.

Access denied, unsupported ACL backend и другие неполные проверки дают `UNKNOWN/INCOMPLETE`, а не `PASS`. Filesystem errors не должны приводить к panic.

### FR-04 — Policies

| Policy | Требование |
| --- | --- |
| `SecretFile` | Regular file текущего пользователя; запрещён весь доступ `group/others` и несовместимый ACL. `0600` — рекомендуемое, но более строгое состояние, например `0400`, допустимо. |
| `CredentialConfig` | Config file, который может содержать token или credential для agent, MCP или utility; применяется та же confidentiality policy, что и для `SecretFile`. Наличие credential не определяется чтением contents. |
| `PrivateDirectory` | Directory текущего пользователя; запрещён весь доступ `group/others` и несовместимый ACL. Рекомендуемое состояние — `0700`. |
| `TrustedConfig` | Текущий owner; запрещена запись через `group/others` или ACL. Чтение может быть разрешено. |
| `ExecutableConfig` | Требования `TrustedConfig` плюс строгая проверка writable ancestors, поскольку изменение может привести к выполнению команд агентом. |

Известный или явно объявленный credential-bearing config классифицируется как `CredentialConfig`, независимо от extension. Writable ancestor между trusted Home и target создаёт отдельный integrity finding.

ACL backend должен либо корректно вычислять effective rights с учётом inherited entries и порядка allow/deny, либо применять консервативное правило: любой релевантный non-owner `ALLOW`, безопасность которого нельзя доказать, даёт `FINDING` или `UNKNOWN`, но не `PASS`.

### FR-05 — Findings и отчёт

Каждый finding содержит:

- profile и абсолютный target path;
- стабильный rule ID и severity;
- current metadata и expected policy;
- краткое описание риска;
- remediation-команду либо объяснение, почему безопасную команду предложить нельзя.

Summary показывает число `PASS`, `FINDING`, `UNKNOWN` и `SKIP`, а также полный проверенный scope. Отчёт не должен формулировать общий вывод «все токены безопасны».

### FR-06 — Remediation guidance

Tokenkeeper только печатает команды и никогда их не выполняет. Path должен быть абсолютным и корректно shell-escaped; terminal control characters экранируются. Если lossless и безопасное представление невозможно, команда не генерируется.

Remediation-команда генерируется только для проверенного regular file/directory, если target и промежуточные components не являются symlink, а parent chain не writable для посторонних. При unsafe ancestor, ACL/owner finding или unexpected node type выводится только manual guidance.

Допустимы минимальные предложения вроде:

```bash
chmod go-rwx '/Users/alice/.agent/credentials.json'
chmod go-w '/Users/alice/.agent/config.toml'
```

В `v0.1` запрещено предлагать `chmod -R`, автоматический `chown`, полное удаление ACL или изменение symlink. Утилита не вызывает и не запрашивает `sudo`.

### FR-07 — Exit codes

| Code | Значение |
| --- | --- |
| `0` | Проверка завершена полностью, findings отсутствуют. |
| `1` | Проверка завершена полностью, найдены security findings. |
| `2` | Invalid usage, operational error или неполная проверка. |

## 8. Embedded profile system

Profile — data-only descriptor, встроенный в binary. Core не содержит условий вида `if profile == "codex"`. Profile описывает:

- стабильные `id` и `display_name`;
- поддерживаемые platforms;
- semantic root и relative path/ограниченный selector;
- ожидаемый node type;
- policy и optionality;
- traversal limits;
- источник и дату последней проверки location.

Unknown fields, duplicate IDs, absolute profile paths, `..` и unbounded traversal отклоняются validation test. Физический формат — typed Rust data либо embedded manifest — определяется в technical design. Пользовательские TOML profiles могут использовать ту же модель позднее, но не входят в `v0.1`.

## 9. CLI и пример результата

```text
$ tokenkeeper check --profile codex

FINDING  ~/.codex/auth.json
         rule: TK-PERM-001
         current: regular file, owner=alice, mode=0644
         expected: SecretFile (owner-only access)
         risk: credential may be read by another local user
         suggested: chmod go-rwx '/Users/alice/.codex/auth.json'

Summary: 3 checked, 1 finding, 0 unknown, 2 passed
```

Default output предназначен человеку, стабилен в основных терминах, но не является machine-readable API. JSON/SARIF output рассматривается после `v0.1`.

## 10. Homebrew distribution

Для `v0.1` используется maintainer-owned tap согласно [официальному руководству Homebrew](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap). Основной пользовательский путь — одна fully qualified команда:

```bash
brew install <owner>/homebrew-tap/tokenkeeper
```

Требования к release pipeline и formula:

- stable SemVer tag и immutable source archive с SHA-256 checksum;
- formula фиксирует `url`, `version`, `sha256`, project license и build dependencies;
- сборка выполняется из исходного кода через Cargo, без загрузки unversioned content;
- formula устанавливает только binary и необходимую metadata;
- `post_install`, daemon/service setup и автоматический scan пользовательского `$HOME` запрещены;
- `test do` не требует input и проверяет безопасные команды вроде `tokenkeeper --version` и `tokenkeeper profiles` во временном Homebrew test environment;
- CI выполняет `brew audit`, source install, `brew test`, upgrade и uninstall validation;
- README документирует install, update, upgrade и uninstall commands.

Formula должна следовать [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook). Публикация в `homebrew-core` рассматривается отдельно после появления стабильных releases, требуемой platform support и достаточной известности проекта.

## 11. Security и privacy requirements

- Утилита работает от обычного пользователя и не требует elevated privileges.
- Утилита не требует и не инициирует privilege elevation. Elevated invocation не должна молча изменять определение target user или audit scope.
- Никакие target contents, token values или Keychain records не читаются и не логируются.
- Никакие network requests, telemetry, cache или subprocess remediation не допускаются.
- ANSI/control sequences из filenames не могут влиять на terminal output.
- Unsafe/FFI, необходимый для platform ACL API, изолируется, документируется и тестируется.
- Dependencies минимизируются; `Cargo.lock` фиксируется в репозитории.
- Результат описывается как point-in-time audit: файл может измениться после проверки или до выполнения скопированной команды.

## 12. Предлагаемая архитектура

Один Rust crate с внутренними компонентами:

```text
CLI → Profile Registry → Path Resolver → Platform Inspector
                                      → Policy Evaluator → Report
```

- `cli` — arguments и exit codes;
- `profiles` — встроенные data-only definitions для agent, MCP и utility categories;
- `resolver` — semantic roots и bounded discovery;
- `platform` — Unix metadata и macOS ACL backend;
- `policy` — platform-neutral evaluation;
- `report` — findings, summary и shell-safe guidance.

Linux backend должен переиспользовать Unix core. Linux считается supported только после отдельных ACL semantics tests и CI validation.

## 13. Verification strategy

Unit tests покрывают policy matrix и profile validation. Integration tests используют временное filesystem tree и проверяют:

- safe `0600`/`0400` и unsafe `0640`/`0604` для `SecretFile`;
- safe `0700` и unsafe `0750`/`0707` для `PrivateDirectory`;
- допустимый `0644` и unsafe `0664` для `TrustedConfig`;
- wrong owner, unexpected node type и writable ancestor;
- symlink наружу, symlink loop и отсутствие follow behavior;
- macOS ACL, выдающий другому principal read/write access;
- inherited ACL и неоднозначный allow/deny ordering без ложного `PASS`;
- различие missing optional, missing required и access denied;
- paths с spaces, quotes, leading dash, newline и ANSI bytes;
- отсутствие content reads, filesystem mutations, network access и выполнения remediation.

Golden tests фиксируют человекочитаемый report и exit codes. Profile fixtures подтверждают все обязательные built-in IDs и заявленные agent/MCP/utility locations.

## 14. Acceptance criteria `v0.1`

- Все обязательные profiles отображаются через `tokenkeeper profiles`: Codex, Claude Code, OpenCode, Cursor и MCP/integration configs. GitHub Copilot может быть optional.
- Любой selector соблюдает limits по depth и entry count; превышение limit даёт `INCOMPLETE`, а не частичный `PASS`.
- `SecretFile` текущего пользователя с mode `0644` даёт finding и shell-safe предложение убрать доступ `group/others`; `0600` и `0400` проходят проверку.
- `PrivateDirectory` с mode `0777` даёт finding; `0700` проходит проверку.
- Mode, owner, node type, symlink, ancestor и macOS ACL findings проходят integration tests.
- Неподдержанная или неудачная ACL-проверка никогда не отображается как `PASS`.
- Утилита не читает содержимое target files и ничего не изменяет.
- Remediation output устойчив к shell и terminal injection.
- Для каждого finding доступны rule ID, risk, expected state и actionable guidance.
- Все заявленные profile locations имеют evidence и fixture; неизвестные locations не выдаются как проверенные.
- Документация явно описывает ограничения same-UID access, Keychain, `root` и point-in-time audit.
- Formula устанавливается из maintainer-owned tap, проходит `brew audit` и `brew test`, а uninstall не изменяет пользовательские configs или credentials.
- Release tag, source archive, formula version и SHA-256 checksum согласованы между собой.

## 15. Риски и mitigations

| Риск | Mitigation |
| --- | --- |
| Agent изменил storage location | Versioned profiles, source metadata, fixtures и явный scope summary. |
| ACL интерпретирован неверно | Отдельный macOS backend; `UNKNOWN` при сомнении; platform integration tests. |
| Symlink выводит scan за scope | No-follow metadata и запрет recursive symlink traversal. |
| Filename внедряет shell/terminal input | Раздельное display/command escaping; отказ от команды для unsafe path. |
| Remediation слишком широкая | Только минимальные non-recursive команды; ownership/ACL требуют ручного review. |
| Пользователь получает ложное чувство безопасности | Чёткие non-goals, проверенный scope и отсутствие глобального «secure» verdict. |
| Сканирование вызывает DoS | Limits на depth, entry count и glob expansion. |
| Rust воспринимается как полная security guarantee | Минимум dependencies, изоляция `unsafe`, threat-model tests и code review. |
| Formula указывает на изменяемый или неподтверждённый artifact | Immutable release tag, pinned URL, SHA-256 и automated install test. |

## 16. Success metrics

- 100% заявленных built-in locations имеют validation evidence и tests.
- 100% перечисленных acceptance scenarios дают ожидаемый статус и exit code.
- Ноль target-content reads, автоматических mutations и network calls.
- Ноль panic на malformed profile/path corpus.
- Пользователь может понять finding и применить предложенное исправление без обращения к исходному коду.
- Homebrew install, audit, test, upgrade и uninstall gates проходят для каждого public release.

## 17. Roadmap

1. **Discovery:** подтвердить locations и storage semantics пяти agents.
2. **Core:** реализовать Rust CLI, resolver, Unix metadata и policy evaluator.
3. **macOS:** добавить ACL backend, profiles, reports и security tests.
4. **Release `v0.1`:** documentation, versioned source release и Homebrew tap formula.
5. **После `v0.1`:** bottles, возможная публикация в `homebrew-core`, Linux support, external declarative profiles и machine-readable output.

## 18. Открытые вопросы

- Какие versions каждого агента и MCP/utility integrations считаются baseline для location research?
- Нужен ли bounded glob/recursion всем profiles уже в `v0.1`?
- Следует ли публиковать inspection-only команды для сложных ACL findings?
- Нужны ли JSON или SARIF раньше пользовательских external profiles?
- Каковы GitHub owner и имя maintainer-owned Homebrew tap?
- Какую open-source license указать для crate, release и formula?
- Как именно обрабатывать запуск через `sudo` или от `root`, не меняя target user и scope молча?
