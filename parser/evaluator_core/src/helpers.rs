use chrono::Local;

pub fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}