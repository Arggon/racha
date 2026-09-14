---
plan_id: streak-engine-001
title: Plan for Motor de rachas: semántica y estadísticas
spec: docs/specs/spec-streak-engine-001.md
status: implemented
created: 2026-09-14
---

# Plan: Motor de rachas: semántica y estadísticas (streak-engine-001)

Derived from `docs/specs/spec-streak-engine-001.md`. Each task carries a
verifiable acceptance criterion and links back to the spec.

## Tasks

### T1: Formalizar la semántica vigente en tests (ya implementada en FASE A)

- Auditoría de `streaks.rs` contra la spec: cada invariant con su test
  (racha actual con/sin check de hoy, mejor racha con desorden+duplicados,
  fronteras de mes, semana lunes..domingo).
- **Acceptance:** `cargo test` pasa y cada regla de la spec tiene un test
  con nombre que la cita.

### T2: Métricas nuevas del motor (puras)

- `checks_in_month(checks, today)`, `checks_in_year(checks, today)`,
  `week_completion(checks, today) -> u32` (% de la semana corriente,
  redondeo hacia abajo).
- **Acceptance:** tests unitarios de cada función nueva incluyendo
  fronteras (semana empezando lunes, mes de 28/30/31 días, 1 de enero).

### T3: Render de stats

- `racha stats [nombre]` agrega `% semana`, `mes`, `año` al output; la
  vista de todos los hábitos incluye las mismas columnas.
- **Acceptance:** tests de integración (assert_cmd) sobre el output exacto;
  docs/FORMAT.md actualizado en el mismo PR.

### T4: API interna para integraciones

- `pub use racha::streaks` documentado como superficie para `remind`,
  `web` y `export` (lib.rs re-exporta; sin lógica duplicada en main.rs).
- **Acceptance:** un test de integración que consuma `racha::streaks` desde
  `tests/` y compile sin feature flags.

### T5: Flip de estado

- Spec y plan pasan a `implemented` en el mismo PR que aterrice T1-T4.
- **Acceptance:** frontmatter de spec/plan con `status: implemented` y
  `arggon spec validate` ok.
