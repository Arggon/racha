# racha

Tracker de hábitos con rachas — CLI en Rust, datos locales en JSON, cero nube.

```console
$ racha add meditar
hábito agregado: meditar
$ racha check meditar
✓ meditar — racha actual: 1 día(s)
$ racha list
meditar — racha actual: 1 día(s)
$ racha stats
meditar
  racha actual : 1 día(s)
  mejor racha  : 1 día(s)
  total checks : 1
  semana (L..D): ······✓
```

## Recordatorios: `racha remind`

Lista los hábitos sin check hoy y dispara una notificación nativa de
escritorio por cada uno (canal estándar `org.freedesktop.Notifications` vía
[notify-rust](https://crates.io/crates/notify-rust) / zbus, sin dependencias C).

```console
$ racha remind
vencido: meditar
vencido: leer
```

Comportamiento:

| Caso                          | Salida                                             | Exit |
| ----------------------------- | -------------------------------------------------- | ---- |
| Hábitos vencidos              | lista + una notificación nativa por hábito         | 0    |
| Todos con check hoy           | `nada vencido hoy — todos los hábitos con check ✓` | 0    |
| Sin hábitos                   | `sin hábitos todavía — probá: racha add meditar`   | 0    |
| Sin bus de sesión D-Bus       | error en stderr indicando el hábito                | **2** |

El exit code 2 es el camino degradado pensado para el timer de systemd: el
comando falla en claro si no hay daemon de notificaciones, sin corromper datos.

## Stack

| Pieza     | Elección                             |
| --------- | ------------------------------------ |
| Lenguaje  | Rust 1.98.1 (estable al 2026-09-13)  |
| CLI       | clap 4.6 (derive)                    |
| Datos     | serde + serde_json, ledger local     |
| Fechas    | chrono 0.4                           |
| Tests     | `cargo test` + assert_cmd (binario)  |

## Motivación

Las apps de hábitos son todos subscriptions con nube que no necesitamos.
`racha` es un binario local: el ledger es un JSON legible en
`~/.local/share/racha/ledger.json`, funciona offline, y se respalda con `cp`.
Filosofía Unix: un comando, una salida, datos tuyos.

Roadmap vivo (issues): motor de rachas y estadísticas, notificaciones nativas
de escritorio (Omarchy/Hyprland), recordatorios con systemd user timer, vista
web de solo lectura, export/import.

## Desarrollo

```bash
cargo test          # unit + integración (assert_cmd contra el binario)
cargo clippy        # lint
cargo build         # debug; --release para uso diario
```

Formato del ledger y semántica de rachas: [docs/FORMAT.md](docs/FORMAT.md).
Decisiones informales del diseño: [docs/DECISIONS.md](docs/DECISIONS.md).
