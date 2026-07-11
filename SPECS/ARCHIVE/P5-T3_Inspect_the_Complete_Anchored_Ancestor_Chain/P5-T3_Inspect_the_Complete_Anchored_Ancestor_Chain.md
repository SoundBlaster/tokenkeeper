# P5-T3 — Inspect the Complete Anchored Ancestor Chain

## Objective

Evaluate every path component from canonical Home through each target without following symlinks, and surface ancestor ownership, node-type, mode, metadata, and ACL replacement risks.

## Deliverables

- Resolver/inspector component walk anchored at `home`.
- Ancestor ACL evaluation using the policy relevant to replacement/search rights.
- Findings for foreign-owned, non-directory, unreadable, mutable, or symlinked ancestors while retaining target findings.
- Regression tests for parent ACL grants and incomplete ancestors.

## Acceptance Criteria

- Parent ACL grants (`add_file`, `add_subdirectory`, `delete_child`, search, or write) cannot yield a clean result.
- Foreign-owned, non-directory, unreadable, or mutable ancestors and unsafe Home become findings or incomplete results.
- Semantic-root components are inspected no-follow from Home; symlink components are diagnosed before traversal.
- A known target finding remains visible when another component is incomplete.

## Dependencies

- P5-T2 — Correct Native macOS ACL Evaluation.
