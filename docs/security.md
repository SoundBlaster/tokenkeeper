# Security and Privacy Notes

## What is checked

Tokenkeeper performs a point-in-time metadata audit below a trusted Home directory. It uses no-follow filesystem metadata for owner UID/GID, node type, Unix mode bits, parent directories and symlink components. On macOS it also evaluates extended ACLs conservatively: relevant non-owner allows are findings, while inherited, ambiguous or unreadable ACL state is `UNKNOWN`.

The audit does not open target files, parse JSON/TOML, inspect tokens, access Keychain records, contact a network service or run a remediation command. Profiles classify known credential-bearing files without asserting that a credential is present.

## Reading the result

- `PASS` means only that the evaluated metadata and available ACL checks met the selected policy at that instant.
- `FINDING` identifies an observable owner, mode, ACL, symlink, node-type or ancestor risk.
- `SKIP` is an optional location that is absent.
- `UNKNOWN`/exit code `2` means the required scope could not be completed (for example missing required path, denied access, traversal limit or unsupported ACL backend).

Review any suggested command before copying it. The tool prints commands only for a verified regular file/directory with a shell-safe absolute path and no ambiguous owner, symlink, ACL or writable-ancestor state. Never use a recursive `chmod`, automatic `chown`, ACL deletion or `sudo` based solely on the report.

## Threat-model limits

The normal-user audit is not protection from a process with the same UID, `root`, malware, a compromised agent, or a process racing the check. It does not prove token validity, inspect environment variables or process arguments, detect secrets in unknown files, or protect against exposure through backups, cloud sync, snapshots, crash dumps or terminal logs. A file may change between the audit and a manually executed command.

Keychain-only credentials and remote MCP state are outside this metadata-only tool. Project-root-dependent configs are not recursively discovered; use an explicit path or a future bounded profile. Linux reuses the Unix resolver and mode checks in CI, but the macOS extended-ACL backend is unavailable there. Linux checks print an incomplete-audit warning and exit `2`; Linux ACL semantics are not declared supported.

## Safe operating practice

Run as the intended user without privilege escalation, inspect the full path and finding reason, and apply any change manually after confirming the target. Keep reports free of filenames containing secrets and do not paste credential values into issues or logs. The MIT license and repository history describe the software terms; security claims are limited to the documented checks above.
