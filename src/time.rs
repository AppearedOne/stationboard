use chrono::{Local, NaiveTime};
pub fn format_time(iso_time: &str) -> String {
    let datetime: iso8601::DateTime = iso8601::datetime(iso_time).unwrap();
    let mut h = datetime.time.hour.to_string();
    let mut m = datetime.time.minute.to_string();

    if h.len() == 1 {
        h.insert(0, '0');
    }
    if m.len() == 1 {
        m.insert(0, '0');
    }

    let s = String::from(format!("{}:{}", h, m));
    s
}

pub fn get_delta_time(iso_time: &str) -> String {
    let dep_time =
        NaiveTime::parse_from_str(&format_time(iso_time), "%H:%M").expect("invalid format");
    let now = Local::now().time();
    let delta_minutes = if now < dep_time {
        dep_time.signed_duration_since(now).num_minutes()
    } else {
        now.signed_duration_since(dep_time).num_minutes()
    };

    return delta_minutes.to_string();
}
