use chrono::{Datelike, Duration, NaiveDate};

/// Racha actual contando hacia atrás desde `today` (o desde ayer si hoy no
/// hay check: el día todavía no terminó, la racha no se rompe hasta medianoche).
pub fn current_streak(checks: &[NaiveDate], today: NaiveDate) -> u32 {
    let set: std::collections::HashSet<NaiveDate> = checks.iter().copied().collect();
    let start = if set.contains(&today) {
        today
    } else if set.contains(&(today - Duration::days(1))) {
        today - Duration::days(1)
    } else {
        return 0;
    };

    let mut streak = 0u32;
    let mut day = start;
    while set.contains(&day) {
        streak += 1;
        day -= Duration::days(1);
    }
    streak
}

/// Mejor racha histórica: la corrida más larga de días consecutivos.
pub fn best_streak(checks: &[NaiveDate]) -> u32 {
    let mut dates = checks.to_vec();
    dates.sort_unstable();
    dates.dedup();

    let mut best = 0u32;
    let mut run = 0u32;
    for (i, d) in dates.iter().enumerate() {
        if i > 0 && *d - dates[i - 1] == Duration::days(1) {
            run += 1;
        } else {
            run = 1;
        }
        best = best.max(run);
    }
    best
}

/// Vista semanal: los 7 días de la semana calendario (lunes a domingo)
/// que contiene `today`. true = con check.
pub fn week_view(checks: &[NaiveDate], today: NaiveDate) -> [bool; 7] {
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let set: std::collections::HashSet<NaiveDate> = checks.iter().copied().collect();
    let mut week = [false; 7];
    for (i, day) in week.iter_mut().enumerate() {
        *day = set.contains(&(monday + Duration::days(i as i64)));
    }
    week
}

pub fn total_checks(checks: &[NaiveDate]) -> usize {
    checks.len()
}

/// Checks con fecha en el mes de `today`.
pub fn checks_in_month(checks: &[NaiveDate], today: NaiveDate) -> usize {
    checks
        .iter()
        .filter(|d| d.year() == today.year() && d.month() == today.month())
        .count()
}

/// Checks con fecha en el año de `today`.
pub fn checks_in_year(checks: &[NaiveDate], today: NaiveDate) -> usize {
    checks.iter().filter(|d| d.year() == today.year()).count()
}

