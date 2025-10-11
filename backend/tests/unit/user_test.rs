use book_store::schemas::user::RegisterUser;
use rstest::rstest;
use validator::Validate;

#[rstest]
fn test_valid_registration() {
   let input = RegisterUser {
      email: "newuser@example.com".to_string(),
      password: "SecurePass123".to_string(),
      confirm_password: "SecurePass123".to_string(),
   };

   assert!(input.validate().is_ok());
}

#[rstest]
#[case::not_email("notanemail")]
#[case::no_email("noemail@")]
#[case::with_space("notane  mail@email")]
fn test_invalid_email(#[case] email: &str) {
   let input = RegisterUser {
      email: email.to_string(),
      password: "SecurePass123".to_string(),
      confirm_password: "SecurePass123".to_string(),
   };

   assert!(input.validate().is_err());
}

#[rstest]
fn test_passwords_mismatch() {
   let input = RegisterUser {
      email: "user@example.com".to_string(),
      password: "SecurePass123".to_string(),
      confirm_password: "DifferentPass123".to_string(),
   };

   assert!(input.validate().is_err());
}

#[rstest]
#[case::empty("")]
#[case::too_short("weak")]
#[case::missing_lowercase("WEAKPASS123")]
#[case::missing_uppercase("weakpass123")]
#[case::missing_number("WeakpassABCjj")]
fn test_weak_password(#[case] password: &str) {
   let input = RegisterUser {
      email: "user@example.com".to_string(),
      password: password.to_string(),
      confirm_password: password.to_string(),
   };

   assert!(input.validate().is_err());
}
