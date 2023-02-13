use chrono::NaiveDateTime;

pub type UrlParams<'sq> = Vec<(&'sq str, &'sq str)>;

pub struct ChannelData {
  pub id: String,
  pub name: String,
  pub last_message: i64,
  pub members_count: i32,
  pub is_old: bool,
  pub is_small: bool,
  pub is_ignored: bool,
}

impl ChannelData {
  pub fn last_message_formatted(&self) -> String {
    let t: i64 = self.last_message;
    if t == 0 { return "[unable to parse timestamp]".to_string() }
    NaiveDateTime::from_timestamp_opt(t, 0).unwrap().format("%b %d, %Y").to_string()
  }
}
