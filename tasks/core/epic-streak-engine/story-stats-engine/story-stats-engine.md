---
type: story
status: in_progress
id: story-stats-engine
title: "Estadísticas agregadas: mes/año y % de cumplimiento semanal"
assignee: Arggon
branch: feat/story-stats-engine
parent: epic-streak-engine
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:05:18.286Z"
depends_on: [story-streak-engine-spec]
worktree_path: /home/arggon/Projects/racha-story-stats-engine
---
<!--
  Placement (v0): tasks/core/epic-streak-engine/story-stats-engine/story-stats-engine.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# "Estadísticas agregadas: mes/año y % de cumplimiento semanal"

## Context

Issue #1: completar estadísticas del motor sobre la base de la spec
(story-streak-engine-spec). Implementación en `streaks.rs` (puro) + render en CLI.

## Acceptance

- [x] `racha stats` muestra total de checks del mes corriente y del año por hábito
- [x] % de cumplimiento de la semana corriente (checks de la semana / días transcurridos de la semana)
- [x] Tests unitarios de cada regla nueva + integration del output
- [x] docs/FORMAT.md actualizado en el mismo PR

Closes #1.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
