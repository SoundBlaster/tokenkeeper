# P5-T6 — Make Bounded Traversal Fail-closed and Resource-bounded

## Objective

Ensure bounded traversal never returns a partial success and does not collect an unbounded directory listing before applying `max_entries`.

## Acceptance Criteria

- Entry limits are checked before adding a result.
- Directory children are traversed incrementally without an unbounded intermediate vector.
- Limit, permission, and I/O errors remain explicit and fail closed.
- Existing symlink and bounded traversal tests remain green.

## Dependency

P5-T1 — Audit v0.1.0 Release.
