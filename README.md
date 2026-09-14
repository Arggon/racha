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

## Recordatorios programados (systemd user timer)

Para que el recordatorio llegue solo todos los días a las 09:00, instalá el
binario y un user timer de systemd (4 comandos):

```bash
install -Dm755 target/release/racha ~/.local/bin/racha
mkdir -p ~/.config/systemd/user && cp packaging/systemd/user/* ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now racha-remind.timer
```

- `racha-remind.service` (oneshot) corre `racha remind`; se ordena después de
  `graphical-session.target` para que el bus de sesión ya exista.
- `racha-remind.timer` dispara con `OnCalendar=*-*-* 09:00:00` (editá la hora
  en el archivo, hay ejemplos) y `Persistent=true`: si la máquina estaba
  apagada a las 09:00, recupera el disparo al arrancar la sesión.

Diagnóstico operativo: [docs/runbooks/timer-no-dispara.md](docs/runbooks/timer-no-dispara.md)
y [docs/runbooks/notificaciones-no-aparecen.md](docs/runbooks/notificaciones-no-aparecen.md).
## Vista web: `racha web`

Genera una página HTML estática de SOLO lectura con las estadísticas de todos
los hábitos: racha actual, mejor racha, total, % de la semana, totales del mes
y del año, la vista semanal (✓/·) y los últimos 14 días por hábito.

```console
$ racha web [--out <dir>]
vista web generada: ./racha-web/index.html
```

- Sin servidor y sin estado: el resultado es un único `index.html` abrible
  directo con `file://` (default: `racha-web/index.html` relativo a cwd).
- Escribir en el ledger sigue siendo territorio del CLI (`racha check`); la
  página es una foto del momento en que se generó.
- Los nombres de hábito se escapan HTML: un hábito llamado `<script>` no puede
  inyectar markup.

Para refrescar los datos, volvé a correr `racha web` y recargá la página.
## Portabilidad: `racha export` / `racha import`

El ledger es tuyo: exportalo a JSON canónico o CSV, y volvé a importarlo en
cualquier máquina (o después de un desastre — ver el
[runbook de restore](docs/runbooks/restore-de-datos.md)).

```console
$ racha export --format json --out backup.json   # default: stdout
$ racha export --format csv                      # a stdout
habit,date
meditar,2026-09-12
meditar,2026-09-13
$ racha import backup.json
importado: backup.json (2 hábito(s) en el ledger)
```

- **JSON canónico**: `{"version":1,"habits":[{"name":...,"created":...,"checks":[...]}]}`
  — el mismo shape del ledger más el campo `version` (ver [docs/FORMAT.md](docs/FORMAT.md)).
- **CSV**: header `habit,date`, una fila por check, ordenado. (No viaja
  `created`; en el import se toma la primera check importada.)
- **Import validado**: detecta el formato por contenido (no por extensión);
  valida fechas ISO, nombres no vacíos y ausencia de checks duplicados ANTES
  de escribir. Ante cualquier error: mensaje claro, exit 1, y el ledger
  existente queda intocado.
- **Idempotente**: importar dos veces el mismo archivo no duplica nada
  (dedup por nombre+fecha; el merge con un ledger existente es aditivo).
- **Escritura atómica**: todo guardado del ledger es temp-file + rename
  (`save_atomic`): un crash a mitad de escritura nunca deja un ledger truncado.

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
