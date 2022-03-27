use clap::{Arg, App, AppSettings};

pub fn init_cli() -> App<'static> {
    let app = App::new("Courier")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com")
        .about("Lightweight customizable email client")
        .arg(Arg::new("emails").help("Use an emails.toml file"))
        .arg(Arg::new("v").help("Show verbose output"))
        .arg(Arg::new("host").help("The email provider"))
        .arg(Arg::new("username").help("Your email address"))
        .arg(Arg::new("password").help("Your email password"))
        .arg(Arg::new("port").help("The port to listen for emails on"));
        app
}
