---
exploration_id: desktop-notifications-001
title: Canal de notificaciones nativas de escritorio
status: resolved
created: 2026-09-14
---

# Exploration: desktop-notifications (desktop-notifications-001)

Cómo debe `racha` emitir notificaciones nativas de escritorio (recordatorios,
confirmaciones de racha) en el entorno actual: Omarchy (Hyprland + Quickshell)
en Wayland, con un daemon que ya registra `org.freedesktop.Notifications` en
el bus de sesión (verificado: `busctl --user status org.freedesktop.Notifications`
responde, PID 1355). Contexto: CLI en Rust, binario único, preferencia por
cero dependencias C (ver docs/DECISIONS.md §1-2).

## Candidates

1. **notify-rust** (crate Rust puro, backend zbus por defecto).
2. **Shell out a `notify-send`** (libnotify 0.8.8, instalada en esta máquina).
3. **Canal propio del compositor / Quickshell** (invocar directamente el shell
   de Omarchy, fuera del estándar freedesktop).

## Criteria

Ponderados de mayor a menor:

1. **Peso de dependencias / fricción de packaging** — racha quiere binario
   único sin C; las deps de Rust puras son tolerables, las C no.
2. **Compatibilidad Wayland/DBus** — debe funcionar sobre el estándar
   `org.freedesktop.Notifications`; nada acoplado a un shell concreto.
3. **Robustez de errores** — fallar suave (no romper el comando principal) y
   con mensajes accionables si no hay daemon.
4. **Features del protocolo** — urgencia, acciones, expiración: lo mínimo
   útil para "recordatorio de hábito".
5. **Mantenimiento** — proyecto vivo, releases recientes.

## Findings

- **notify-rust 4.18.0** (crates.io, publicada 2026-06-16): backend por
  defecto **z** = zbus (D-Bus 100% Rust, sin libdbus); features `d`/`d_vendored`
  cambian al crate `dbus` (C). MSRV 1.89.0 (nuestro toolchain: 1.98.1). Licencia
  MIT OR Apache-2.0. ~14.2M downloads, 83 versiones.
  (source: https://crates.io/crates/notify-rust, 2026-09-13)
- **zbus 5.19.0** (2026-08-09): D-Bus puro Rust sobre sockets Unix; arrastra
  el ecosistema async de smol (async-io, nix, etc.) — unas ~25 crates
  transitivas. Compila sin dependencias del sistema.
  (source: https://crates.io/crates/zbus, 2026-09-13)
- notify-rust se mantiene activo: badge "maintained" vigente, Renovate + CI en
  GitHub, 20 issues / 6 PRs abiertos, ~1.4k stars.
  (source: https://github.com/hoodie/notify-rust, 2026-09-13)
- **libnotify 0.8.8** (2026-01-09): serie 0.8.x con cadencia estable
  (0.8.4 feb 2025 → 0.8.8 ene 2026). `notify-send 0.8.8` está instalado en
  esta máquina. Envolverlo con `std::process::Command` no agrega dependencias
  de compilación, pero hace de la notificación una dependencia **de runtime**
  del paquete del sistema (`libnotify` no es garantizable en toda máquina
  objetivo del usuario de racha).
  (source: https://gitlab.gnome.org/GNOME/libnotify/-/tags, 2026-09-13)
- **Omarchy**: el shell Quickshell registra `org.freedesktop.Notifications`
  en el bus de sesión; cualquier cliente DBus estándar (notify-send o
  notify-rust) le llega. Un canal propietario del compositor acoplaría racha
  a Omarchy y rompería en cualquier otro WM/de.
  (source: verificación local `busctl --user status org.freedesktop.Notifications`, 2026-09-13)
- **Spike notify-send** (2026-09-13): `notify-send -a racha "racha exploración"
  "spike notify-send — canal DBus nativo"` mostró la notificación en pantalla
  (evidencia: `/tmp/racha-evidence/exploracion-spike-notify-send.png`).
- **Spike notify-rust** (2026-09-13): proyecto mínimo en /tmp con
  `Notification::new().summary(...).body(...).show()` compiló y mostró la
  notificación con cargo 1.98.1 (evidencia:
  `/tmp/racha-evidence/exploracion-spike-notify-rust.png`). Compile time del
  spike (debug, build completo desde cero): 15.22s con cargo 1.98.1 — el
  árbol de zbus se compila una sola vez y queda cacheado, aceptable.

## Recommendation

**notify-rust (backend zbus por defecto)**. Gana en el criterio 1/2 combinado:
dependencia 100% Rust (sin libnotify/libdbus en runtime, coherente con las
decisiones de packaging del proyecto), y habla el estándar
`org.freedesktop.Notifications` directamente, así que funciona con el shell
Quickshell de Omarchy y con cualquier otro daemon. `notify-send` pierde por
convertir una dependencia de compilación en una de runtime no controlada
(aunque sirve como fallback documentado); el canal del compositor pierde por
acoplamiento a Omarchy. El costo (~25 crates transitivas de zbus) es el precio
de no depender de C, y el spike confirma que compila y funciona tal cual.

## Decision

ADR: [docs/adr/0001-usar-notify-rust-para-notificaciones.md](../adr/0001-usar-notify-rust-para-notificaciones.md) (Proposed).
