#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::process::{Child, Command, Stdio};
    use std::{env, io, thread, time};

    #[cfg(not(target_os = "windows"))]
    fn get_wws_path() -> PathBuf {
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

        // Use release when it's available
        let wws_path = if path.join("target/release/wws").exists() {
            path.join("target/release/wws")
        } else {
            path.join("target/debug/wws")
        };

        println!("[E2E] Running wws from {}", wws_path.display());

        wws_path
    }

    #[cfg(target_os = "windows")]
    fn get_wws_path() -> PathBuf {
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

        // Use release when it's available
        let wws_path = if path.join("target/release/wws.exe").exists() {
            path.join("target/release/wws.exe")
        } else {
            path.join("target/debug/wws.exe")
        };

        println!("[E2E] Running wws from {}", wws_path.display());

        wws_path
    }

    fn run(example_path: &str) -> io::Result<Child> {
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let example_path = path.join("examples").join(example_path);
        let wws_path = get_wws_path();

        // Install missing runtimes
        println!("[E2E] Installing missing runtimes");
        Command::new(&wws_path)
            .current_dir(&example_path)
            .args(["runtimes", "install"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        // Run the example
        println!("[E2E] Running the service");
        Command::new(&wws_path)
            .arg(&example_path)
            .spawn()
    }

    fn sleep_for(seconds: u64) {
        thread::sleep(time::Duration::from_secs(seconds));
    }

    fn request_body(url: &str) -> Result<String, reqwest::Error> {
        reqwest::blocking::get(url)?.text()
    }

    // Check the examples/js-json works
    fn run_end_to_end_test(example: &str, waiting_seconds: u64, url: &str, expected_text: &str) {
        println!("[E2E] Running example: {example}");

        let mut child = run(example).expect("Failed to execute command");

        sleep_for(waiting_seconds);

        let body = match request_body(url) {
            Ok(body) => body,
            Err(err) => {
                eprintln!("[E2E] Error getting the body from the request to {url}");
                eprintln!("[E2E] Error: {}", err);
                String::new()
            }
        };

        println!("[E2E] Body content: {body}");

        println!("[E2E] Stopping wws process [{}]", &child.id());
        child.kill().expect("Error stopping wws");

        // Test
        assert!(body.contains(expected_text));
    }

    #[test]
    // Use this approach to run tests sequentially
    fn test_end_to_end() {
        // Allow configuring waiting times. It avoids having long waiting times
        // in development, while making it configurable in the CI
        let global_timeout: Option<u64> = match env::var("E2E_WAITING_TIME") {
            Ok(val) => match val.parse::<u64>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            Err(_) => None,
        };

        let tests = [
            (
                "rust-basic",
                global_timeout.unwrap_or(5),
                "http://localhost:8080/basic",
                "This page was generated by a Wasm module built from Rust",
            ),
            (
                "rust-kv",
                global_timeout.unwrap_or(5),
                "http://localhost:8080/kv",
                "Counter: 0",
            ),
            (
                "rust-params",
                global_timeout.unwrap_or(5),
                "http://localhost:8080/thisisatest",
                "thisisatest",
            ),
            (
                "js-basic",
                global_timeout.unwrap_or(5),
                "http://localhost:8080",
                "This page was generated by a JavaScript file",
            ),
            (
                "js-json",
                global_timeout.unwrap_or(5),
                "http://localhost:8080/handler",
                "This message comes from an environment variable",
            ),
            (
                "js-params",
                global_timeout.unwrap_or(10),
                "http://localhost:8080/thisisatest",
                "thisisatest",
            ),
            (
                "python-basic",
                global_timeout.unwrap_or(20),
                "http://localhost:8080/",
                "This page was generated by a Python script",
            ),
            (
                "python-mount",
                global_timeout.unwrap_or(20),
                "http://localhost:8080/",
                "This page was loaded from a mounted file",
            ),
            (
                "ruby-basic",
                global_timeout.unwrap_or(20),
                "http://localhost:8080/",
                "This page was generated by a Ruby script",
            ),
        ];

        for (example, waiting_seconds, url, expected_text) in tests {
            run_end_to_end_test(example, waiting_seconds, url, expected_text);
        }
    }
}
