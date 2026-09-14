---
type: story
status: todo
id: story-stats-engine
title: "Estadísticas agregadas: mes/año y % de cumplimiento semanal"
parent: epic-streak-engine
labels: []
created: "2026-09-14"
updated: "2026-09-14"
depends_on: [story-streak-engine-spec]
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

- [ ] `racha stats` muestra total de checks del mes corriente y del año por hábito
- [ ] % de cumplimiento de la semana corriente (checks de la semana / días transcurridos de la semana)
- [ ] Tests unitarios de cada regla nueva + integration del output
- [ ] docs/FORMAT.md actualizado en el mismo PR

Closes #1.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
