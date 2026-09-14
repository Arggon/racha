# Decisiones (informal)

Bitácora de decisiones de diseño, en orden. Formato corto a propósito —
cuando el proyecto adopte metodología formal, estas migran a ADRs.

## 1. Rust como lenguaje (2026-09-13)

Binario único, sin runtime, arranca en milisegundos (importante para un CLI
que se invoca desde timers de systemd), y el sistema de tipos ayuda a que el
motor de rachas no tenga off-by-ones. Rust estable al momento del arranque:
**1.98.1** (publicada 2026-09-03, blog.rust-lang.org).

Alternativas descartadas: Go (binario también, pero queríamos probar el
ecosistema Rust en este dominio), Python (arranque lento + distribución de
dependencias dolorosa para un tool que corre en timers).

## 2. Storage: un JSON local, no SQLite (2026-09-13)

El ledger entero vive en `~/.local/share/racha/ledger.json`. Volumen esperado:
decenas de hábitos × miles de checks = ~cientos de KB en el peor caso
decenal — trivial para cargar/serializar entero en memoria. A cambio:
archivo legible por humanos, backup = `cp`, sync con dotfiles, cero
dependencias C (sqlite3 via bindings complica el build para algo de este
tamaño). Si algún día hay concurrencia multi-proceso real, migrar a SQLite
es una decisión reversible documentada.

## 3. CLI-first, sin GUI (2026-09-13)

El uso primario es `racha check` después de hacer el hábito — un comando de
dos palabras es más rápido que abrir cualquier app. La vista web (roadmap)
es de solo lectura: reporte, no interfaz de escritura. La entrada de datos
queda en el terminal.

## 4. Fechas locales, no UTC (2026-09-13)

Un check es "hice el hábito hoy" — hoy según el usuario, no según UTC.
Se guardan fechas (`YYYY-MM-DD`), no timestamps: la zona horaria de un check
histórico no importa, y evita bugs de "me contó para ayer" por offsets.
Semántica de racha en docs/FORMAT.md.
