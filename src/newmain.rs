use std::process::{Command, Stdio};
use std::io::{self, Read};

fn call_curl(url: &str) -> io::Result<()> {
    let mut curl_cmd = Command::new("curl")
        .arg("-k") // Disable cert verification for testing
        .arg("--tlsv1.0") // Enforce TLS 1.0
        .arg(url)
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdout) = curl_cmd.stdout.take() {
        let mut response = String::new();
        stdout.read_to_string(&mut response)?;
        println!("Response: {}", response);
    }

    let status = curl_cmd.wait()?;
    println!("Exited with: {}", status);
    Ok(())
}

fn main() {
    let url = "https://controller.access.network/101/portal";
    if let Err(e) = call_curl(url) {
        eprintln!("Error: {}", e);
    }
}
