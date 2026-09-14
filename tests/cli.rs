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
