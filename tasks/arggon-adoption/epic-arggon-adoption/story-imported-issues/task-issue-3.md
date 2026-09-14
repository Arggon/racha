---
type: task
status: done
id: task-issue-3
title: "issue #3: Recordatorios programados (systemd user timer)"
assignee: Arggon
parent: story-imported-issues
labels: [enhancement]
created: "2026-09-14"
updated: "2026-09-14"
issue: 3
---
Recordatorio diario programado con systemd **user** timer + service (unidades instaladas por el proyecto, `systemctl --user`).

- [ ] `racha remind`: consulta vencimientos del día y dispara notificaciones nativas
- [ ] unidades systemd (service oneshot + timer) en el repo, target de instalación documentado
- [ ] runbooks: timer no dispara, notificaciones no aparecen (Wayland/DBus), restore de datos

Criterio de aceptación: timer instalado dispara `racha remind` y produce notificación visible; runbooks probados.
> imported from issue #3

### 2026-09-14 @Arggon
Implementado por story-systemd-timer (PR #13): unidades instaladas, timer activo, disparo real con journalctl de evidencia, runbooks timer-no-dispara + notificaciones-no-aparecen.
