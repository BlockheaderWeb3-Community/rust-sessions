use super::attendance_status::AttendanceStatus;
// Attendance struct
/// This struct holds the attendance information
/// It contains the date, time in, time out, and attendance status
#[derive(Debug, Clone)]
pub struct AttendanceStruct {
    pub date: String,
    pub time_in: String,
    pub time_out: String,
    pub attendance_status: AttendanceStatus,
}
