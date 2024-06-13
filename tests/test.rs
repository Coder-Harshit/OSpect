#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*; // Import the functions from the parent module

    #[test]
    fn test_ospect_runs() {
        let result = Command::new("./ospect")
            .output()
            .expect("Failed to execute command");
        assert!(result.status.success());
    }

    #[test]
    fn test_ospect_all_runs() {
        let result = Command::new("./ospect")
            .arg("all")
            .output()
            .expect("Failed to execute command");
        assert!(result.status.success());
    }
}