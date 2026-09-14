---
playbook_id: clippy
version: 1.98
researched: 2026-09-14
status: current
---

# clippy playbook (racha)

## Setup

- Clippy es un componente de rustup y viaja versionado con el compilador:
  **clippy 1.98** con Rust 1.98.x. Instalar con
  `rustup component add clippy` (necesario si instalaste con perfil
  `minimal`, como este repo).

## Conventions

- Estado del release (accedido 2026-09-13): clippy 1.98 **no introduce
  lints nuevos significativos; va concentrado en bug fixes** (rust-clippy
  #15086). Fuente:
  https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md
- Gate del repo: `cargo clippy --all-targets` **cero warnings** — se arregla
  en el mismo PR, nunca se silencia sin comentario de por qué. Hoy el repo
  tiene 0 `allow`/`expect`.
- Los lints se extienden entre releases (proyectos reportan warnings nuevos
  en código previamente limpio): al subir versión de Rust, resolver el ruido
  de clippy en el PR del upgrade. Fuente:
  https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md
- Documentación de lints versionada por release:
  https://rust-lang.github.io/rust-clippy/ (versión rust-1.98.0, accedido
  2026-09-13).

## Testing

- Clippy no reemplaza tests: es un gate estático aparte. Orden del gate
  local: `cargo fmt --check` → `cargo clippy --all-targets` → `cargo test`.

## Security

- Clippy no trackea advisories. Para vulnerabilidades en deps, `cargo audit`
  (ver playbook de cargo).

## Upgrade policy

- Clippy sube de versión con cada release de Rust (cada 6 semanas);
  re-research cuando `arggon playbook status` marque stale o en el PR que
  suba el toolchain. Revisar la sección "Moves and Deprecations" del
  changelog del release antes de actualizar este archivo.
