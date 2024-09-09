#[cfg(test)]
    mod tests {
        use std::process::Command;
        use std::env;

        #[test]
        fn test_ospect_runs() {
            let binary_path = env::var("OSPECT_BINARY_PATH").unwrap_or("target/release/ospect".to_string()); // Adjust if needed
            let result = Command::new(binary_path)
                .output()
                .expect("Failed to execute command");

            assert!(result.status.success(), "Command failed with exit code: {}", result.status.code().unwrap_or(-1));

            // Optional: Check output or stderr
            if let Ok(output) = String::from_utf8(result.stdout) {
                // Assert on output content
                assert!(output.contains("Basic System Information"));
            }

            if let Ok(stderr) = String::from_utf8(result.stderr) {
                // Check for error messages
                assert!(!stderr.contains("error"));
            }
        }
    }
//     #[test]
//     fn test_ospect_network_runs() {
//         let result = Command::new("target/release/ospect") // Specify the binary path
//             .arg("network")
//             .output()
//             .expect("Failed to execute command (NETWORK)");
//         assert!(result.status.success());
//     }

//     #[test]
//     fn test_ospect_os_runs() {
//         let result = Command::new("target/release/ospect") // Specify the binary path
//             .arg("os")
//             .output()
//             .expect("Failed to execute command (OS)");
//         assert!(result.status.success());
//     }

//     #[test]
//     fn test_ospect_hardware_runs() {
//         let result = Command::new("target/release/ospect") // Specify the binary path
//             .arg("hardware")
//             .output()
//             .expect("Failed to execute command (HARDWARE)");
//         assert!(result.status.success());
//     }


//     #[test]
//     fn test_ospect_all_runs() {
//         let result = Command::new("target/release/ospect") // Specify the binary path
//             .arg("all")
//             .output()
//             .expect("Failed to execute command (ALL)");
//         assert!(result.status.success());
//     }
