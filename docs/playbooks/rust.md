---
playbook_id: rust
version: 1.98.1
researched: 2026-09-14
status: current
---

# rust playbook (racha)

## Setup

- Toolchain vía rustup (canal `stable`, perfil `minimal` + componente
  `clippy`); el repo no pincha rustc: todos los builds asumen el stable del
  día. Verificar: `rustc --version` ≥ 1.98.1.
- Estado estable al 2026-09-13: **1.98.1** (parche del 3-sep-2026 que
  corrige una *miscompilation* en la generación de vtables de trait objects
  de 1.98.0 — actualizar sí o sí si se está en 1.98.0).
  Fuente: https://blog.rust-lang.org/releases/latest/ (accedido 2026-09-13).
- Calendario: 1.99.0 beta el 1-oct-2026; ciclo de 6 semanas.
  Fuente: https://releases.rs/ (accedido 2026-09-13).

## Conventions

- Edition 2024 (default de `cargo new` con el toolchain actual): `set_var`
  es unsafe, `unsafe_op_in_unsafe_fn`, `let ... else` disponible.
  Fuente: https://releases.rs/docs/1.98.0/ y edición 2024
  (accedido 2026-09-13).
- 1.98 permite acortar el lifetime de `&mut` en unsize-coercions incluso en
  posición invariante — simplifica wrappers sobre `&mut dyn Trait`.
  Fuente: https://releases.rs/docs/1.98.0/ (accedido 2026-09-13).
- v0 symbol mangling es default desde 1.98 → backtraces legibles sin flags.
  Fuente: https://blog.rust-lang.org/releases/latest/ (accedido 2026-09-13).
- En racha: errores de negocio como `String` legible con exit code 1 (CLI
  pequeño), fechas locales `NaiveDate` (docs/FORMAT.md), `unsafe` prohibido
  salvo la env-var de test documentada en `storage.rs`.

## Testing

- `cargo test` corre unit tests (módulos `#[cfg(test)]`) + integration
  (`tests/`). En racha: unit para el motor puro (`streaks`), integration con
  `assert_cmd` contra el binario y data dir aislado (`RACHA_DATA_DIR`).
- Gates de merge: `cargo test` verde + `cargo clippy --all-targets` sin
  warnings + `cargo fmt --check`.

## Security

- Instalar siempre el último point release del stable: 1.98.1 corrige una
  miscompilation de vtables de 1.98.0 (correctness bug con implicancias de
  seguridad potencial, no solo panics). Fuente:
  https://blog.rust-lang.org/releases/latest/ (accedido 2026-09-13).
- Rust no publica advisories de crates: para eso, `cargo audit` es política
  del repo cuando se agreguen dependencias nuevas (hoy: 5 directas).

## Upgrade policy

- Re-research cuando `arggon playbook status` marque stale (90 días) o al
  salir un release con cambios que nos afecten (cada 6 semanas hay uno).
- Elegir el último point release del stable; actualizar Setup/Conventions/
  Security con fuentes re-fechadas y correr `playbook refresh rust
  --version <v>`. Probar el upgrade en un PR con la suite completa antes de
  refrescar el playbook.
