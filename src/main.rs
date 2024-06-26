use colored::*;
use clap::Command;
use sysinfo::{CpuRefreshKind, RefreshKind, System ,Networks};
use whoami;
use std::{env, fs};
use std::path::PathBuf;

fn get_logo_path() -> PathBuf {
    let mut project_root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
    project_root_dir.push("assets");
    project_root_dir.push("logo.txt");
    project_root_dir
}

fn read_logo(path: PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "Logo not found".red().to_string())}


fn format_uptime(uptime: u64) -> String {
    let (secs, mins, hours, days) = (uptime % 60, (uptime / 60) % 60, (uptime / 3600) % 24, uptime / (3600 * 24));

    if days > 0 {
        format!("{} day{}", days, if days > 1 { "s" } else { "" })
    } else if hours > 0 {
        format!("{} hour{}", hours, if hours > 1 { "s" } else { "" })
    } else if mins > 0 {
        format!("{} minute{}", mins, if mins > 1 { "s" } else { "" })
    } else {
        format!("{} second{}", secs, if secs > 1 { "s" } else { "" })
    }
}

fn format_network_data(mut data_bytes: u64) -> String {
    if data_bytes == 0 {
      return "0 B".to_string();
    }
  
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut power = 0;
  
    while data_bytes > 1024 && power < units.len() - 1 {
      data_bytes /= 1024;
      power += 1;
    }
  
    format!("{:.1} {}", data_bytes as f64 / 1024.0f64.powf(power as f64), units[power])
  }
  

fn main() {
    let mut system = System::new_all();

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


    // Read ASCII logo
    let logo_path = get_logo_path();
    let logo = read_logo(logo_path);
    println!("{}", logo);  // Display the logo


    // Handle subcommands or default behavior
    match matches.subcommand() {
        Some(("all", _)) => {
            // println!("'all' subcommand used");
            print_basic_info(&mut system);
            print_hardware_info(&mut system);
            print_network_info();
            print_os_info();
        },
        Some(("hardware", _)) => {
            // println!("'hardware' subcommand used");
            print_hardware_info(&mut system);
        },
        Some(("network", _)) => {
            // println!("'network' subcommand used");
            print_network_info();
        },
        Some(("os", _)) => {
            // println!("'os' subcommand used");
            print_os_info();
        },
        None => print_basic_info(&mut system), // Default behavior
        _ => println!("Invalid subcommand"), // Handle unknown subcommands (optional)
    }

}

fn print_basic_info(system: &mut System) {
    println!("{}", "Basic System Information".yellow());
    println!("{}: {}", "Username".blue(), whoami::username());
    println!("{}: {}", "Hostname".blue(), sysinfo::System::host_name().unwrap_or_default());
    println!("{}: {}", "OS".blue(), sysinfo::System::name().unwrap_or_default());
    println!("{}: {}", "Kernel version".blue(), sysinfo::System::kernel_version().unwrap_or_default());
    println!("{}: {} GB", "Total RAM".blue(), system.total_memory() /1024 /1024 /1024);
    println!("{}: {}", "CPU Architecture".blue(), std::env::consts::ARCH);
}

fn print_hardware_info(system: &mut System) {
    println!("{}", "Detailed Hardware Information".yellow());
    println!("{}: {} GB", "Total Memory".blue(), system.total_memory() / 1024 / 1024);
    println!("{}: {}", "Total CPU Cores".blue(), system.physical_core_count().unwrap_or_default());
    let s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );
    let mut i=0;
    let mut is_first_cpu = true;
    for cpu in s.cpus() {
        if is_first_cpu {
            println!("{}: {}", "CPU Model".blue(), cpu.brand());
            is_first_cpu = false;
        }
        println!("{} {}: {} MHz", "CPU Speed".blue(), i, cpu.frequency());
        println!("{} {}: {}%", "CPU Usage".blue(), i, cpu.cpu_usage());
        i+=1;
    }
}

fn print_network_info() {
    println!("{}", "Network Information".yellow());
    let networks = Networks::new_with_refreshed_list();
    for (interface_name , network) in &networks {
        println!("[{interface_name}]"); 
        println!("MAC Address: {}", network.mac_address());
        println!("Total Packets Received: {}", format_network_data(network.total_packets_received()));
        println!("Total Packets Transmitted: {}", format_network_data(network.total_packets_transmitted()));
        println!();
    }
}

fn print_os_info() {
    println!("{}", "Detailed OS Information".yellow());
    println!("{}: {}", "OS Name".blue(), sysinfo::System::name().unwrap_or_default());
    println!("{}: {}", "Kernel Version".blue(), sysinfo::System::kernel_version().unwrap_or_default());
    println!("{}: {}", "System Uptime".blue(), format_uptime(sysinfo::System::uptime()));
}