use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::time::Duration;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Redirection Detector")]
struct Cli {
    /// Sets the timeout for the request in seconds
    #[arg(short, long, default_value = "5")]
    timeout: Option<u64>,

    /// Sets the URL to detect redirection
    url: String,
}

struct RedirectionDetector {
    client: Client,
}

impl RedirectionDetector {
    fn new(timeout: Option<Duration>) -> Self {
        let client = Client::builder()
            .redirect(Policy::none())
            .timeout(timeout)
            .build()
            .expect("Failed to create client");
        RedirectionDetector { client }
    }

    fn detect(&self, url: &str) -> Result<Option<String>, String> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|err| format!("Error sending request: {}", err))?;

        if response.status().is_redirection() {
            if let Some(redirect_uri) = response.headers().get("Location") {
                if let Ok(redirect_uri_str) = redirect_uri.to_str() {
                    return Ok(Some(redirect_uri_str.to_owned()));
                }
            }
        }

        Ok(None)
    }
}

fn print_colored_message(message: &str, color_code: u8) {
    // ANSI escape codes for coloring the output
    const COLOR_START: &str = "\x1B[38;5;";
    const COLOR_END: &str = "m";
    const RESET_COLOR: &str = "\x1B[0m";

    let colored_message = format!("{}{}{}", COLOR_START, color_code, COLOR_END);
    println!("{}{}{}", colored_message, message, RESET_COLOR);
}

fn main() {
    let args = Cli::parse();

    let timeout_secs = args.timeout.map(Duration::from_secs);

    let detector = RedirectionDetector::new(timeout_secs);

    // https://www.ditig.com/256-colors-cheat-sheet
    match detector.detect(&args.url) {
        Ok(Some(uri)) => print_colored_message(&format!("Redirect URI: {}", uri), 41), // SpringGreen3	#00d75f
        Ok(None) => print_colored_message("No redirect occurred.", 39), // DeepSkyBlue1	#00afff
        Err(err) => print_colored_message(&format!("Error: {}", err), 202), //OrangeRed1	#ff5f00
    }
}
