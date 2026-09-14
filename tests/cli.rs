use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn racha(data: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("racha").expect("binario racha");
    cmd.env("RACHA_DATA_DIR", data.path());
    cmd
}

#[test]
fn add_check_stats_full_cycle() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["add", "meditar"])
        .assert()
        .success()
        .stdout(predicates::str::contains("hábito agregado"));

    racha(&data)
        .args(["check", "meditar"])
        .assert()
        .success()
        .stdout(predicates::str::contains("racha actual: 1"));

    racha(&data)
        .args(["stats", "meditar"])
        .assert()
        .success()
        .stdout(predicates::str::contains("mejor racha  : 1 día(s)"));
}

#[test]
fn duplicate_add_fails() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "leer"]).assert().success();
    racha(&data)
        .args(["add", "leer"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("ya existe"));
}

#[test]
fn check_missing_habit_fails() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["check", "fantasma"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("hábito inexistente"));
}

#[test]
fn double_check_is_idempotent() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "agua"]).assert().success();
    racha(&data).args(["check", "agua"]).assert().success();
    racha(&data)
        .args(["check", "agua"])
        .assert()
        .success()
        .stdout(predicates::str::contains("ya estaba registrado"));
}

#[test]
fn stats_empty_is_friendly() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["stats"])
        .assert()
        .success()
        .stdout(predicates::str::contains("sin hábitos"));
}

#[test]
fn stats_without_name_lists_all() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "a"]).assert().success();
    racha(&data).args(["add", "b"]).assert().success();
    racha(&data)
        .args(["stats"])
        .assert()
        .success()
        .stdout(predicates::str::contains("a").and(predicates::str::contains("b")));
}

#[test]
fn list_empty_is_friendly() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "sin hábitos todavía — probá: racha add meditar",
        ));
}

#[test]
fn list_shows_one_habit_per_line_with_streak() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["add", "leer"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();
    racha(&data).args(["list"]).assert().success().stdout(
        predicates::str::contains("meditar — racha actual: 1 día(s)")
            .and(predicates::str::contains("leer — racha actual: 0 día(s)")),
    );
}

#[test]
fn stats_shows_week_percentage_month_and_year() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();
    racha(&data)
        .args(["stats", "meditar"])
        .assert()
        .success()
        .stdout(
            predicates::str::contains("% semana     : ")
                .and(predicates::str::contains("mes          : 1"))
                .and(predicates::str::contains("año          : 1")),
        );
}

#[test]
fn remind_without_habits_is_friendly() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["remind"])
        .assert()
        .success()
        .stdout(predicates::str::contains("sin hábitos"));
}

#[test]
fn remind_with_everything_checked_is_quiet_success() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();
    racha(&data)
        .args(["remind"])
        .assert()
        .success()
        .stdout(predicates::str::contains("nada vencido hoy"));
}

#[test]
fn stats_all_habits_show_new_metrics() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "agua"]).assert().success();
    racha(&data).args(["check", "agua"]).assert().success();
    racha(&data).args(["stats"]).assert().success().stdout(
        predicates::str::contains("% semana     : ")
            .and(predicates::str::contains("mes          : 1"))
            .and(predicates::str::contains("año          : 1")),
    );
}

#[test]
fn add_empty_name_fails() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["add", ""])
        .assert()
        .failure()
        .stderr(predicates::str::contains("no puede quedar vacío"));
}

#[test]
fn add_whitespace_only_name_fails() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["add", "   "])
        .assert()
        .failure()
        .stderr(predicates::str::contains("no puede quedar vacío"));
}

#[test]
fn add_trims_name_before_persisting() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["add", "  leer  "])
        .assert()
        .success()
        .stdout(predicates::str::contains("hábito agregado: leer"));

    // El nombre trimeado es el referenciable, y no hay duplicado con espacios.
    racha(&data).args(["stats", "leer"]).assert().success();
    racha(&data)
        .args(["add", "leer"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("ya existe"));
}

#[test]
fn check_trims_name_before_lookup() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "leer"]).assert().success();
    racha(&data)
        .args(["check", "  leer  "])
        .assert()
        .success()
        .stdout(predicates::str::contains("racha actual: 1"));
}

#[test]
fn integrations_consume_streaks_engine_without_duplicating_logic() {
    // T4 del plan: la superficie `racha::streaks` es consumible desde afuera.
    use chrono::NaiveDate;
    let today = NaiveDate::from_ymd_opt(2026, 9, 13).unwrap();
    let checks = vec![today];
    assert_eq!(racha::streaks::current_streak(&checks, today), 1);
    assert_eq!(racha::streaks::week_completion(&checks, today), 14); // 1/7
    assert_eq!(racha::streaks::checks_in_month(&checks, today), 1);
}

#[test]
fn remind_without_session_bus_fails_with_exit_code_2() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "leer"]).assert().success();
    // vencido: sin check hoy y sin bus de sesión (env limpiada)
    racha(&data)
        .args(["remind"])
        // Dirección de bus inválida: simula sesión sin daemon D-Bus (zbus
        // tiene fallback a /run/user/$UID/bus, con solo env -u no alcanza).
        .env(
            "DBUS_SESSION_BUS_ADDRESS",
            "unix:path=/tmp/racha-sin-bus-inexistente",
        )
        .env_remove("XDG_RUNTIME_DIR")
        .assert()
        .failure()
        .code(2)
        .stderr(predicates::str::contains("no pude notificar"));
}

#[test]
fn web_creates_index_html_with_expected_content() {
    let data = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();

    racha(&data)
        .args(["web", "--out", out.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("vista web generada"));

    let path = out.path().join("index.html");
    assert!(path.exists());
    let html = std::fs::read_to_string(&path).unwrap();
    assert!(!html.is_empty());
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("meditar"));
    assert!(html.contains("racha actual"));
    assert!(html.contains("mejor racha"));
    assert!(html.contains("total checks"));
}

#[test]
fn web_defaults_to_racha_web_dir_relative_to_cwd() {
    let data = TempDir::new().unwrap();
    let cwd = TempDir::new().unwrap();
    racha(&data).args(["add", "leer"]).assert().success();
    racha(&data)
        .current_dir(cwd.path())
        .args(["web"])
        .assert()
        .success();
    let html = std::fs::read_to_string(cwd.path().join("racha-web/index.html")).unwrap();
    assert!(html.contains("leer"));
}

#[test]
fn web_with_empty_ledger_shows_empty_message() {
    let data = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    racha(&data)
        .args(["web", "--out", out.path().to_str().unwrap()])
        .assert()
        .success();
    let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
    assert!(html.contains("sin hábitos todavía"));
}
