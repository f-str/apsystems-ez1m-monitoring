use std::error::Error;

use lettre::{
    message::{MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use log::{error, info};

use crate::config::EmailConfig;

pub async fn send_emails(
    config: &EmailConfig,
    subject: &String,
    body: &str,
    recipients: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for recipient in recipients {
        match send_email(config, subject, body, recipient) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub fn send_email(
    config: &EmailConfig,
    subject: &String,
    body: &str,
    recipient: &str,
) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .to(recipient.to_owned().parse().unwrap())
        .from(config.smtp_from.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(body.to_owned()))
                .singlepart(SinglePart::html(body.to_owned())),
        )
        .unwrap();

    let mailer = SmtpTransport::starttls_relay(&config.smtp_server)
        .unwrap()
        .credentials(Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        ))
        .build();

    match mailer.send(&email) {
        Ok(_) => info!("Email sent successfully!"),
        Err(e) => error!("Could not send email: {:?}", e),
    }

    Ok(())
}
