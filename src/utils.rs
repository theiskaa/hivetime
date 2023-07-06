/*
 * TODO: add documentation about file.
 */

pub struct Utils {}

impl Utils {
    // Convert minutes to proper understandable time String
    // For example: if minutes is 120 return 2h,
    // and if minutes is 150m return to 2h 30m.
    pub fn to_time(minutes: f64) -> String {
        let hours = (minutes / 60.0).floor();
        let remaining_minutes = (minutes % 60.0).floor();

        let mut time_string = String::new();

        if hours > 0.0 {
            time_string.push_str(&format!("{}h", hours));
        }

        if remaining_minutes > 0.0 {
            if !time_string.is_empty() {
                time_string.push(' ');
            }
            time_string.push_str(&format!("{}m", remaining_minutes));
        }

        time_string
    }
}
