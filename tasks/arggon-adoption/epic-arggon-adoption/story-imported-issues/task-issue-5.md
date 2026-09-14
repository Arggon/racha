---
type: task
status: in_progress
id: task-issue-5
title: "issue #5: Export/import de datos"
assignee: Arggon
parent: story-imported-issues
labels: [enhancement]
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:25:47.928Z"
issue: 5
---
Portabilidad del ledger:

- [ ] export: CSV y/o JSON canónico versionado
- [ ] import idempotente con validación (nunca corrompe el ledger existente)
- [ ] docs de migración entre formatos

Criterio de aceptación: round-trip export→import no pierde ni duplica checks.
> imported from issue #5
