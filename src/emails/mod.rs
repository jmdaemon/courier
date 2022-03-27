extern crate imap;
extern crate native_tls;

pub mod email {
    pub fn fetch_inbox_top(host: &str, username: &str,
        password: &str, port: u16) -> imap::error::Result<Option<String>> {

        let tls = native_tls::TlsConnector::builder().build().unwrap();
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
}
