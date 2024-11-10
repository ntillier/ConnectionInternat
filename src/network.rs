use std::time::Duration;
use std::io::{stdout, Write};

use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, exit};
use std::env;

use miniserde::json::{self, Value};

use crate::config::Config;

const LINUX_SCRIPT: &[u8] = include_bytes!("../assets/linux.sh");
const OPENSSL_CONFIG: &[u8] = include_bytes!("../assets/openssl_config.cnf");

const LINUX_PATH: &str = "internat/linux.sh";
const OPENSSL_CONFIG_PATH: &str = "internat/openssl_config.cnf";

pub struct CustomClient{
    executor: Option<String>,
}

fn check_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .map(|ext| ext == extension)
        .unwrap_or(false)
}

impl CustomClient {

    pub fn new(config: &Config) -> Self {
        let mut executor = None;

        if config.update {
            let files = vec![
                (LINUX_PATH, LINUX_SCRIPT),
                (OPENSSL_CONFIG_PATH, OPENSSL_CONFIG)
            ];

            for &(path, content) in files.iter() {
                let path = env::temp_dir().join(path);
                fs::create_dir_all(path.parent().unwrap()).unwrap();

                let mut file = File::create(&path).expect("[ERREUR]: Impossible de créer le fichier.\n");
                file.write_all(content).unwrap();

                #[cfg(unix)]
                if check_extension(&path, "sh") {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&path).expect("[ERREUR]: Impossible d'accéder à l'exécutable.\n").permissions();
                    perms.set_mode(0o755); // Allow execute permissions
                    fs::set_permissions(&path, perms).expect("[ERREUR]: Impossible de changer les permission.\n");
                }
            }
        }

        #[cfg(unix)]
        {
            executor = Some(LINUX_PATH.to_string());
        }

        return Self {
            executor
        };
    }

    fn request(&self, url: &str, method: &str, headers: &str, body: &str) -> (bool, String) {
        if let Some(path) = &self.executor {
            let path = env::temp_dir().join(path);

            if path.exists() {
                let output = Command::new("sh")
                        .arg(path.as_os_str().to_str().expect("[ERREUR]: Impossible d'exécuter la requête."))
                        .arg(url)
                        .arg(method)
                        .arg(headers)
                        .arg(body)
                        .output()
                        .expect("[ERREUR]: Impossible d'exécuter le script.\n");

                if output.status.success() {
                    return (true, String::from_utf8_lossy(&output.stdout).to_string());
                }
            }
        }

        return (false, String::new());
    }

    pub fn login(&self, username: &str, password: &str) -> (bool, String) {
        let (ok, result) = self.request(
            "https://controller.access.network/portal_api.php",
            "POST",
            "Content-Type: application/x-www-form-urlencoded",
            &format!("action=authenticate&login={}&password={}&policy_accept=false", username, password)
        );

        let value: Value = json::from_str(&result).expect("[ERREUR]: Impossible de parser le json");

        match value {
            Value::Object(item) => println!("{:?}", item),
            _ => {}
        }

        /*if let Value::Object(user) = &value["user"] {
            println!("Name: {:?}", user);
        }*/

        //println!("{}", value["user"]["passwordDigest"]["value"]);

        return (false, String::new());
        // https://controller.access.network/portal_api.php
        /*let response = self.client
            //.get("https://controller.access.network/101/portal")
            .post("https://controller.access.network/portal_api.php") // replace with your target URL
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("action=authenticate&login={}&password={}&policy_accept=false", username, password))
            .send()?;

        println!("done");

        //println!("Status: {}", response.clone().unwrap().status());

        println!("Status: {}", response.status());

        //if let Ok(res) = response {
            if let Ok(text) = response.text() {
                println!("Response: {:?}", text);
            }
            //}*/
    }
}
