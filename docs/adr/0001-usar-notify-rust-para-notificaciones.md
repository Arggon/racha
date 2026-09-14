# ADR 0001: Usar notify-rust para notificaciones nativas de escritorio

- **Status:** Proposed
- **Date:** 2026-09-13
- **Exploration:** [docs/explorations/exploration-desktop-notifications-001.md](../explorations/exploration-desktop-notifications-001.md)

## Context

`racha` necesita emitir notificaciones nativas de escritorio (recordatorios de
hábitos, confirmaciones). Entorno objetivo: Omarchy (Hyprland + Quickshell) en
Wayland, donde el shell Quickshell registra el estándar
`org.freedesktop.Notifications` en el bus de sesión (verificado en vivo:
`busctl --user status org.freedesktop.Notifications` responde). El proyecto es
un CLI en Rust con binario único y preferencia documentada por cero
dependencias C (docs/DECISIONS.md §1-2).

Candidatos: la crate `notify-rust`, shelling out a `notify-send` (libnotify),
o un canal propio del compositor/Quickshell.

## Decision

Usar la crate **notify-rust 4.18.0** con su backend por defecto **zbus**
(D-Bus 100% Rust, sin libdbus) para enviar notificaciones
`org.freedesktop.Notifications`.

## Consequences

- **Positivas**
  - Dependencia de compilación 100% Rust: sin libnotify/libdbus en runtime,
    coherente con la decisión de binario único sin C.
  - Habla el estándar freedesktop: funciona con Quickshell (Omarchy) y con
    cualquier otro daemon de notificaciones. Soporta urgencia y acciones
    (features del protocolo) desde la API.
  - Proyecto activo: 4.18.0 publicada 2026-06-16, MSRV 1.89.0 (toolchain
    local 1.98.1), mantenido según badges y CI del repo.
  - Spike verificado en esta máquina: compila (15.2s debug desde cero) y la
    notificación aparece en pantalla (evidencia:
    `/tmp/racha-evidence/exploracion-spike-notify-rust.png`).
- **Negativas / riesgos**
  - zbus arrastra ~25 crates transitivas del ecosistema async de smol: más
    superficie de compile-time que una llamada a proceso externo.
  - Si no hay daemon de notificaciones, `show()` devuelve error: el caller
    debe degradar suave (log, no fallar el comando principal).
- **Neutras**
  - La notificación es fire-and-forget; no mantenemos state del daemon.

## Alternatives considered

1. **Shell out a `notify-send` (libnotify 0.8.8)** — cero dependencias de
   compilación y ya instalado aquí, pero convierte la notificación en una
   dependencia de runtime del sistema no garantizable para cualquier usuario
   de racha; el manejo de errores queda limitado al exit code. Se documenta
   como fallback manual si algún día zbus da problemas de build.
2. **Canal propio del compositor (Quickshell/Omarchy)** — descartado:
   acopla racha a un shell concreto y rompe en cualquier otro entorno;
   el estándar freedesktop ya cubre el caso.

## Status lifecycle

Proposed al abrir el PR; **Accepted** al merge (metodología arggon:
Proposed → Accepted, nunca reescribir, superseder para cambiar).
