---
type: story
status: done
id: story-systemd-timer
title: Timer + service de usuario + runbooks operativos
assignee: Arggon
branch: feat/story-systemd-timer
parent: epic-systemd-reminders
labels: []
created: "2026-09-14"
updated: "2026-09-14"
depends_on: [story-remind-command]
worktree_path: /home/arggon/Projects/racha-story-systemd-timer
---
<!--
  Placement (v0): tasks/integraciones/epic-systemd-reminders/story-systemd-timer/story-systemd-timer.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# Timer + service de usuario + runbooks operativos

## Context

Issue #3: recordatorios programados con systemd **user** units versionadas en el
repo: service oneshot que corre `racha remind` + timer diario. Incluye runbooks
operativos (alguien va a tener que debuggear esto a las 8am).

## Acceptance

- [x] Unidades systemd user (service + timer) en el repo (packaging/systemd/user/)
- [x] Instalación documentada y probada en vivo: `systemctl --user enable --now racha-remind.timer` y el timer dispara `racha remind`
- [x] Runbook docs/runbooks/timer-no-dispara.md
- [x] Runbook docs/runbooks/notificaciones-no-aparecen.md (Wayland/DBus)
- [x] Evidencia del disparo real (journalctl) en comentario del story

Closes #3.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.

### 2026-09-14 @Arggon
Implementado y probado en vivo. Unidades en packaging/systemd/user/ (racha-remind.service oneshot con After=graphical-session.target; racha-remind.timer OnCalendar=*-*-* 09:00:00 + Persistent=true, WantedBy=timers.target). Instalación en 4 comandos en README (seccion Recordatorios programados). Runbooks: docs/runbooks/timer-no-dispara.md y docs/runbooks/notificaciones-no-aparecen.md (Wayland/DBus, fallback /run/user/$UID/bus, daemon Quickshell, exit 2). Evidencia del disparo real (systemctl --user start racha-remind.service, journal muestra 'vencido: meditar'): /tmp/racha-evidence/systemd-list-timers.txt, /tmp/racha-evidence/systemd-journalctl.txt y notificacion capturada en pantalla en /tmp/racha-evidence/timer-notificacion-real.png. OnCalendar elegido: *-*-* 09:00:00 (diario 09:00, con ejemplos de variante en el timer). PR #13 (squash-merged), Closes #3.
