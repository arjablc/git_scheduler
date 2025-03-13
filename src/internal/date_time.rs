use chrono::Local;

pub fn get_local_time() -> String {
    let local_time = Local::now();
    local_time.format("%d/%m/%Y %I:%M %p").to_string()
}


