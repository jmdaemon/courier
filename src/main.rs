use clap::{Arg, App, AppSettings};
use log::{debug, error, info, warn};
use gtk::{prelude::*, Application, ApplicationWindow};
use anyhow::Result;
use toml::Value;
use std::fs::read_to_string;

use emails::email::fetch_inbox_top;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let matches = App::new("Courier")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com")
        .about("Lightweight customizable email client")
        .arg(Arg::new("emails").help("Use an emails.toml file"))
        .arg(Arg::new("v").help("Show verbose output"))
        .arg(Arg::new("host").help("The email provider"))
        .arg(Arg::new("username").help("Your email address"))
        .arg(Arg::new("password").help("Your email password"))
        .arg(Arg::new("port").help("The port to listen for emails on"))
        .get_matches();

    let cfg = matches.value_of("emails");

    let config: Value;
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
            let email = &emails[0];

            host = &email["host"].as_str().expect("Could not parse host.");
            username = &email["username"].as_str().expect("Could not parse username.");
            password = &email["password"].as_str().expect("Could not parse password.");
            //port = (&emails["port"].to_string()).parse::<u16>().unwrap();
            //port = (&emails["port"].as_integer()).parse::<u16>().unwrap();
            port = (&email["port"].as_integer()).unwrap() as u16;

        }
        None => {
            // Use the credentials passed in as cli parameters
            host = matches.value_of("host").unwrap_or("imap.gmail.com");
            username = matches.value_of("username").expect("Username not found.");
            password = matches.value_of("password").expect("Password not found.");
            port = matches.value_of("port").unwrap_or("993").parse::<u16>().unwrap();
        }
    }
    
    info!("Hostname: {}", host);
    info!("Username: {}", username);
    info!("Password: {}", password);
    info!("Port: {}", port);

    // GTK4 
    let app = Application::builder()
        .application_id("jmdaemon.github.io.courier")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Accept command line arguments but don't do anything
    // This is a temporary hack to be able to pass in command line arguments
    // Run the application
    app.run_with_args(&[host]);

    let msg = fetch_inbox_top(host, username, password, port).unwrap().unwrap();
    println!("{}", msg);

    Ok(())
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Courier")
        .default_width(1920)
        .default_height(1080)
        .build();

    // Present window
    window.present();
}
