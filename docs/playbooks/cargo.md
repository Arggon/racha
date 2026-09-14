---
playbook_id: cargo
version: 1.98.1
researched: 2026-09-14
status: current
---

# cargo playbook (racha)

## Setup

- Cargo viaja con el toolchain de rustup: **cargo 1.98.1**, empaquetado con
  rustc 1.98.1 (estable al 2026-09-13). Fuente:
  https://blog.rust-lang.org/releases/latest/ (accedido 2026-09-13).
- Cargo 1.98 parsea manifiestos TOML v1.1 — no usarlo aún en este repo:
  sube el MSRV de desarrollo de quien compile.
  Fuente: https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html (accedido
  2026-09-13).

## Conventions

- Dependencias se agregan/editan con `cargo add <crate> --features ...`
  (nunca a mano sin actualizar Cargo.lock); `Cargo.lock` se commitea (bin).
- Cargo 1.98 reorganiza el layout de artefactos intermedios de build
  (prepara GC de artefactos y mejoras de cross-compile) — no confiar en
  rutas internas de `target/`. Fuente:
  https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html (accedido
  2026-09-13).
- Las features default son API: agregar features al default puede romper
  consumidores con `default-features = false`. Fuente:
  https://slint.dev/blog/rust-adding-default-cargo-feature (accedido
  2026-09-13).
- En racha: deps mínimas — clap (derive+env), serde (derive), serde_json,
  chrono (serde+clock); dev: assert_cmd, predicates, tempfile. Ninguna
  feature que no se use.

## Testing

- `cargo test` local antes de cada push; los tests de integración toman el
  binario de cargo (`Command::cargo_bin("racha")`), nunca rutas manuales.
- Bump de versiones: actualizar requirement + `cargo update` + suite
  completa en el mismo PR. Fuente:
  https://users.rust-lang.org/t/best-practices-for-bumping-versions-in-cargo-toml/111565
  (accedido 2026-09-13).

## Security

- `cargo audit` al agregar dependencias nuevas (política del repo; hoy no
  corre en CI por falta de runner).
- Commitear Cargo.lock hace reproducibles los builds de seguridad.

## Upgrade policy

- Re-research cuando `arggon playbook status` marque stale o en cada release
  de Rust que traiga cambios de cargo relevantes (cada 6 semanas).
- Actualizar Setup/Conventions con fuentes re-fechadas, probar con la suite
  completa y cerrar con `playbook refresh cargo --version <v>`.
