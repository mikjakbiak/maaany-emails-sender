# maaany-emails-sender

## 🇵🇱 PL (ENG below)

Wyślij duuużo emaili do polskich polityków żeby pomogli Global Sumud Flotilla

## Potrzebujesz

- Konto Gmail (Możliwe, że działa też z innymi skrzynkami, ja testowałem tylko na Gmailu)
- Włączona weryfikacja dwuskładnikowa na tym koncie
- Hasło do aplikacji na tym koncie
  - Ten filmik pokazuje jak to zrobić: https://www.youtube.com/watch?v=y5IasMFYdBc (jak masz Gmaila po polsku to szukaj "Hasła do aplikacji")
- Zainstalowany Rust (https://www.rust-lang.org/tools/install)

## Jak odpalić aplikacje

1. Sklonuj repozytorium
2. Stwórz `.env` na roocie repa i dodaj następujące zmienne:
   - `SENDER_NAME` - imię nadawcy
   - `SENDER_ADDRESS` - adres email nadawcy
   - `SMTP_PASSWORD` - wcześniej utworzone hasło do aplikacji dla tego adresu email
   - `SMTP_HOST` - SMTP host Twojej poczty (jeśli używasz Gmaila to będzie `smtp.gmail.com`)
   - `EMAIL_SUBJECT` - temat wysyłanych emaili
3. Podpisz i zmodyfikuj treść emaili znajdujących się w folderze `resources/email-bodies`
4. Odpal komendę `cargo build`
5. Odpal komendę `cargo run`

## Znane problemy

Gmail ma dzienny limit wysyłania emaili przy około 500 emailach więc używając jednego konta wyślesz tylko część emaili (ok. 90% więc i tak sporo). Możesz wysłać ile dasz radę na jednym koncie, a następnie usunąć użyte pliki `.csv` z folderu `resources/politicians`, stworzyć nowe hasło do aplikacji dla innego konta Gmail, podmienić `SENDER_ADDRESS` i `SMTP_PASSWORD` na nowe dane do logowania i odpalić skrypt ponownie. Fix WIP

## 🇬🇧 ENG

Send maaany emails to Polish politicians to help Global Sumud Flotilla

## Prerequisites

- Gmail account (Maybe you can use other email providers, but I tested with gmail)
- Two-factor authentication enabled on the account
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
3. Sign and modify the email bodies from the `resources/email-bodies` directory to your needs
4. Run `cargo build`
5. Run `cargo run`

## Known issues

Gmail has daily user sending limit which is around 500 mails so using one account you can only send part of the mails (which is still ~90% of them). You can send as many you can on one Google account and then remove used `.csv` files from the `resources/politicians` directory, create new app password on a different Google account, change `SENDER_ADDRESS` and `SMTP_PASSWORD` for the new credentials and run the script again. Fix for that WIP
