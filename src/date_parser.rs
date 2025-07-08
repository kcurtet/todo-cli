use chrono::{DateTime, Local, NaiveDate, TimeZone, Datelike};
use crate::error::{Result, TodoError};

pub fn parse_date(date_str: &str) -> Result<DateTime<Local>> {
    let date_str = date_str.trim().to_lowercase();
    
    // Handle special cases first
    match date_str.as_str() {
        "today" => {
            let today = Local::now().date_naive();
            return Ok(Local.from_local_datetime(&today.and_hms_opt(23, 59, 59).unwrap()).unwrap());
        }
        "tomorrow" => {
            let tomorrow = Local::now().date_naive() + chrono::Duration::days(1);
            return Ok(Local.from_local_datetime(&tomorrow.and_hms_opt(23, 59, 59).unwrap()).unwrap());
        }
        _ => {}
    }
    
    // Try ISO date format (YYYY-MM-DD)
    if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        return Ok(Local.from_local_datetime(&date.and_hms_opt(23, 59, 59).unwrap()).unwrap());
    }
    
    // Try other common formats
    let formats = [
        "%m/%d/%Y",
        "%d/%m/%Y", 
        "%Y/%m/%d",
        "%m-%d-%Y",
        "%d-%m-%Y",
    ];
    
    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, format) {
            return Ok(Local.from_local_datetime(&date.and_hms_opt(23, 59, 59).unwrap()).unwrap());
        }
    }
    
    // Try natural language parsing with chrono-english
    match chrono_english::parse_date_string(&date_str, Local::now(), chrono_english::Dialect::Us) {
        Ok(datetime) => Ok(datetime),
        Err(_) => {
            // Try some common day names
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
                _ => {
                    return Err(TodoError::DateParse(format!(
                        "Unable to parse date: '{}'. Try formats like: YYYY-MM-DD, today, tomorrow, monday, etc.", 
                        date_str
                    )));
                }
            };
            
            let days_ahead = if target_weekday >= current_weekday {
                target_weekday - current_weekday
            } else {
                7 - current_weekday + target_weekday
            };
            
            let target_date = now.date_naive() + chrono::Duration::days(days_ahead as i64);
            Ok(Local.from_local_datetime(&target_date.and_hms_opt(23, 59, 59).unwrap()).unwrap())
        }
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
