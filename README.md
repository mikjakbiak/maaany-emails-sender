# politician-abuser

Abuse a lot of politicians with emails

## Prerequisites

- Gmail account with SMTP server enabled (Maybe you can use other email providers, but I tested with gmail)
- Two-factor authentication enabled
- App password for the account
  - This video shows how to get the credentials for Gmail SMTP server: https://www.youtube.com/watch?v=y5IasMFYdBc
- Rust installed (https://www.rust-lang.org/tools/install)

## How to run the application

1. Clone the repository
2. Create a `.env` file in the root of the repository and add the following variables:
   - `SENDER_NAME` - the name of the sender
   - `SENDER_ADDRESS` - the email address of the account
   - `SMTP_PASSWORD` - the app password that you created earlier
   - `SMTP_HOST` - the SMTP host of the account (if you use gmail, it's `smtp.gmail.com`)
   - `EMAIL_SUBJECT` - the subject of the email
3. Modify the email bodies from the `resources/email-bodies` directory to your needs
4. Run `cargo build`
5. Run `cargo run`
