#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::process::{Child, Command, Stdio};
    use std::time::Instant;
    use std::{env, io};

    use reqwest::StatusCode;

    // Default timeout when waiting for wws to be ready
    static DEFAULT_MAX_TIMEOUT: u64 = 30;

    fn get_wws_path() -> PathBuf {
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

        let binary = if cfg!(target_os = "windows") {
            "wws"
        } else {
            "wws.exe"
        };

        // Use release when it's available
        let wws_path = if path.join("target/release").join(binary).exists() {
            path.join("target/release").join(binary)
        } else {
            path.join("target/debug").join(binary)
        };

        println!("[E2E] Running wws from {}", wws_path.display());

        wws_path
    }

    fn run(example_path: &str, max_timeout: u64) -> io::Result<Child> {
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
        let mut child = Command::new(&wws_path)
            .arg(&example_path)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        // Set a max timeout
        let instant = Instant::now();

        for line in reader.lines() {
            let line = line.unwrap();

            // Break when ready of after the timeout
            if line.contains("Start serving requests") {
                break;
            } else if instant.elapsed().as_secs() >= max_timeout {
                println!("Timeout waiting for wws to be ready");
                break;
            }
        }

        Ok(child)
    }

    fn request_body(url: &str) -> Result<String, reqwest::Error> {
        reqwest::blocking::get(url)?.text()
    }

    fn request_status(url: &str) -> Result<StatusCode, reqwest::Error> {
        Ok(reqwest::blocking::get(url)?.status())
    }

    // Check the examples/js-json works
    fn run_end_to_end_test(example: &str, max_timeout: u64, url: &str, expected_text: &str) {
        println!("[E2E] Running example: {example}");

        let mut child = run(example, max_timeout).expect("Failed to execute command");

        // sleep_for(waiting_seconds);

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
        assert!(
            body.contains(expected_text),
            "result \"{body}\" does not contain \"{expected_text}\""
        );
    }

    /// Runs the given example and ensures it returns a 404 error.
    fn run_not_found_test(example: &str, max_timeout: u64, urls: &[&str]) {
        println!("[E2E] Running example (not found): {example}");
        let mut codes = Vec::new();

        let mut child = run(example, max_timeout).expect("Failed to execute command");

        for url in urls {
            let code = match request_status(url) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("[E2E] Error getting the StatusCode from the request to {url}");
                    eprintln!("[E2E] Error: {}", err);

                    // Make it fail
                    StatusCode::FOUND
                }
            };

            codes.push((url, code));

            println!("[E2E] URL: {url} / Status code: {code}");
        }

        println!("[E2E] Stopping wws process [{}]", &child.id());
        child.kill().expect("Error stopping wws");

        // Assert all of them at the same time to avoid leaving an instance running
        for (_url, code) in codes {
            // Test
            assert!(matches!(code, StatusCode::NOT_FOUND));
        }
    }

    #[test]
    // Use this approach to run tests sequentially
    fn test_end_to_end() {
        // Allow configuring waiting times. It avoids having long waiting times
        // in development, while making it configurable in the CI
        let max_timeout = env::var("E2E_MAX_WAITING_TIME").map_or(DEFAULT_MAX_TIMEOUT, |str| {
            str.parse::<u64>().ok().unwrap_or(DEFAULT_MAX_TIMEOUT)
        });

        let tests = [
            (
                "rust-basic",
                "http://localhost:8080/rust-basic",
                "This page was generated by a Wasm module built from Rust",
            ),
            (
                &format!("components{}rust-basic", std::path::MAIN_SEPARATOR_STR),
                "http://localhost:8080/rust-basic",
                "This page was generated by a Wasm module built from Rust",
            ),
            ("rust-kv", "http://localhost:8080/rust-kv", "Counter: 0"),
            (
                &format!("components{}rust-kv", std::path::MAIN_SEPARATOR_STR),
                "http://localhost:8080/rust-kv",
                "Counter: 0",
            ),
            (
                "rust-params",
                "http://localhost:8080/thisisatest",
                "thisisatest",
            ),
            (
                &format!("components{}rust-params", std::path::MAIN_SEPARATOR_STR),
                "http://localhost:8080/thisisatest",
                "thisisatest",
            ),
            (
                "js-basic",
                "http://localhost:8080",
                "This page was generated by a JavaScript file",
            ),
            (
                "js-async",
                "http://localhost:8080",
                "This page was generated by a JavaScript (async worker) file",
            ),
            (
                "js-json",
                "http://localhost:8080/handler",
                "This message comes from an environment variable",
            ),
            (
                "js-params",
                "http://localhost:8080/thisisatest",
                "thisisatest",
            ),
            (
                "js-params",
                "http://localhost:8080/main.css",
                "This is just a comment for testing purposes",
            ),
            (
                "python-basic",
                "http://localhost:8080/",
                "This page was generated by a Python script",
            ),
            (
                "python-mount",
                "http://localhost:8080/",
                "This page was loaded from a mounted file",
            ),
            (
                "ruby-basic",
                "http://localhost:8080/",
                "This page was generated by a Ruby script",
            ),
        ];

        for (example, url, expected_text) in tests {
            run_end_to_end_test(example, max_timeout, url, expected_text);
        }
    }

    #[test]
    fn test_not_found_examples() {
        // Allow configuring waiting times. It avoids having long waiting times
        // in development, while making it configurable in the CI
        let max_timeout = env::var("E2E_MAX_WAITING_TIME").map_or(DEFAULT_MAX_TIMEOUT, |str| {
            str.parse::<u64>().ok().unwrap_or(DEFAULT_MAX_TIMEOUT)
        });
        let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let cargo_toml = path.join("Cargo.toml");
        let cargo_toml_str = cargo_toml.to_string_lossy().to_string();

        // Test we're not exposing wrong files
        let tests = [
            (
                "js-basic",
                Vec::from([
                    "http://localhost:8080/index.js",
                    "http://localhost:8080/examples/js-basic/index.js",
                    "http://localhost:8080/Cargo.toml",
                    // Check path traversal issues
                    "http://localhost:8080/../README.md",
                    "http://localhost:8080/%C0AE%C0AE%C0AFREADME.md",
                    "http://localhost:8080/%2e%2e/README.md",
                    cargo_toml_str.as_str(),
                ]),
            ),
            (
                "js-json",
                Vec::from([
                    "http://localhost:8080/handler.js",
                    "http://localhost:8080/examples/js-json/handler.js",
                    "http://localhost:8080/Cargo.toml",
                    "http://localhost:8080/handler.toml",
                    // Check path traversal issues
                    "http://localhost:8080/../README.md",
                    "http://localhost:8080/%C0AE%C0AE%C0AFREADME.md",
                    "http://localhost:8080/%2e%2e/README.md",
                    cargo_toml_str.as_str(),
                ]),
            ),
        ];

        for (example, urls) in tests {
            run_not_found_test(example, max_timeout, &urls);
        }
    }
}
