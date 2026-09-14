---
type: story
status: in_progress
id: story-streak-engine-spec
title: Spec + plan del motor de rachas
assignee: Arggon
branch: feat/story-streak-engine-spec
parent: epic-streak-engine
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T01:53:27.744Z"
worktree_path: /home/arggon/Projects/racha-story-streak-engine-spec
---
<!--
  Placement (v0): tasks/core/epic-streak-engine/story-streak-engine-spec/story-streak-engine-spec.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# Spec + plan del motor de rachas

## Context

El motor existe en versión mínima (FASE A: racha actual, mejor racha, vista semanal).
Antes de extenderlo (stats agregadas, API para integraciones) la metodología exige
spec + plan. Los docs que nacen acá gobiernan el resto de la iniciativa core.

## Acceptance

- [x] docs/specs/ con la spec del motor: propósito, sinopsis, invariants (un check por día, unicidad de nombre, fechas locales, ledger nunca corrupto) y acceptance criteria verificables
- [x] docs/plans/ con el plan ordenado de tareas, cada una con criterio de verificación
- [x] `arggon spec validate` ok
- [x] Spec/plan en estado `proposed` (flip a `implemented` en el PR de stats-engine, tarea T5 del plan)

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.

### 2026-09-14 @Arggon
Spec + plan publicados (PR #7, merge squash f406cb5): docs/specs/spec-streak-engine-001.md + docs/plans/plan-streak-engine-001.md. Nota de proceso: arggon start --worktree pusheó la branch desde el HEAD local incluyendo commits de master sin pushear; el squash merge divergió el master local (resuelto con reset a origin — contenido idéntico + spec). Lección: pushear master antes de start --worktree.
