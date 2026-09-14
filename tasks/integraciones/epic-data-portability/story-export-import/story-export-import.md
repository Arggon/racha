---
type: story
status: in_progress
id: story-export-import
title: Export/import del ledger + runbook de restore
assignee: Arggon
branch: feat/story-export-import
parent: epic-data-portability
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:15:29.971Z"
depends_on: [story-streak-engine-spec]
worktree_path: /home/arggon/Projects/racha-story-export-import
---
<!--
  Placement (v0): tasks/integraciones/epic-data-portability/story-export-import/story-export-import.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# Export/import del ledger + runbook de restore

## Context

Issue #5: portabilidad. Export a CSV/JSON canónico e import idempotente con
validación; más runbook de restore de datos (operación: recuperar un ledger
perdido/corrupto desde un export).

## Acceptance

- [ ] `racha export --format csv|json [--out <file>]`
- [ ] `racha import <file>` idempotente y validado: rechaza datos inválidos sin tocar el ledger existente
- [ ] Round-trip test: export → import no pierde ni duplica checks
- [ ] Runbook docs/runbooks/restore-de-datos.md

Closes #5.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
