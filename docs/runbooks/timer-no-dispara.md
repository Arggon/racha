# Runbook: el timer de racha no dispara

## Trigger

- Pasaron las 09:00 (o el horario configurado) y no llegó ninguna notificación de racha.
- `systemctl --user list-timers` no muestra `racha-remind.timer`, o lo muestra sin `NEXT`.

## Prerequisites

- Sesión de usuario systemd activa (`loginctl show-user $USER | grep Linger` opcional; el timer de user units no requiere linger mientras haya sesión).
- Binario instalado en `~/.local/bin/racha`.
- Unidades en `~/.config/systemd/user/racha-remind.{service,timer}`.

## Diagnosis

1. ¿El timer está cargado y agendado?

   ```bash
   systemctl --user list-timers --no-pager | grep racha
   ```

   Esperado: una fila con `racha-remind.timer`, columna `NEXT` con la próxima fecha/hora, `ACTIVATES racha-remind.service`. Sin fila → ir a Mitigation 1.

2. ¿Está enabled?

   ```bash
   systemctl --user is-enabled racha-remind.timer
   ```

   Esperado: `enabled`. Si es `disabled`/`not-found` → Mitigation 1 o 2.

3. ¿El unit editado a mano está stale? (síntoma: cambiaste `OnCalendar` y no surte efecto)

   ```bash
   systemctl --user show racha-remind.timer -p LastTriggerUSec -p TimersCalendar
   systemctl --user status racha-remind.timer --no-pager
   ```

   Si el calendar mostrado no coincide con el archivo → Mitigation 3.

4. ¿El service asociado falla al dispararse? (timer dispara pero no hay efecto visible)

   ```bash
   journalctl --user -u racha-remind.service --no-pager -n 30
   ```

   Buscar `Starting`/`Finished` (disparo OK, problema de notificaciones → ver
   `notificaciones-no-aparecen.md`) o mensajes de error (binario ausente, exit 2).

## Mitigation

1. Timer nunca instalado o no cargado:

   ```bash
   install -Dm755 target/release/racha ~/.local/bin/racha
   cp packaging/systemd/user/* ~/.config/systemd/user/
   systemctl --user daemon-reload
   systemctl --user enable --now racha-remind.timer
   ```

   Esperado: symlink creado y fila en `list-timers`.

2. Timer instalado pero disabled:

   ```bash
   systemctl --user enable --now racha-remind.timer
   ```

3. Configuración editada y stale:

   ```bash
   systemctl --user daemon-reload && systemctl --user restart racha-remind.timer
   ```

4. Máquina apagada a la hora del disparo: `Persistent=true` en
   `racha-remind.timer` recupera el disparo al arrancar la sesión. Verificar:

   ```bash
   grep Persistent ~/.config/systemd/user/racha-remind.timer
   journalctl --user -u racha-remind.service --no-pager -n 10
   ```

   Si el catch-up ocurrió, journal muestra `Starting` con timestamp de login.

5. Verificar que el timer está en el grafo de arranque:

   ```bash
   ls -l ~/.config/systemd/user/timers.target.wants/ | grep racha
   ```

## Escalation

Si tras `daemon-reload` + `restart` el timer sigue sin aparecer en
`list-timers`, capturar:

- `systemctl --user status racha-remind.timer racha-remind.service --no-pager`
- `journalctl --user -n 50 --no-pager`
- salida de `systemd-analyze calendar '*-*-* 09:00:00'`

y filear un bug en el tracker (`arggon create bug ... --parent
story-systemd-timer`) con esa evidencia.

## Rollback

Deshabilitar y desinstalar sin tocar datos:

```bash
systemctl --user disable --now racha-remind.timer
rm ~/.config/systemd/user/racha-remind.{service,timer}
systemctl --user daemon-reload
```

El ledger (`~/.local/share/racha/ledger.json`) no se modifica.
