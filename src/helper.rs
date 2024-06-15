use std::env;

use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub fn send_mail(subject: &str, body: String) {
  let service = if let Ok(service) = env::var("SMTP_SERVICE") {
    service
  } else {
    return;
  };
  let username = if let Ok(username) = env::var("SMTP_USER") {
    username
  } else {
    return;
  };
  let password = if let Ok(password) = env::var("SMTP_PASSWORD") {
    password
  } else {
    return;
  };
  let receiver = if let Ok(receiver) = env::var("EMAIL_RECEIVER") {
    receiver
  } else {
    return;
  };
  let sender = format!("Punchme <{}>", receiver);
  let receiver = "861947542@qq.com";
  let creds = Credentials::new(username, password);
  let mailer = SmtpTransport::relay(&service).unwrap().credentials(creds).build();
  let email = Message::builder()
    .from(sender.parse().unwrap())
    .to(receiver.parse().unwrap())
    .subject(subject)
    .body(body)
    .unwrap();
  match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {e:?}"),
  }
}
