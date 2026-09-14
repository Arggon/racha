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

### 2026-09-14 @Arggon
Implementado y mergeado en PR https://github.com/Arggon/racha/pull/11 (squash, 30aa7f1). Motor puro (streaks.rs): checks_in_month, checks_in_year, week_completion (0..100, floor, hoy incluido) con 12 tests unitarios de frontera (semana desde lunes, mes de 28/30/31, 1 de enero, día sin checks). main.rs: 'racha stats [nombre]' agrega '% semana', 'mes', 'año' alineado al formato existente. tests/cli.rs: +5 tests de integración (output nuevo con RACHA_DATA_DIR+TempDir y consumo de racha::streaks, T4). docs/FORMAT.md documenta las métricas; spec/plan streak-engine-001 → implemented (T5), arggon spec validate ok. Gates: cargo test 35 pasando (20 unit + 15 integration), clippy --all-targets 0 warnings, fmt --check limpio. Evidencia start: {"ok":true,"command":"start","worktreePath":"/home/arggon/Projects/racha-story-stats-engine","postStart":{"command":"cargo check","ok":true}}. Asperezas: (1) cargo fmt reformateó storage.rs preexistente — reverteo para mantener el diff enfocado (el árbol principal no estaba fmt-limpio antes de esta PR); (2) conflicto de rebase en tests/cli.rs con tests de  y validación de  de otros agentes — resuelto preservando ambos conjuntos; (3) 2026-01-01 cae jueves y su semana calendario arranca el 29-dic-2025: la semana de week_completion cruza el límite de año (documentado en test).
