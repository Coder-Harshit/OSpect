use colored::*;
use clap::Command;
use sysinfo::{NetworkExt, ProcessorExt, System, SystemExt};
use whoami;

fn main() {
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    let matches = Command::new("ospect")
    .about("OSpect: Comprehensive System Insights utility tool")
    .version("0.1.0")
    .subcommand(
        Command::new("all")
            .about("Display all system information")
    )
    .subcommand(
        Command::new("hardware")
            .about("Display hardware information")
    )
    .subcommand(
        Command::new("network")
            .about("Display network information")
    )
    .subcommand(
        Command::new("os")
            .about("Display OS information")
    )
    .get_matches();

    // Handle subcommands or default behavior

    match matches.subcommand() {
        Some(("all", _)) => {
            // println!("'all' subcommand used");
            print_basic_info(&mut system);
            print_hardware_info(&mut system);
            print_network_info(&mut system);
            print_os_info(&mut system);
        },
        Some(("hardware", _)) => {
            // println!("'hardware' subcommand used");
            print_hardware_info(&mut system);
        },
        Some(("network", _)) => {
            // println!("'network' subcommand used");
            print_network_info(&mut system);
        },
        Some(("os", _)) => {
            // println!("'os' subcommand used");
            print_os_info(&mut system);
        },
        // None => println!("ospect command used without subcommands"), // Default behavior
        None => print_basic_info(&mut system), // Default behavior
        _ => println!("Invalid subcommand"), // Handle unknown subcommands (optional)
    }

}

fn print_basic_info(system: &mut System) {
    println!("{}", "Basic System Information".yellow());
    println!("{}: {}", "Username".blue(), whoami::username());
    println!("{}: {}", "Hostname".blue(), system.host_name().unwrap_or_default());
    println!("{}: {}", "OS".blue(), system.name().unwrap_or_default());
    println!("{}: {}", "Kernel version".blue(), system.kernel_version().unwrap_or_default());
    println!("{}: {} GB", "Total RAM".blue(), system.total_memory() / 1024 / 1024);
    // Add more basic info as needed
}

fn print_hardware_info(system: &mut System) {
    println!("{}", "Detailed Hardware Information".yellow());
    println!("{}: {} GB", "Total Memory".blue(), system.total_memory() / 1024 / 1024);
    println!("{}: {}", "Total CPU Cores".blue(), system.processors().len());
    for processor in system.processors() {
        println!("{}: {}", "CPU Model".blue(), processor.name());
        println!("{}: {} MHz", "CPU Speed".blue(), processor.frequency());
    }
}
fn print_network_info(system: &mut System) {
    println!("{}", "Network Information".yellow());
    for (interface_name, network) in system.networks() {
        println!("{}: {}", "Network Interface".blue(), interface_name);
        println!("{}: {} Mbps", "Received Data".blue(), network.received() as f64 / 1024.0 / 1024.0);
        println!("{}: {} Mbps", "Transmitted Data".blue(), network.transmitted() as f64 / 1024.0 / 1024.0);
    }
}

fn print_os_info(system: &mut System) {
    println!("{}", "Detailed OS Information".yellow());
    println!("{}: {}", "OS Name".blue(), system.name().unwrap_or_default());
    println!("{}: {}", "Kernel Version".blue(), system.kernel_version().unwrap_or_default());
    // Add more OS info as needed
}
