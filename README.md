# OSpect: Comprehensive System Diagnostics and Insights


<p align="left">
  <img src="assets/github/OSpect_github.gif" alt="OSpect Logo" width="200" />
</p>


## Overview

OSpect is a powerful and versatile system information utility tool designed to provide comprehensive diagnostics and insights into your system's hardware and software configuration. With OSpect, you can effortlessly gather detailed information about your operating system, hardware components, network status, and more.

## Features

- **Detailed OS Information**: Retrieve the name, version, and kernel details of your operating system.
- **Hardware Insights**: Get in-depth details about your CPU, RAM, storage, and other hardware components.
- **Network Statistics**: Monitor network interfaces, IP addresses, and connection statuses.
- **Customizable Output**: Use flags to customize the level of detail and specific information you want to display.
- **Visual Appeal**: Clean and colorful output designed for readability and ease of use.

## Installation

### Automated Install Script

To simplify the installation process, you can use the provided `install_ospect.sh` script. This script will install Rust, clone the OSpect repository, build the project, and run OSpect.

1. **Download and Run the Script**:
   ```sh
   curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/install_ospect.sh
   chmod +x install_ospect.sh
   ./install_ospect.sh
   ```

### Manual Installation

If you prefer to manually install OSpect, follow these steps:

1. **Install Rust and Cargo**: If you don't have Rust and Cargo installed, you can get them from [rustup.rs](https://rustup.rs/).

2. **Clone the Repository**:
   ```sh
   git clone https://github.com/Coder-Harshit/OSpect.git
   cd OSpect
   ```

3. **Build and Install**:
   ```sh
   cargo build --release
   cargo install --path .
   ```

## Usage

OSpect provides various flags to display specific system information. Here are some examples:

- **Display All Information**:
  ```sh
  ospect all
  ```

- **List Available Flags**:
  ```sh
  ospect --list
  ```

- **Show Detailed Hardware Information**:
  ```sh
  ospect hardware
  ```

- **Show Network Information**:
  ```sh
  ospect network
  ```

- **Show Detailed OS Information**:
  ```sh
  ospect os
  ```

- **Show Version Information**:
  ```sh
  ospect --version
  ```
- **Display Help**:
  ```
  ospect --help
  ```

### Available Flags

| Flag         | Description                                     |
|--------------|-------------------------------------------------|
| `--all`      | Show all available information                  |
| `--list`     | List all available flags                        |
| `--hardware` | Show detailed hardware information              |
| `--network`  | Show network information                        |
| `--os`       | Show detailed OS information                    |
| `--version`  | Show version information                        |
| `--help`     | Display usage information and available flags   |

## Contributing

We welcome contributions from the community! If you'd like to contribute to OSpect, please follow these steps:

1. **Fork the Repository**: Click the "Fork" button at the top of this page.

2. **Clone Your Fork**:
   ```sh
   git clone https://github.com/Coder-Harshit/OSpect.git
   cd OSpect
   ```

3. **Create a Branch**:
   ```sh
   git checkout -b feature-branch
   ```

4. **Make Your Changes**: Implement your feature or fix the bug.

5. **Commit and Push**:
   ```sh
   git add .
   git commit -m "Description of your changes"
   git push origin feature-branch
   ```

6. **Create a Pull Request**: Open a pull request from your forked repository's feature branch to the main repository's main branch.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

For questions or feedback, please contact [harshitvj07@gmail.com](mailto:harshitvj07@gmail.com).

---

Thank you for using OSpect! We hope you find it as useful and powerful as we do.

<!-- ![Footer Logo](https://your-footer-logo-url-here.com/logo.png) -->
