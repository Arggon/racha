# Runbook: las notificaciones de racha no aparecen

## Trigger

- El timer dispara (`journalctl --user -u racha-remind.service` muestra
  `Starting`/`Finished` o `vencido: <hábito>`) pero no se ve nada en pantalla.
- `racha remind` a mano termina con exit code 2 y error en stderr.

## Prerequisites

- Sesión gráfica Wayland activa (Omarchy: Hyprland + daemon Quickshell).
- `racha remind` con exit 0 cuando hay hábitos vencidos (el exit 2 indica
  problema de bus, no de hábitos).

## Diagnosis

1. ¿El comando mismo falla o dispara OK?

   ```bash
   ~/.local/bin/racha remind; echo "exit: $?"
   ```

   - Exit 0 + imprime `vencido: <hábito>` → el comando envió la notificación;
     el problema está del lado del daemon (ir a 3).
   - Exit 2 + error de bus en stderr → no hay bus de sesión alcanzable (ir a 2).
   - `nada vencido hoy` → no hay nada que notificar; comportamiento normal.

2. ¿Existe el bus de sesión? Los user units de systemd NO setean
   `DBUS_SESSION_BUS_ADDRESS`; notify-rust/zbus hace fallback a
   `unix:path=/run/user/$UID/bus`. Verificar ambos caminos:

   ```bash
   id -u
   ls -l /run/user/$(id -u)/bus
   DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/$(id -u)/bus ~/.local/bin/racha remind; echo "exit: $?"
   ```

   Si con la variable explícita funciona y desde el unit no, revisar que
   `XDG_RUNTIME_DIR` esté definido en el entorno del user manager
   (`systemctl --user show-environment | grep XDG_RUNTIME_DIR`).

3. ¿Hay un notification daemon corriendo? En Omarchy lo sirve el daemon
   Quickshell de la sesión:

   ```bash
   pgrep -a quickshell
   busctl --user list | grep -i Notifications
   ```

   Esperado: proceso vivo y el nombre `org.freedesktop.Notifications` en el bus.

4. Aislar el problema con `notify-send` (mismo canal, otro cliente):

   ```bash
   notify-send "test racha" "si ves esto, el canal funciona"
   ```

   - Se ve → el canal y el daemon están bien; el problema es específico de
     cómo corre el binario desde systemd (ir a Mitigation 1).
   - No se ve → el notification daemon de la sesión está caído (Mitigation 2).

## Mitigation

1. Reproducir exactamente el entorno del unit:

   ```bash
   systemd-run --user --pipe ~/.local/bin/racha remind
   ```

   Si acá falla pero a mano funciona, comparar entornos:
   `systemctl --user show-environment` vs `env | grep -Ei 'dbus|wayland|xdg'`.

2. Notification daemon caído: reiniciar la capa de sesión de Omarchy
   (recargar Hyprland/Quickshell) o reloguear la sesión Wayland. Verificar
   después con `notify-send` del paso Diagnosis 4.

3. Sin bus en `/run/user/$UID/bus`: la sesión no es systemd-managed (login
   fuera de un display manager). Reloguear normalmente; verificar que
   `systemctl --user` funcione sin error antes de re-testear.

4. Error persistente del binario: actualizar el binario instalado contra la
   versión del repo (`install -Dm755 target/release/racha ~/.local/bin/racha`)
   y reintentar `systemctl --user start racha-remind.service`.

## Escalation

Si `notify-send` funciona pero `racha remind` desde el unit falla con exit 2
aunque `/run/user/$UID/bus` existe, capturar:

- `journalctl --user -u racha-remind.service --no-pager -n 30`
- `systemctl --user show-environment`
- salida de `RUST_LOG=debug ~/.local/bin/racha remind` (si aplica)

y filear un bug en el tracker (`arggon create bug ... --parent
story-systemd-timer`) con esa evidencia.

## Rollback

Sin side-effects que revertir: las notificaciones son fire-and-forget y el
ledger no se modifica. Para silenciar los recordatorios por completo:

```bash
systemctl --user disable --now racha-remind.timer
```
