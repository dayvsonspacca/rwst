use clap::Parser;
use colored::Colorize;
use std::time::Instant;
use tungstenite::connect;

fn main() {
    let host = Args::parse().host;
    let now = Instant::now();

    println!("{}", format!("Connecting to {}...", host.cyan()).magenta());

    let (mut socket, _) = connect(&host).unwrap_or_else(|e| {
        eprintln!(
            "{} {}",
            format!("{}", "Failed to connect, error:".red()),
            format!("{}", e).red()
        );
        std::process::exit(0);
    });

    println!(
        "{}",
        format!(
            "Connection with {} established successfully! {}",
            host.cyan(),
            format!("Took {:?}", now.elapsed()).italic().bright_yellow()
        )
        .magenta()
    );

    loop {
        let cmd = inquire::Select::new("Enter Command: ", vec!["Ping", "Listen", "Send", "Exit"])
            .with_help_message("Enter a valid command")
            .prompt()
            .unwrap();

        match cmd {
            "Ping" => {
                let now = Instant::now();
                if let Err(e) = socket.send(tungstenite::Message::Text("Ping".into())) {
                    eprintln!("{} {}", "Error trying to ping: ".red(), e);
                } else {
                    println!(
                        "{} {}",
                        "Pong".magenta(),
                        format!("Took {:?}", now.elapsed()).yellow()
                    );
                }
            }
            "Listen" => loop {
                match socket.read() {
                    Err(e) => {
                        eprintln!("{} {}", "Error trying to listen: ".red(), e);
                        break;
                    }
                    Ok(msg) => {
                        println!("{} {}", "Message recived: ".magenta(), msg)
                    }
                }
            },
            "Send" => {
                let message = inquire::Text::new("Message: ")
                    .with_help_message("Enter the message to be sent:")
                    .prompt()
                    .unwrap();

                let now = Instant::now();
                if let Err(e) = socket.send(tungstenite::Message::Text(message.clone().into())) {
                    eprintln!("{} {}", "Error trying to send message: ".red(), e);
                } else {
                    println!(
                        "{} {} {}",
                        "Message sent: ".magenta(),
                        message.green(),
                        format!("Took {:?}", now.elapsed()).yellow()
                    );
                }
            }
            "Exit" => {
                println!("{}", format!("Disconnecting from {}", host.cyan()).blue());
                if let Err(e) = socket.close(None) {
                    eprintln!("Failed to close connection: {}", format!("{}", e).red());
                }
                println!("{}", "You chose to exit, bye 👋 see you later.".magenta());
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Test websockets servers via CLI", long_about = None)]
struct Args {
    #[arg(required = true)]
    host: String,
}
