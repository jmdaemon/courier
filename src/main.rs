use clap::{Arg, App, AppSettings};
use log::{debug, error, info, warn};

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
    
    let host = matches.value_of("host").ok_or("Host not found.");
    let username = matches.value_of("username").ok_or("Username not found.");
    let password = matches.value_of("password").ok_or("Password not found.");
    let port = matches.value_of("port").unwrap_or("430");
    
    info!("Hostname: {}", host.unwrap());
    info!("Username: {}", username.unwrap());
    info!("Password: {}", password.unwrap());
    info!("Port: {}", port);
    Ok(())
}
