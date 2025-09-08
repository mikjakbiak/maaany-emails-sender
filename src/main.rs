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

            send_email(politician_data)?;
        }
    }
    Ok(())
}

fn send_email(politician_data: PoliticianData) -> Result<(), Box<dyn Error>> {
    let subject = env::var("EMAIL_SUBJECT").unwrap();
    let sender_name = env::var("SENDER_NAME").unwrap();
    let sender_address = env::var("SENDER_ADDRESS").unwrap();
    let smtp_host = env::var("SMTP_HOST").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();

    //? Check if the politician already helps
    let is_already_helping = politician_data.notes.contains("wspiera");
    if is_already_helping {
        println!("🥳 {} already helps", politician_data.name);
        return Ok(());
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

    //? Open a secure connection to the SMTP server using STARTTLS
    let credentials = Credentials::new(sender_address.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::starttls_relay(&smtp_host)
        .unwrap()
        .credentials(credentials)
        .build();

    //? Send the email via the SMTP transport
    match mailer.send(&email) {
        Ok(_) => println!("✅ Email sent successfully!"),
        Err(e) => eprintln!(
            "❌ Could not send email to {}: {:?}",
            politician_data.name, e
        ),
    }

    Ok(())
}
