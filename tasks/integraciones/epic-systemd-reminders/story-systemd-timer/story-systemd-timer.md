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
