use crate::error::{Result, TodoError};
use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone};

/// Parse a date string into a `DateTime<Local>`.
///
/// This function supports multiple formats:
/// - ISO format: `YYYY-MM-DD`
/// - US/European formats: `MM/DD/YYYY`, `DD/MM/YYYY`, `YYYY/MM/DD`, `MM-DD-YYYY`, `DD-MM-YYYY`
/// - Relative keywords: `today`, `tomorrow`
/// - Natural language (via `chrono-english`): e.g. `next friday`, `in 2 days`
/// - Day names: `monday`, `tue`, etc. (returns the next occurrence)
///
/// # Errors
/// Returns a `TodoError::DateParse` if the string cannot be parsed as a date.
///
/// # Examples
/// ```
/// use todo_cli::date_parser::parse_date;
/// let dt = parse_date("2025-07-15").unwrap();
/// ```
pub fn parse_date(date_str: &str) -> Result<DateTime<Local>> {
    let date_str = date_str.trim().to_lowercase();

    match date_str.as_str() {
        "today" => {
            let today = Local::now().date_naive();
            return Ok(Local
                .from_local_datetime(&today.and_hms_opt(23, 59, 59).unwrap())
                .unwrap());
        }
        "tomorrow" => {
            let tomorrow = Local::now().date_naive() + chrono::Duration::days(1);
            return Ok(Local
                .from_local_datetime(&tomorrow.and_hms_opt(23, 59, 59).unwrap())
                .unwrap());
        }
        _ => {}
    }

    if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        return Ok(Local
            .from_local_datetime(&date.and_hms_opt(23, 59, 59).unwrap())
            .unwrap());
    }

    let formats = ["%m/%d/%Y", "%d/%m/%Y", "%Y/%m/%d", "%m-%d-%Y", "%d-%m-%Y"];
    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, format) {
            return Ok(Local
                .from_local_datetime(&date.and_hms_opt(23, 59, 59).unwrap())
                .unwrap());
        }
    }

    // Custom handling for 'in N <unit>' phrases
    if let Some((n, unit)) = parse_in_n_unit(&date_str) {
        let now = Local::now().date_naive();
        let target_date = match unit {
            "day" | "days" => now + chrono::Duration::days(n),
            "week" | "weeks" => now + chrono::Duration::days(n * 7),
            "month" | "months" => {
                let mut y = now.year();
                let mut m = now.month() as i32 + n as i32;
                while m > 12 {
                    y += 1;
                    m -= 12;
                }
                // Clamp day to last day of month
                let last_day = match m {
                    1 => 31,
                    2 => {
                        if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
                            29
                        } else {
                            28
                        }
                    }
                    3 => 31,
                    4 => 30,
                    5 => 31,
                    6 => 30,
                    7 => 31,
                    8 => 31,
                    9 => 30,
                    10 => 31,
                    11 => 30,
                    12 => 31,
                    _ => 28,
                };
                let d = now.day().min(last_day);
                NaiveDate::from_ymd_opt(y, m as u32, d).unwrap_or(now)
            }
            "year" | "years" => {
                let y = now.year() + n as i32;
                NaiveDate::from_ymd_opt(y, now.month(), now.day()).unwrap_or(now)
            }
            _ => now,
        };
        return Ok(Local
            .from_local_datetime(&target_date.and_hms_opt(23, 59, 59).unwrap())
            .unwrap());
    }

    // Try chrono-english for natural language parsing
    match chrono_english::parse_date_string(&date_str, Local::now(), chrono_english::Dialect::Us) {
        Ok(datetime) => Ok(datetime),
        Err(_) => {
            // Only try weekday fallback if the input is a weekday
            let weekdays = [
                "monday",
                "mon",
                "tuesday",
                "tue",
                "wednesday",
                "wed",
                "thursday",
                "thu",
                "friday",
                "fri",
                "saturday",
                "sat",
                "sunday",
                "sun",
            ];
            if weekdays.contains(&date_str.as_str()) {
                let now = Local::now();
                let current_weekday = now.weekday().num_days_from_monday();
                let target_weekday = match date_str.as_str() {
                    "monday" | "mon" => 0,
                    "tuesday" | "tue" => 1,
                    "wednesday" | "wed" => 2,
                    "thursday" | "thu" => 3,
                    "friday" | "fri" => 4,
                    "saturday" | "sat" => 5,
                    "sunday" | "sun" => 6,
                    _ => unreachable!(),
                };
                let days_ahead = if target_weekday >= current_weekday {
                    target_weekday - current_weekday
                } else {
                    7 - current_weekday + target_weekday
                };
                let target_date = now.date_naive() + chrono::Duration::days(days_ahead as i64);
                Ok(Local
                    .from_local_datetime(&target_date.and_hms_opt(23, 59, 59).unwrap())
                    .unwrap())
            } else {
                Err(TodoError::DateParse(format!(
                    "Unable to parse date: '{}'. Try formats like: YYYY-MM-DD, today, tomorrow, monday, etc.",
                    date_str
                )))
            }
        }
    }
}

/// Try to extract a date phrase from a list of words and parse it.
pub fn parse_date_from_words(words: &[&str]) -> Option<DateTime<Local>> {
    let mut best: Option<(usize, DateTime<Local>)> = None;
    for window_size in (1..=4).rev() {
        for window in words.windows(window_size) {
            let phrase = window.join(" ");
            match parse_date(&phrase) {
                Ok(dt) => {
                    // Heuristic: prefer longer matches, but avoid ambiguous single-word matches
                    if window_size == 1 {
                        // Only accept single-word matches if they are a weekday or a known keyword
                        let w = phrase.as_str();
                        let valid = matches!(
                            w,
                            "today"
                                | "tomorrow"
                                | "monday"
                                | "mon"
                                | "tuesday"
                                | "tue"
                                | "wednesday"
                                | "wed"
                                | "thursday"
                                | "thu"
                                | "friday"
                                | "fri"
                                | "saturday"
                                | "sat"
                                | "sunday"
                                | "sun"
                        );
                        if !valid {
                            continue;
                        }
                    }
                    // If we already have a match, prefer the longer window
                    if best.is_none() || window_size > best.as_ref().unwrap().0 {
                        best = Some((window_size, dt));
                    }
                }
                Err(_) => continue,
            }
        }
    }
    best.map(|(_, dt)| dt)
}

// Helper function for 'in N <unit>'
fn parse_in_n_unit(s: &str) -> Option<(i64, &str)> {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix("in ") {
        let mut parts = rest.splitn(2, ' ');
        if let (Some(num), Some(unit)) = (parts.next(), parts.next()) {
            let n = num.parse::<i64>().ok().or_else(|| word_to_number(num))?;
            let unit = unit.trim();
            if [
                "day", "days", "week", "weeks", "month", "months", "year", "years",
            ]
            .contains(&unit)
            {
                return Some((n, unit));
            }
        }
    }
    None
}

// Helper to convert small number words to i64
fn word_to_number(word: &str) -> Option<i64> {
    match word.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_today() {
        let result = parse_date("today");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_tomorrow() {
        let result = parse_date("tomorrow");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_iso_date() {
        let result = parse_date("2025-07-15");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_date() {
        let result = parse_date("invalid");
        assert!(result.is_err());
    }
}
