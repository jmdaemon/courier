use log::{debug, error, info, warn};
use anyhow::Result;
use toml::Value;
use std::fs::read_to_string;

use libcourier::cli::init_cli;
use libcourier::emails::fetch_inbox_top;
use libcourier::ui::{init_courier, run_app};

pub struct Email {
    host: String,
    username: String,
    password: String,
    port: u16
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

pub fn parse_config(emailcfg: Value, key: &str, msg: &str) -> String {
    emailcfg[key].as_str().expect(msg).to_string()
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let app = init_cli();

    let matches = app.get_matches();

    let cfg = matches.value_of("emails");

    let config: Value;
    let email: Email;

    let host: &str;
    let username: &str;
    let password: &str;
    let port;

    match cfg {
        Some(_) => {
            // Use the credentials found in the config file
            let contents = read_to_string(cfg.unwrap()).expect("Could not parse emails.toml");
            config = toml::from_str(&contents).unwrap();
            let emails = &config["emails"];

            // For now we'll just pick the only email available
            // which is the first one.
            let emailcfg = &emails[0];

            host = &emailcfg["host"].as_str().expect("Could not parse host.");
            username = &emailcfg["username"].as_str().expect("Could not parse username.");
            password = &emailcfg["password"].as_str().expect("Could not parse password.");
            port = (&emailcfg["port"].as_integer()).unwrap() as u16;
            email = Email::new_from_str(host, username, password, port);
        }
        None => {
            // Use the credentials passed in as cli parameters
            host = matches.value_of("host").unwrap_or("imap.gmail.com");
            username = matches.value_of("username").expect("Username not found.");
            password = matches.value_of("password").expect("Password not found.");
            port = matches.value_of("port").unwrap_or("993").parse::<u16>().unwrap();
            email = Email::new_from_str(host, username, password, port);
        }
    }
    
    info!("Hostname: {}", host);
    info!("Username: {}", username);
    info!("Password: {}", password);
    info!("Port: {}", port);

    // Initialize and run the app
    let app = init_courier();
    run_app(&app);

    let msg = fetch_inbox_top(host, username, password, port).unwrap().unwrap();
    println!("{}", msg);

    Ok(())
}