/// % de cumplimiento de la semana corriente: checks de la semana
/// (lunes..domingo que contiene `today`) sobre los días transcurridos de
/// la semana (`today` incluido), 0..100, redondeo hacia abajo.
pub fn week_completion(checks: &[NaiveDate], today: NaiveDate) -> u32 {
    let week = week_view(checks, today);
    let elapsed = today.weekday().num_days_from_monday() + 1;
    let done = week.iter().filter(|&&done| done).count() as u32;
    done * 100 / elapsed
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn current_streak_counts_back_from_today() {
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 11), d(2026, 9, 12), today];
        assert_eq!(current_streak(&checks, today), 3);
    }

    #[test]
    fn current_streak_survives_until_midnight_when_today_missing() {
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 11), d(2026, 9, 12)];
        assert_eq!(current_streak(&checks, today), 2);
    }

    #[test]
    fn current_streak_zero_when_yesterday_also_missing() {
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 10)];
        assert_eq!(current_streak(&checks, today), 0);
    }

    #[test]
    fn best_streak_ignores_order_and_duplicates() {
        let checks = vec![
            d(2026, 1, 3),
            d(2026, 1, 1),
            d(2026, 1, 2),
            d(2026, 1, 2),
            d(2026, 3, 8),
        ];
        assert_eq!(best_streak(&checks), 3);
    }

    #[test]
    fn best_streak_crosses_month_boundary() {
        let checks = vec![d(2026, 8, 30), d(2026, 8, 31), d(2026, 9, 1)];
        assert_eq!(best_streak(&checks), 3);
    }

    #[test]
    fn week_view_maps_monday_to_sunday() {
        // 2026-09-13 es domingo; la semana es 7..13 de septiembre.
        let today = d(2026, 9, 13);
        let checks = vec![d(2026, 9, 7), d(2026, 9, 13)];
        assert_eq!(
            week_view(&checks, today),
            [true, false, false, false, false, false, true]
        );
    }

    #[test]
    fn empty_checks_give_zeroes() {
        let today = d(2026, 9, 13);
        assert_eq!(current_streak(&[], today), 0);
        assert_eq!(best_streak(&[]), 0);
        assert_eq!(week_view(&[], today), [false; 7]);
    }

    #[test]
    fn checks_in_month_counts_only_current_month() {
        // Septiembre (30 días) vs agosto y octubre.
        let today = d(2026, 9, 30);
        let checks = vec![
            d(2026, 9, 1),
            d(2026, 9, 30),
            d(2026, 8, 31),
            d(2026, 10, 1),
            d(2025, 9, 15),
        ];
        assert_eq!(checks_in_month(&checks, today), 2);
    }

    #[test]
    fn checks_in_month_february_28() {
        // 2026 no es bisiesto: febrero tiene 28 días.
        let today = d(2026, 2, 28);
        let checks = vec![d(2026, 2, 28), d(2026, 3, 1)];
        assert_eq!(checks_in_month(&checks, today), 1);
    }

    #[test]
    fn checks_in_month_31_day_month_includes_last_day() {
        let today = d(2026, 1, 31);
        let checks = vec![d(2026, 1, 31), d(2026, 2, 1)];
        assert_eq!(checks_in_month(&checks, today), 1);
    }

    #[test]
    fn checks_in_year_counts_only_current_year() {
        // 1 de enero: todo check previo es del año pasado.
        let today = d(2026, 1, 1);
        let checks = vec![d(2026, 1, 1), d(2025, 12, 31), d(2024, 6, 1)];
        assert_eq!(checks_in_year(&checks, today), 1);
        assert_eq!(checks_in_month(&checks, today), 1);
    }

    #[test]
    fn checks_in_year_empty_day_has_no_checks() {
        let today = d(2026, 9, 13);
        assert_eq!(checks_in_year(&[], today), 0);
        assert_eq!(checks_in_month(&[], today), 0);
    }

    #[test]
    fn week_completion_full_week_from_monday() {
        // Lunes con los 7 días chequeados: 100%.
        let today = d(2026, 9, 13); // domingo
        let checks: Vec<NaiveDate> = (7..=13).map(|day| d(2026, 9, day)).collect();
        assert_eq!(week_completion(&checks, today), 100);
    }

    #[test]
    fn week_completion_monday_counts_only_today() {
        // Lunes: solo transcurrió 1 día; con check → 100%, sin → 0%.
        let today = d(2026, 9, 7); // lunes
        assert_eq!(week_completion(&[today], today), 100);
        assert_eq!(week_completion(&[], today), 0);
    }

    #[test]
    fn week_completion_rounds_down() {
        // Miércoles: 3 días transcurridos, 2 con check → 66% (no 67).
        let today = d(2026, 9, 9); // miércoles
        let checks = vec![d(2026, 9, 7), d(2026, 9, 9)];
        assert_eq!(week_completion(&checks, today), 66);
    }

    #[test]
    fn week_completion_ignores_last_week_and_duplicates() {
        // Solo cuenta checks de la semana corriente (lunes..domingo).
        let today = d(2026, 9, 13); // domingo
        let checks = vec![d(2026, 9, 6), d(2026, 9, 7), d(2026, 9, 7)];
        assert_eq!(week_completion(&checks, today), 14); // 1/7
    }

    #[test]
    fn week_completion_on_january_first() {
        // 1 de enero de 2026 es jueves: 4 días transcurridos. La semana
        // arranca el lunes 29-dic-2025: el check del 31-dic cuenta también
        // (la semana calendario cruza el límite de año).
        let today = d(2026, 1, 1); // jueves
        let checks = vec![d(2026, 1, 1), d(2025, 12, 31)];
        assert_eq!(week_completion(&checks, today), 50); // 2/4
        assert_eq!(week_completion(&[d(2026, 1, 1)], today), 25); // 1/4
    }

    #[test]
    fn week_completion_no_checks_is_zero() {
        let today = d(2026, 9, 13);
        assert_eq!(week_completion(&[], today), 0);
    }
}
