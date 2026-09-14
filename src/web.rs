use std::fmt::Write as _;

use chrono::{Duration, NaiveDate};

use crate::model::Ledger;
use crate::streaks;

/// Días de historial listados por hábito en la vista web.
const RECENT_DAYS: usize = 14;

/// Escapa texto para uso seguro en contenido y atributos HTML.
fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Últimos `n` días terminando en `today` (inclusive), en orden cronológico.
fn recent_days(today: NaiveDate, n: usize) -> Vec<NaiveDate> {
    (0..n)
        .map(|i| today - Duration::days((n - 1 - i) as i64))
        .collect()
}

/// Genera la página HTML completa (solo lectura) del ledger.
/// Función pura: sin I/O ni estado.
pub fn render(ledger: &Ledger, today: NaiveDate) -> String {
    let mut html = String::with_capacity(4096);
    let _ = write!(
        html,
        "<!DOCTYPE html>\n\
         <html lang=\"es\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>racha — vista de solo lectura</title>\n\
         <style>\n\
         body {{ font-family: sans-serif; max-width: 48rem; margin: 2rem auto; padding: 0 1rem; }}\n\
         .habit {{ border: 1px solid #ccc; border-radius: 8px; padding: 1rem; margin-bottom: 1rem; }}\n\
         .habit h2 {{ margin-top: 0; }}\n\
         table {{ border-collapse: collapse; }}\n\
         td, th {{ padding: 0 0.5rem; text-align: left; }}\n\
         .done {{ color: #0a7d0a; font-weight: bold; }}\n\
         .miss {{ color: #bbb; }}\n\
         footer {{ margin-top: 2rem; color: #777; font-size: 0.85rem; }}\n\
         </style>\n\
         </head>\n\
         <body>\n\
         <h1>racha</h1>\n\
         <p>Generado: {today}</p>\n\
         <p>Vista de solo lectura — para registrar checks usá el CLI \
         (<code>racha check &lt;hábito&gt;</code>).</p>\n",
        today = today,
    );

    if ledger.habits.is_empty() {
        writeln!(
            html,
            "<p id=\"empty\">sin hábitos todavía — probá: <code>racha add meditar</code></p>"
        )
        .expect("escribiendo en String");
    }

    for habit in &ledger.habits {
        let checks = &habit.checks;
        let current = streaks::current_streak(checks, today);
        let best = streaks::best_streak(checks);
        let total = streaks::total_checks(checks);
        let pct = streaks::week_completion(checks, today);
        let month = streaks::checks_in_month(checks, today);
        let year = streaks::checks_in_year(checks, today);
        let week = streaks::week_view(checks, today);

        let _ = write!(
            html,
            "<section class=\"habit\">\n<h2>{}</h2>\n",
            esc(&habit.name)
        );
        let _ = write!(
            html,
            "<table>\n\
             <tr><th>racha actual</th><td>{current} día(s)</td></tr>\n\
             <tr><th>mejor racha</th><td>{best} día(s)</td></tr>\n\
             <tr><th>total checks</th><td>{total}</td></tr>\n\
             <tr><th>% semana</th><td>{pct}%</td></tr>\n\
             <tr><th>mes</th><td>{month}</td></tr>\n\
             <tr><th>año</th><td>{year}</td></tr>\n\
             </table>\n"
        );

        // Vista semanal (lunes..domingo).
        let week_str: String = week
            .iter()
            .map(|&done| if done { "✓" } else { "·" })
            .collect();
        writeln!(html, "<p>semana (L..D): {week_str}</p>").expect("escribiendo en String");

        // Últimos 14 días.
        let set: std::collections::HashSet<NaiveDate> = checks.iter().copied().collect();
        let days = recent_days(today, RECENT_DAYS);
        let _ = write!(
            html,
            "<p>últimos {} días:</p>\n<ul class=\"recent\">\n",
            days.len()
        );
        for day in days {
            if set.contains(&day) {
                writeln!(html, "<li><span class=\"done\">✓</span> {day}</li>")
                    .expect("escribiendo en String");
            } else {
                writeln!(html, "<li><span class=\"miss\">·</span> {day}</li>")
                    .expect("escribiendo en String");
            }
        }
        let _ = write!(html, "</ul>\n</section>\n");
    }

    let _ = write!(
        html,
        "<footer>Generado por <code>racha web</code> — archivo estático, sin servidor.</footer>\n\
         </body>\n</html>\n"
    );
    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Habit, Ledger};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn habit(name: &str, checks: &[NaiveDate]) -> Habit {
        let mut h = Habit::new(name.to_string(), d(2026, 9, 1));
        h.checks = checks.to_vec();
        h
    }

    #[test]
    fn empty_ledger_renders_empty_message() {
        let html = render(&Ledger::default(), d(2026, 9, 13));
        assert!(html.contains("sin hábitos todavía"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn renders_all_sections_per_habit() {
        let today = d(2026, 9, 13);
        let mut ledger = Ledger::default();
        ledger.habits.push(habit(
            "meditar",
            &[d(2026, 9, 7), d(2026, 9, 12), d(2026, 9, 13), d(2026, 9, 1)],
        ));
        let html = render(&ledger, today);
        assert!(html.contains("meditar"));
        assert!(html.contains("racha actual"));
        assert!(html.contains("mejor racha"));
        assert!(html.contains("total checks"));
        assert!(html.contains("% semana"));
        assert!(html.contains(">4</td>")); // mes: 1, 7, 12 y 13 de septiembre
        assert!(html.contains("Generado: 2026-09-13"));
        // semana: lunes ✓, sábado ✓ y domingo ✓
        assert!(html.contains("✓····✓✓"));
        // últimos 14 días listados
        assert!(html.contains("2026-08-31"));
        assert!(html.contains("2026-09-13"));
    }

    #[test]
    fn escapes_habit_names() {
        let mut ledger = Ledger::default();
        ledger
            .habits
            .push(habit("<script>alert('x')</script>", &[d(2026, 9, 13)]));
        let html = render(&ledger, d(2026, 9, 13));
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;alert(&#39;x&#39;)&lt;/script&gt;"));
    }

    #[test]
    fn recent_days_are_sorted_and_end_today() {
        let days = recent_days(d(2026, 9, 13), 14);
        assert_eq!(days.len(), 14);
        assert_eq!(days[0], d(2026, 8, 31));
        assert_eq!(days[13], d(2026, 9, 13));
    }
}
