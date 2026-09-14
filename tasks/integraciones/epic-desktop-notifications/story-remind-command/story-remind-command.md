---
type: story
status: in_progress
id: story-remind-command
title: racha remind — vencimientos del día y notificación nativa
assignee: Arggon
branch: feat/story-remind-command
parent: epic-desktop-notifications
labels: []
created: "2026-09-14"
updated: "2026-09-14"
claimed_at: "2026-09-14T02:05:19.918Z"
depends_on: [story-notify-exploration]
worktree_path: /home/arggon/Projects/racha-story-remind-command
---
<!--
  Placement (v0): tasks/integraciones/epic-desktop-notifications/story-remind-command/story-remind-command.md (story index; required).
  parent MUST be the epic id. Optional style prefixes (e.g. story-) are not type discriminators.
-->

# racha remind — vencimientos del día y notificación nativa

## Context

Issue #2: `racha remind` consulta qué hábitos no tienen check hoy y dispara una
notificación nativa por cada uno, usando el canal decidido en el ADR-0001
(story-notify-exploration). Es el comando que el timer de systemd va a invocar.

## Acceptance

- [x] `racha remind` lista vencidos (sin check hoy) y dispara una notificación nativa por hábito con el canal del ADR
- [x] Sin hábitos vencidos: mensaje claro, sin notificaciones, exit 0
- [x] Sin sesión gráfica/DBus disponible: comportamiento degradado claro y exit code documentado
- [x] Notificación real visible en Omarchy verificada (captura adjunta en la exploración o comentario del story)
- [x] Tests de la lógica de vencimientos (pura) + integration del flujo

Closes #2.

## Notes

- Story de la estructura multi-iniciativa (core/integraciones); ver docs/convention.md.

### 2026-09-14 @Arggon
Implementado y mergeado en PR #12 (squash). Comportamiento: con vencidos lista y notifica uno por uno (notify-rust 4.18.0/zbus, summary 'racha', body 'te falta checkear hoy: <habito>'), exit 0; sin vencidos imprime 'nada vencido hoy — todos los hábitos con check ✓', exit 0, sin notificaciones; sin hábitos mensaje amigable, exit 0; sin bus de sesión D-Bus error en stderr ('no pude notificar (...): ¿Hay bus de sesión D-Bus y daemon de notificaciones?'), exit code 2 (degradación documentada en README, pensada para el timer de systemd). Lógica pura en src/remind.rs (due_habits) con 5 tests unitarios + 3 de integración; suite completa 25 unit + 18 integration verde, clippy 0 warnings, fmt ok. Notificación real visible en Omarchy (Quickshell): captura /tmp/racha-evidence/remind-notificacion-real.png (866 KB) mostrando 'racha — te falta checkear hoy: leer' arriba a la derecha. Aspereza: env -u DBUS_SESSION_BUS_ADDRESS no degrada en esta máquina porque zbus hace fallback a /run/user/$UID/bus; el test de degradación usa DBUS_SESSION_BUS_ADDRESS inválida.
