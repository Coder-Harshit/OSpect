#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_ospect_runs() {
        let result = Command::new("target/release/ospect") // Specify the binary path
            .output()
            .expect("Failed to execute command");
        assert!(result.status.success());
    }

    #[test]
    fn test_ospect_all_runs() {
        let result = Command::new("target/release/ospect") // Specify the binary path
            .arg("all")
            .output()
            .expect("Failed to execute command");
        assert!(result.status.success());
    }
}
