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

// ------------------------------------------------ export/import (issue #5)

#[test]
fn json_round_trip_is_byte_identical() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["add", "leer"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();

    let export1 = TempDir::new().unwrap();
    let p1 = export1.path().join("e1.json");
    racha(&data)
        .args(["export", "--format", "json", "--out"])
        .arg(&p1)
        .assert()
        .success();

    let restored = TempDir::new().unwrap();
    racha(&restored)
        .args(["import"])
        .arg(&p1)
        .assert()
        .success();

    let export2 = TempDir::new().unwrap();
    let p2 = export2.path().join("e2.json");
    racha(&restored)
        .args(["export", "--format", "json", "--out"])
        .arg(&p2)
        .assert()
        .success();

    let a = std::fs::read_to_string(&p1).unwrap();
    let b = std::fs::read_to_string(&p2).unwrap();
    assert_eq!(a, b, "round-trip export→import→export debe ser idéntico");
    assert!(a.contains("\"version\": 1"));
    assert!(a.contains("\"name\": \"meditar\""));
}

#[test]
fn export_json_to_stdout_has_canonical_shape() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "agua"]).assert().success();
    racha(&data).args(["check", "agua"]).assert().success();
    racha(&data)
        .args(["export", "--format", "json"])
        .assert()
        .success()
        .stdout(
            predicates::str::contains("\"version\": 1")
                .and(predicates::str::contains("\"created\": "))
                .and(predicates::str::contains("\"checks\": [")),
        );
}

#[test]
fn export_csv_header_and_rows_sorted() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["add", "agua"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();
    racha(&data)
        .args(["export", "--format", "csv"])
        .assert()
        .success()
        .stdout(
            predicates::str::starts_with("habit,date\n").and(predicates::str::contains("meditar,")),
        );
}

#[test]
fn import_is_idempotent() {
    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["check", "meditar"]).assert().success();
    let export = TempDir::new().unwrap();
    let p = export.path().join("e.json");
    racha(&data)
        .args(["export", "--format", "json", "--out"])
        .arg(&p)
        .assert()
        .success();

    racha(&data).args(["import"]).arg(&p).assert().success();
    let after_first = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();
    racha(&data)
        .args(["import"])
        .arg(&p)
        .assert()
        .success()
        .stdout(predicates::str::contains("nada que importar"));
    let after_second = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();
    assert_eq!(after_first, after_second, "import doble no debe duplicar");
}

#[test]
fn import_csv_works() {
    let csv = TempDir::new().unwrap();
    let p = csv.path().join("datos.csv");
    std::fs::write(&p, "habit,date\nmeditar,2026-09-01\nmeditar,2026-09-02\n").unwrap();

    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["import"])
        .arg(&p)
        .assert()
        .success()
        .stdout(predicates::str::contains("importado"));
    racha(&data)
        .args(["stats", "meditar"])
        .assert()
        .success()
        .stdout(predicates::str::contains("total checks : 2"));
}

#[test]
fn import_invalid_date_fails_and_leaves_ledger_untouched() {
    let bad = TempDir::new().unwrap();
    let p = bad.path().join("malo.csv");
    std::fs::write(&p, "habit,date\nmeditar,2026-13-99\n").unwrap();

    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "original"]).assert().success();
    let before = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();

    racha(&data)
        .args(["import"])
        .arg(&p)
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains("fecha inválida"));

    let after = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();
    assert_eq!(
        before, after,
        "ledger debe quedar intocado tras import inválido"
    );
}

#[test]
fn import_empty_name_fails_and_leaves_ledger_untouched() {
    let bad = TempDir::new().unwrap();
    let p = bad.path().join("malo.json");
    std::fs::write(
        &p,
        r#"{"version":1,"habits":[{"name":"   ","created":"2026-09-01","checks":[]}]}"#,
    )
    .unwrap();

    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "original"]).assert().success();
    let before = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();

    racha(&data)
        .args(["import"])
        .arg(&p)
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains("vacío"));

    let after = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();
    assert_eq!(before, after);
}

#[test]
fn import_missing_file_fails_clearly() {
    let data = TempDir::new().unwrap();
    racha(&data)
        .args(["import", "/tmp/racha-no-existe-1234.json"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains("no pude leer"));
}

#[test]
fn import_merges_with_existing_habits_without_duplicating() {
    let csv = TempDir::new().unwrap();
    let p = csv.path().join("extra.csv");
    std::fs::write(&p, "habit,date\nmeditar,2026-09-01\nnuevo,2026-09-02\n").unwrap();

    let data = TempDir::new().unwrap();
    racha(&data).args(["add", "meditar"]).assert().success();
    racha(&data).args(["import"]).arg(&p).assert().success();

    // meditar no se duplicó (sigue único) y ganó el check importado.
    let ledger = std::fs::read_to_string(data.path().join("ledger.json")).unwrap();
    assert_eq!(ledger.matches("\"meditar\"").count(), 1, "{}", ledger);
    assert!(ledger.contains("nuevo"));
    racha(&data)
        .args(["stats", "meditar"])
        .assert()
        .success()
        .stdout(predicates::str::contains("total checks : 1"));
}
