---
type: story
status: in_progress
id: story-web-static
title: racha web — HTML estático de solo lectura
assignee: Arggon
branch: feat/story-web-static
parent: epic-web-view
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:15:49.116Z"
depends_on: [story-streak-engine-spec]
worktree_path: /home/arggon/Projects/racha-story-web-static
---
<!--
  Placement (v0): tasks/integraciones/epic-web-view/story-web-static/story-web-static.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# racha web — HTML estático de solo lectura

## Context

Issue #4: vista web de SOLO lectura. `racha web` genera HTML estático desde el
ledger; sin backend con estado; la escritura sigue siendo CLI.

## Acceptance

- [ ] `racha web [--out <dir>]` genera index.html con rachas actuales, semana y totals por hábito
- [ ] Sin server con estado: archivo estático abrible directo
- [ ] Test de generación (contenido mínimo esperado)
- [ ] README actualizado

Closes #4.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
