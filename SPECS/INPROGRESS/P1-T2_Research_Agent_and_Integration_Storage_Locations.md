# P1-T2 — Research Agent and Integration Storage Locations

## Objective

Собрать проверяемый inventory file-based locations, где Codex, Claude Code, OpenCode, Cursor, MCP servers и сторонние utilities хранят credentials, sensitive configs или executable integration settings. Результат должен быть достаточным для создания embedded profiles, но не должен раскрывать реальные secrets.

## Scope and evidence policy

- Primary platform: macOS; Linux paths фиксируются только если они подтверждены и не задерживают macOS scope.
- GitHub Copilot — optional и не блокирует verdict задачи.
- Upstream documentation — источник заявленной product behavior.
- Local filesystem — только path existence, node type, owner/mode и app/version metadata; contents никогда не читаются.
- Отсутствующий продукт или undocumented storage помечается `not installed`, `unknown` или `Keychain-only`, а не считается покрытым.
- Все пользовательские absolute paths нормализуются до `$HOME/...`; usernames, token values и config contents не записываются.

## Deliverables

1. `docs/agent-storage-locations.md` с inventory и evidence table.
2. Policy classification для каждой location: `SecretFile`, `CredentialConfig`, `PrivateDirectory`, `TrustedConfig` или `ExecutableConfig`.
3. Статус доказательности: `confirmed` (docs + local), `documented` (docs only) или `unverified`.
4. Список Keychain-only, database-backed и неизвестных storages, которые metadata-only Tokenkeeper не проверяет как token content.
5. `SPECS/INPROGRESS/P1-T2_Validation_Report.md` с командами, источниками, датой и итогом.

## Research matrix

Обязательные rows должны покрывать:

- Codex;
- Claude Code;
- OpenCode;
- Cursor;
- MCP server/integration configs;
- credential-bearing configs сторонних utilities.

Для каждой row фиксируются product/integration, platform, version или version source, semantic root, relative path/pattern, expected node type, sensitivity policy, storage backend, evidence links и verification date.

## Safe local inspection

Разрешены только команды, возвращающие path/metadata, например `stat`, bounded `find`, `mdls` для app version и `git ls-files` для repository fixtures. Запрещены `cat`, `jq`/парсинг contents, environment dumps, Keychain export, network upload и копирование config files. При сомнении location помечается `unknown`.

## Execution phases

### Phase 1 — Source collection

- Собрать official documentation links и documented defaults.
- Зафиксировать дату и product/version context.

### Phase 2 — Local path verification

- Проверить существование только ожидаемых paths под `$HOME`.
- Снять type/owner/mode без follow symlink и без открытия contents.
- Проверить candidate MCP/utility config roots с bounded traversal.

### Phase 3 — Profile-ready synthesis

- Удалить usernames и реальные paths из evidence text.
- Сопоставить locations с policies и optionality.
- Явно перечислить unsupported/Keychain-only cases и ограничения.

## Acceptance criteria

- В документе есть rows для всех обязательных categories и отдельно отмечен optional Copilot.
- Каждая claimed location имеет source, platform, version context, verification date и evidence status.
- MCP/utility configs, потенциально содержащие credentials, классифицированы как `CredentialConfig` или обоснованно исключены.
- Ни один token, credential value или config content не попал в repository.
- Local inspection не изменяет `$HOME`, не выполняет remediation и не обращается к network.
- Unknown, absent, Keychain-only и confirmed file-based storage не смешаны в один PASS.
- Следующая task P1-T3 может использовать документ как единственный input для profile schema fixtures.

## Out of scope

Secret scanning по содержимому, проверка token validity, чтение Keychain, реализация resolver/profile registry, ACL evaluation и автоматические исправления.

## Notes

Если upstream docs и локальная установка расходятся, документировать обе версии и выбрать conservative profile rule. Не использовать реальные credentials даже в локальных fixtures.
