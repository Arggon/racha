use std::process::ExitCode;

use chrono::Local;
use clap::{Parser, Subcommand};

use racha::remind;
use racha::storage;
use racha::streaks;

#[derive(Parser)]
#[command(
    name = "racha",
    version,
    about = "Tracker de hábitos con rachas — local, rápido, tuyo"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Directorio de datos (default: $XDG_DATA_HOME/racha o ~/.local/share/racha)
    #[arg(long, global = true, env = "RACHA_DATA_DIR")]
    data_dir: Option<std::path::PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Agrega un hábito nuevo
    Add { name: String },
    /// Registra el check de hoy para un hábito
    Check { name: String },
    /// Lista los hábitos con su racha actual (compacto)
    List,
    /// Rachas actuales, mejores y vista semanal
    Stats { name: Option<String> },
    /// Notifica los hábitos sin check hoy (canal nativo freedesktop)
    Remind,
}

/// Error de ejecución con su exit code: 1 = error genérico,
/// 2 = sin bus de sesión D-Bus (degradación documentada de `remind`).
struct CliError {
    msg: String,
    code: u8,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {}", e.msg);
            ExitCode::from(e.code)
        }
    }
}

fn run(cli: Cli) -> Result<(), CliError> {
    let dir = storage::resolve_dir(cli.data_dir);
    let mut ledger = storage::load(&dir).map_err(err1)?;
    let today = Local::now().date_naive();

    match cli.command {
        Command::Add { name } => {
            let name = name.trim().to_string();
            if name.is_empty() {
                return Err(err1(
                    "el nombre del hábito no puede quedar vacío (probá: racha add meditar)"
                        .to_string(),
                ));
            }
            if ledger.add_habit(name.clone(), today) {
                storage::save(&dir, &ledger).map_err(err1)?;
                println!("hábito agregado: {name}");
            } else {
                return Err(err1(format!("el hábito ya existe: {name}")));
            }
        }
        Command::Check { name } => {
            let name = name.trim().to_string();
            if name.is_empty() {
                return Err(err1(
                    "el nombre del hábito no puede quedar vacío (probá: racha add meditar)"
                        .to_string(),
                ));
            }
            let (msg, changed) = {
                let habit = ledger.habit_mut(&name).ok_or_else(|| {
                    err1(format!(
                        "hábito inexistente: {name} (agregalo con `racha add {name}`)"
                    ))
                })?;
                if habit.add_check(today) {
                    let streak = streaks::current_streak(&habit.checks, today);
                    (format!("✓ {name} — racha actual: {streak} día(s)"), true)
                } else {
                    (
                        format!("✓ {name} — ya estaba registrado hoy, no pasa nada"),
                        false,
                    )
                }
            };
            if changed {
                storage::save(&dir, &ledger).map_err(err1)?;
            }
            println!("{msg}");
        }
        Command::List => {
            if ledger.habits.is_empty() {
                println!("sin hábitos todavía — probá: racha add meditar");
                return Ok(());
            }
            for habit in &ledger.habits {
                let streak = streaks::current_streak(&habit.checks, today);
                println!("{} — racha actual: {} día(s)", habit.name, streak);
            }
        }
        Command::Stats { name } => match name {
            Some(name) => {
                let habit = ledger
                    .habit(&name)
                    .ok_or_else(|| err1(format!("hábito inexistente: {name}")))?;
                print_habit(habit, today);
            }
            None => {
                if ledger.habits.is_empty() {
                    println!("sin hábitos todavía — probá: racha add meditar");
                    return Ok(());
                }
                for habit in &ledger.habits {
                    print_habit(habit, today);
                }
            }
        },
        Command::Remind => {
            if ledger.habits.is_empty() {
                println!("sin hábitos todavía — probá: racha add meditar");
                return Ok(());
            }
            let due = remind::due_habits(&ledger.habits, today);
            if due.is_empty() {
                println!("nada vencido hoy — todos los hábitos con check ✓");
                return Ok(());
            }
            for habit in &due {
                println!("vencido: {}", habit.name);
            }
            for habit in &due {
                let body = format!("te falta checkear hoy: {}", habit.name);
                if let Err(e) = notify_rust::Notification::new()
                    .summary("racha")
                    .body(&body)
                    .show()
                {
                    return Err(CliError {
                        msg: format!(
                            "no pude notificar ({}): {e}. \
                             ¿Hay bus de sesión D-Bus y daemon de notificaciones?",
                            habit.name
                        ),
                        code: 2,
                    });
                }
            }
        }
    }
    Ok(())
}

fn err1(msg: String) -> CliError {
    CliError { msg, code: 1 }
}

fn print_habit(habit: &racha::Habit, today: chrono::NaiveDate) {
    let current = streaks::current_streak(&habit.checks, today);
    let best = streaks::best_streak(&habit.checks);
    let week = streaks::week_view(&habit.checks, today);
    let week_str: String = week
        .iter()
        .map(|&done| if done { "✓" } else { "·" })
        .collect();
    println!("{name}", name = habit.name);
    println!("  racha actual : {current} día(s)");
    println!("  mejor racha  : {best} día(s)");
    println!("  total checks : {}", streaks::total_checks(&habit.checks));
    println!(
        "  % semana     : {}%",
        streaks::week_completion(&habit.checks, today)
    );
    println!(
        "  mes          : {}",
        streaks::checks_in_month(&habit.checks, today)
    );
    println!(
        "  año          : {}",
        streaks::checks_in_year(&habit.checks, today)
    );
    println!("  semana (L..D): {week_str}");
}
