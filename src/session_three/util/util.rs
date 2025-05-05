use core::time;
use std::time::{SystemTime, UNIX_EPOCH};

/// util function to convert a string slice to a String
pub fn convert_to_string(x: &str) -> String {
    x.to_string()
}

/// concat time function
pub fn concat_time(h: String, m: u64, s: u64) -> String {
    format!("{}:{:02}:{:02}", h, m, s)
}

/// get current time function
/// This function returns the current time in the format HH:MM:SS
/// It uses the SystemTime struct to get the current time
/// It then converts the time to seconds since the UNIX_EPOCH
/// It then calculates the number of seconds in a day, hour, minute, and second
pub fn get_current_time() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards"); // Due to friction that occurs during the earth's orbit around the sun
    let total_seconds = now.as_secs();
    let seconds_in_day = total_seconds % 86400; // Seconds in a day
    let hour = seconds_in_day / 3600; // Seconds in an hour
    let minute = (seconds_in_day % 3600) / 60; // Seconds in a minute
    let second = seconds_in_day % 60;
    concat_time(format!("{:02}", hour), minute, second)
}

/// get current date function
/// This function returns the current date in the format YYYY-MM-DD
/// It uses the SystemTime struct to get the current time
/// It then converts the time to seconds since the UNIX_EPOCH
/// It then calculates the number of seconds in a year, month, and day
pub fn get_current_date() -> String {
    let datetime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let year = 1970 + (datetime.as_secs() / 31536000); // Number of seconds in a year
    let month = (datetime.as_secs() % 31536000) / 2629743; // Number of seconds in a month
    let day = (datetime.as_secs() % 2629743) / 86400; // Number of seconds in a day
    concat_time(year.to_string(), month, day)
}
