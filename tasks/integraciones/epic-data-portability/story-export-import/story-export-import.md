---
type: story
status: done
id: story-export-import
title: Export/import del ledger + runbook de restore
assignee: Arggon
branch: feat/story-export-import
parent: epic-data-portability
labels: []
created: "2026-09-14"
updated: "2026-09-14"
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

- [x] `racha export --format csv|json [--out <file>]`
- [x] `racha import <file>` idempotente y validado: rechaza datos inválidos sin tocar el ledger existente
- [x] Round-trip test: export → import no pierde ni duplica checks
- [x] Runbook docs/runbooks/restore-de-datos.md

Closes #5.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.

### 2026-09-14 @Arggon
Implementado en PR #15 (merge squash a master). racha export --format csv|json [--out]: JSON canónico {"version":1,"habits":[{name,created,checks}]} (mismo shape del ledger + version), CSV header habit,date una fila por check ordenado. racha import <file>: detecta formato por contenido, valida fechas ISO/nombres no vacíos/sin checks duplicados antes de escribir, escritura atómica (nuevo storage::save_atomic, temp+rename, también usado por save()), merge aditivo idempotente por nombre+fecha; errores: mensaje claro + exit 1 con ledger intocado. Tests: 69 (35 unit + 30 CLI + 4 doc), incluye round-trip byte-idéntico, idempotencia, CSV/JSON, imports inválidos sin tocar el ledger. Runbook docs/runbooks/restore-de-datos.md (incluye manejo de export truncado). Conflictos del rebase con story-web-static resueltos preservando ambas features.
