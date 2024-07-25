use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UsersLoginGmailRequest {
    pub fullname: String,
    pub email: String,
}

impl UsersLoginGmailRequest {
    pub(crate) fn is_valid(self) -> bool {
        let has_symbol_regex = Regex::new(r"[^A-Za-z0-9]").unwrap();
        let email_valid_regex =
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        let fullname_valid = !has_symbol_regex.is_match(&self.fullname);
        let email_valid = email_valid_regex.is_match(&self.email);

        fullname_valid & email_valid
    }
}
