use chrono::NaiveDateTime;

use crate::util;

#[derive(Debug)]
pub enum Message<'a> {
    InvalidId,
    CandidateAdded,
    CandidateDeleted(&'a str),
    NotRegistered,
    InvalidName,
    Verified(Option<NaiveDateTime>),
    Unauthorized,
    Error,
}

impl From<Message<'_>> for String {
    fn from(value: Message<'_>) -> Self {
        match value {
            Message::InvalidId => "Mã số sinh viên không hợp lệ.".to_string(),
            Message::CandidateAdded => "Các thí sinh đã được thêm.".to_string(),
            Message::CandidateDeleted(id) => format!("Thí sinh với mã số {} đã được xóa.", id),
            Message::NotRegistered => "Mã số sinh viên chưa đăng kí.".to_string(),
            Message::Verified(None) => "Bạn đã verify thành công".to_string(),
            Message::Verified(Some(time)) => format!("Bạn đã verify thành công vào {}.", util::format_datetime(time)),
            Message::InvalidName => "Vui lòng đặt tên đúng quy tắc.".to_string(),
            Message::Unauthorized => "Lệnh dành riêng cho moderator".to_string(),
            Message::Error => "Có một số lỗi đã xảy ra bạn thử lại trong ít phút hoặc tạo ticket để được hỗ trợ nhé!".to_string(),
 }
    }
}
