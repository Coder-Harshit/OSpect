use colored::*;
use clap::{Arg,Command};
use sysinfo::{NetworkExt, ProcessorExt, System, SystemExt};
use whoami;

fn main() {
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    let matches = Command::new("ospect")
        .about("Display system information")
        .arg(Arg::new("all")
            .short('a')
            .long("all")
            .help("Display all system information"))
        .arg(Arg::new("hardware")
            // .short('h')
            .long("hardware")
            .help("Display hardware information"))
        .arg(Arg::new("network")
            // .short('n')
            .long("network")
            .help("Display network information"))
        .arg(Arg::new("os")
            // .short('o')
            .long("os")
            .help("Display OS information"))
        .get_matches();

    match matches.subcommand() {
        Some((subcommand, _)) => { // Destructure the tuple, ignore ArgMatches
            match subcommand {
                "all" => {
                    print_basic_info(&mut system);
                    print_hardware_info(&mut system);
                    print_network_info(&mut system);
                    print_os_info(&mut system);
                },
                "hardware" => print_hardware_info(&mut system),
                "network" => print_network_info(&mut system),
                "os" => print_os_info(&mut system),
                _ => println!("Invalid subcommand"), // Handle unknown subcommands (optional)
            }
        },
        None => print_basic_info(&mut system), // Default behavior
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
