extern crate imap;
extern crate native_tls;

use clap::{Arg, App, AppSettings};
use log::{debug, error, info, warn};

fn fetch_inbox_top(host: &str, username: &str,
    password: &str, port: u16) -> imap::error::Result<Option<String>> {

    let tls = native_tls::TlsConnector::builder().build().unwrap();
    //let client = imap::ClientBuilder::new(host, 993).native_tls()?;
    let client = imap::connect((host, port), host, &tls).unwrap();

    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    // We want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // Fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(Some(body))
}

fn main() -> Result<(), clap::Error> {
    //env_logger::init();
    pretty_env_logger::init();

    let matches = App::new("Courier")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com")
        .about("Lightweight customizable email client")
        .arg(Arg::new("v").help("Show verbose output"))
        .arg(Arg::new("host").help("The email provider"))
        .arg(Arg::new("username").help("Your email address"))
        .arg(Arg::new("password").help("Your email password"))
        .arg(Arg::new("port").help("The port to listen for emails on"))
        .get_matches();
    
    let host = matches.value_of("host").unwrap_or("imap.gmail.com");
    let username = matches.value_of("username").expect("Username not found.");
    let password = matches.value_of("password").expect("Password not found.");
     let port = matches.value_of("port").unwrap_or("993").parse::<u16>().unwrap();
    
    info!("Hostname: {}", host);
    info!("Username: {}", username);
    info!("Password: {}", password);
    info!("Port: {}", port);

    let msg = fetch_inbox_top(host, username, password, port).unwrap().unwrap();

    println!("{}", msg);
    Ok(())
}
