---
type: story
status: done
id: story-web-static
title: racha web — HTML estático de solo lectura
assignee: Arggon
branch: feat/story-web-static
parent: epic-web-view
labels: []
created: "2026-09-14"
updated: "2026-09-14"
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

- [x] `racha web [--out <dir>]` genera index.html con rachas actuales, semana y totals por hábito
- [x] Sin server con estado: archivo estático abrible directo
- [x] Test de generación (contenido mínimo esperado)
- [x] README actualizado

Closes #4.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.

### 2026-09-14 @Arggon
Implementado en PR #14 (squash-merged): módulo puro src/web.rs (render(&Ledger, today) -> String) consumiendo racha::streaks; comando 'racha web [--out <dir>]' escribe index.html estático (default ./racha-web/) abrible con file://; por hábito: racha actual, mejor, total, % semana, mes, año, vista semanal y últimos 14 días; nombres escapados HTML (test de anti-XSS incluido); empty state para ledger vacío; README con sección 'Vista web'. Gates: 50 tests, clippy 0 warnings, fmt limpio. Conflicto de README en rebase resuelto preservando la sección de systemd timer (aditivo).
