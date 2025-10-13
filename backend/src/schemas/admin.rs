#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
   pub fn new(password: String) -> Result<Self, String> {
      if password.len() < 8 {
         return Err("Password must be at least 8 characters long".to_string());
      }

      if !password.chars().any(|c| c.is_uppercase()) {
         return Err("Password must contain at least one uppercase letter".to_string());
      }

      if !password.chars().any(|c| c.is_lowercase()) {
         return Err("Password must contain at least one lowercase letter".to_string());
      }

      if !password.chars().any(|c| c.is_numeric()) {
         return Err("Password must contain at least one number".to_string());
      }

      Ok(Password(password))
   }

   pub fn value(&self) -> &str {
      &self.0
   }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
   pub fn new(email: String) -> Result<Self, String> {
      let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

      if !email_regex.is_match(&email) {
         return Err("Email address format is invalid".to_string());
      }
      Ok(Email(email))
   }
   
   pub fn value(&self) -> &str {
      &self.0
   }
}
