use super::super::types::{attendance::AttendanceStruct, attendance_status::AttendanceStatus};
use super::super::util::util::{get_current_date, get_current_time};
// implementation block

impl AttendanceStruct {
    pub fn new() -> Self {
        AttendanceStruct {
            attendance_status: AttendanceStatus::Present,
            date: get_current_date(),
            time_in: get_current_time(),
            time_out: String::new(),
        }
    }
}
