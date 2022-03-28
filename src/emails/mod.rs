extern crate imap;
extern crate native_tls;

use imap::types::{Fetch, ZeroCopy};

#[derive(Default, Debug)]
pub struct Email {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16
}

impl Email {
    pub fn new_from_str(host: &str, username: &str, password: &str, port: u16) -> Email {
        let host = host.to_string(); 
        let username = username.to_string();
        let password = password.to_string();
        Email { host, username, password, port }
    }

    pub fn new(host: String, username: String, password: String, port: u16) -> Email {
        Email { host, username, password, port }
    }
}

/// Extract the body of an email message
pub fn extract_msg (message: &Fetch) -> String {
    let body = message.body().expect("Message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("Message was not valid utf-8")
        .to_string();
    body
}

/// Fetches messages from an inbox up to some number
pub fn fetch_inbox(email: Email, num_to_fetch: &str)
    -> imap::error::Result<Option<ZeroCopy<Vec<Fetch>>>> {

    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let host = email.host.as_str();
    let client = imap::connect((host, email.port), host, &tls).unwrap();

    let mut imap_session = client
        .login(email.username, email.password)
        .map_err(|e| e.0)?;

    // We want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // Fetch messages in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch(num_to_fetch, "RFC822")?;
    imap_session.logout()?;
    Ok(Some(messages))
}


pub fn fetch_inbox_top(email:Email) -> imap::error::Result<Option<String>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let host = email.host.as_str();
    let client = imap::connect((host, email.port), host, &tls).unwrap();

    let mut imap_session = client
        .login(email.username, email.password)
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
