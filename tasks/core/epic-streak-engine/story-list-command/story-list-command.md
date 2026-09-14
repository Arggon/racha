---
type: story
status: in_progress
id: story-list-command
title: racha list — listado compacto de hábitos
assignee: agent-race-a
branch: feat/story-list-command
parent: epic-streak-engine
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:05:02.113Z"
worktree_path: /home/arggon/Projects/racha-story-list-command
---
<!--
  Placement (v0): tasks/core/epic-streak-engine/story-list-command/story-list-command.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# racha list — listado compacto de hábitos

## Context

Listado compacto para scripting: `racha list` imprime un hábito por línea con su
racha actual, sin el detalle completo de stats. Es además el vehículo del escenario
de prueba de la carrera de claims (lock de `start --worktree`).

## Acceptance

- [ ] `racha list` imprime un hábito por línea: nombre + racha actual
- [ ] exit 0 con lista vacía y mensaje claro si no hay hábitos
- [ ] tests de integración del output
- [ ] README actualizado con el comando

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
