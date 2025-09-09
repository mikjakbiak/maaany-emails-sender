use std::error::Error;
use std::{env, fs};

use dotenvy::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PoliticianData {
    email: String,
    name: String,
    position: String,
    pronouns: String,
    notes: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let sender_address = env::var("SENDER_ADDRESS").unwrap();
    let smtp_host = env::var("SMTP_HOST").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();

    //? Open a secure connection to the SMTP server using STARTTLS
    let credentials = Credentials::new(sender_address.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::starttls_relay(&smtp_host)
        .unwrap()
        .credentials(credentials)
        .build();

    let mut email_counter = 0;
    let mut error_counter = 0;
    let mut already_helping_counter = 0;

    let csv_files_dir = "resources/politicians";

    //? Iterate over the csv files in the directory
    for file in fs::read_dir(csv_files_dir)? {
        let file = file?;
        let path = file.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        println!("Processing file: {}", filename);

        //? Read the csv file and send an email to each politician
        let mut rdr = csv::Reader::from_path(path)?;
        for result in rdr.deserialize() {
            let politician_data: PoliticianData = result?;

            let send_result = send_email(&mailer, politician_data);

            // TODO: implement handling for:
            // 5.4.5 Daily user sending limit exceeded. For more information on Gmail5.4.5 sending limits go to5.4.5  https://support.google.com/a/answer/166852 ffacd0b85a97d-3e7521c99b9sm2149053f8f.15 - gsmtp
            // use two different accounts for sending emails. Google allows 500 emails per day per account. For now it's needed to run the application two times with two different accounts.
            match send_result {
                Ok(true) => email_counter += 1,
                Ok(false) => already_helping_counter += 1,
                Err(e) => {
                    error_counter += 1;
                    eprintln!("❌ Error sending email: {e}");
                }
            }
        }
    }

    println!();
    println!("✉️ {email_counter} emails sent");
    println!("🥳 {already_helping_counter} politicians already help");
    println!("❌ {error_counter} errors occurred");
    println!();

    Ok(())
}

fn send_email(
    mailer: &SmtpTransport,
    politician_data: PoliticianData,
) -> Result<bool, Box<dyn Error>> {
    let subject = env::var("EMAIL_SUBJECT").unwrap();
    let sender_name = env::var("SENDER_NAME").unwrap();
    let sender_address = env::var("SENDER_ADDRESS").unwrap();

    //? Check if the politician already helps
    let is_already_helping = politician_data.notes.contains("wspiera");
    if is_already_helping {
        println!("🥳 {} already helps", politician_data.name);
        return Ok(false);
    }

    println!(
        "✉️ Sending email to {} {} ({})",
        politician_data.position, politician_data.name, politician_data.email
    );

    let is_female = politician_data.pronouns.contains("F");

    //? Read the email body based on the politician's gender
    let body = fs::read_to_string(format!(
        "resources/email-bodies/{}.html",
        if is_female { "female" } else { "male" }
    ))?;

    //? Build the email
    let content_type: ContentType = "text/html; charset=utf-8".parse().unwrap();
    let email = Message::builder()
        .from(
            format!("{} <{}>", sender_name, sender_address)
                .parse()
                .unwrap(),
        )
        .to(
            format!("{} <{}>", politician_data.name, politician_data.email)
                .parse()
                .unwrap(),
        )
        .subject(subject)
        .header(content_type)
        .body(body)?;

    //? Send the email via the SMTP transport
    match mailer.send(&email) {
        Ok(_) => println!("✅ Email sent successfully!"),
        Err(e) => Err(format!(
            "❌ Could not send email to {}: {:?}",
            politician_data.name, e
        ))?,
    }

    Ok(true)
}
