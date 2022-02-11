use clap::{Arg, App, AppSettings};

fn main() {
    let matches = App::new("Optim")
        .version("0.1.0")
        .author("Joseph Diza")
        .about("Lightweight, extremely customizable email client")
        .arg(Arg::new("v").help("Show verbose output"))
        .arg(Arg::new("host").help("The email provider"))
        .arg(Arg::new("username").help("Your email address"))
        .arg(Arg::new("password").help("Your email password"))
        .arg(Arg::new("port").help("The port to listen for emails on"))
        .get_matches();
}
