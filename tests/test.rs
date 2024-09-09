#[cfg(test)]
mod tests {
    use std::process::Command;


    #[test]
    fn test_ospect_runs() {
        let result = Command::new("target/release/ospect") // Specify the binary path
            .output()
            .expect("Failed to execute command (BASIC)");
        assert!(result.status.success());
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
}
