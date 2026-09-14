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

- [ ] docs/specs/ con la spec del motor: propósito, sinopsis, invariants (un check por día, unicidad de nombre, fechas locales, ledger nunca corrupto) y acceptance criteria verificables
- [ ] docs/plans/ con el plan ordenado de tareas, cada una con criterio de verificación
- [ ] `arggon spec validate` ok
- [ ] Spec/plan en estado `drafted` (flip a `implemented` en el PR que aterrice la feature)

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.
