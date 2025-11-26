use chrono::NaiveDateTime;

#[derive(Debug)]
pub enum Message<'a> {
    InvalidId,
    CandidateAdded(&'a str),
    CandidateDeleted(&'a str),
    NotRegistered,
    InvalidName,
    Verified(NaiveDateTime),
    Error,
}

impl From<Message<'_>> for String {
    fn from(value: Message<'_>) -> Self {
        match value {
            Message::InvalidId => "Mã số sinh viên không hợp lệ.".to_string(),
            Message::CandidateAdded(id) => format!("Thí sinh với mã số {} đã được thêm.", id),
            Message::CandidateDeleted(id) => format!("Thí sinh với mã số {} đã được xóa.", id),
            Message::NotRegistered => "Mã số sinh viên chưa đăng kí.".to_string(),
            Message::Verified(time) => format!("Bạn đã verify thành công vào lúc {}.", time),
            Message::InvalidName => "Vui lòng đặt tên đúng quy tắc.".to_string(),
            Message::Error => "Có một số lỗi đã xảy ra bạn thử lại trong ít phút hoặc tạo ticket để được hỗ trợ nhé!".to_string(),
        }
    }
}
