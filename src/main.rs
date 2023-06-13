use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::time::Duration;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Redirection Detector")]
struct Cli {
    /// Sets the timeout for the request in seconds
    #[arg(short, long)]
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

fn main() {
    let args = Cli::parse();

    let timeout_secs = args.timeout.map(Duration::from_secs);

    let detector = RedirectionDetector::new(timeout_secs);

    match detector.detect(&args.url) {
        Ok(Some(uri)) => println!("Redirect URI: {}", uri),
        Ok(None) => println!("No redirect occurred."),
        Err(err) => println!("Error: {}", err),
    }
}
