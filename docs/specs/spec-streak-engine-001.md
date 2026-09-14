---
spec_id: streak-engine-001
title: Motor de rachas: semántica y estadísticas
status: implemented
created: 2026-09-14
---

# Spec: Motor de rachas: semántica y estadísticas (streak-engine-001)

## Purpose

Gobernar toda la lógica de rachas y estadísticas de `racha`: qué significa
una racha, cómo se computan las métricas, y qué garantías tiene el usuario
sobre sus datos. El motor existe en versión mínima; esta spec fija la
semántica antes de extenderlo (stats agregadas, consumos externos como
`remind` y la vista web). Toda regla nueva debe cumplir esta spec y tener
un test que la muestre.

### Invariants

1. **Un check por hábito por día**: registrar dos veces el mismo día no
   duplica ni corrompe (idempotencia).
2. **Nombres únicos por comparación exacta**: `add` rechaza duplicados.
3. **Fechas locales del usuario**, nunca UTC: un check es "hoy" para quien
   lo tilda, formato ISO `YYYY-MM-DD`.
4. **El ledger nunca se corrompe**: archivo ausente = vacío válido; archivo
   inválido = error claro sin sobreescritura; toda escritura pasa por
   `storage::save`.
5. **El motor es puro**: `streaks` recibe `&[NaiveDate]` + fecha de
   referencia; no hace I/O ni toca el ledger.

## Synopsis

```bash
racha add <nombre>          # alta de hábito (rechaza duplicados)
racha check <nombre>        # check de hoy (idempotente)
racha stats [nombre]        # racha actual, mejor, totals, % semana, mes/año
racha list                  # una línea por hábito con racha actual
racha remind                # hábitos sin check hoy → notificación nativa
racha web [--out <dir>]     # HTML estático de solo lectura
racha export --format csv|json [--out <file>]
racha import <file>         # idempotente, validado, atómico
```

### Semántica de rachas (normativa)

- **Racha actual**: corrida de días consecutivos con check terminando en
  hoy. Gracia hasta medianoche: si hoy no hay check pero sí ayer, la racha
  actual sigue siendo la corrida que termina en ayer.
- **Mejor racha**: máxima corrida consecutiva del historial completo;
  ordenar + deduplicar antes de medir; cruza meses y años.
- **Vista semanal**: semana calendario (lunes..domingo) que contiene hoy.
- **% de cumplimiento semanal** (nuevo): checks de la semana corriente /
  días transcurridos de la semana (hoy incluido), 0..100, redondeado hacia
  abajo.
- **Totales mensual/anual** (nuevo): cantidad de checks con fecha en el mes
  y el año corrientes.

## Acceptance

- [ ] Toda regla de esta spec tiene al menos un test unitario en `streaks` que la demuestra (fronteras de mes/año, gracia de medianoche, dedup, desorden)
- [ ] `racha stats` muestra racha actual, mejor, total histórico, % semana, total mes y total año por hábito
- [ ] Cambios de output documentados en docs/FORMAT.md en el mismo PR
- [ ] `cargo test` completo verde; `cargo clippy --all-targets` sin warnings
- [ ] Las integraciones (remind/web/export) consumen el motor sin duplicar lógica de rachas
