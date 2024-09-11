// use colored::*;
// use clap::Command;
// use sysinfo::{CpuRefreshKind, RefreshKind, System ,Networks};
// use whoami;
// use std::{env, fs};
// use std::path::PathBuf;

use std::{env, process::Command, fs, error::Error};
use colored::Colorize;
use serde::Deserialize;


#[derive(Deserialize)]
struct Config{
    basic: BasicConfig,
}

#[derive(Deserialize)]
struct BasicConfig{
    hostname: bool,
    username: bool,
    osname: bool,
    kernel: bool,
    total_memory: bool,
}


fn load_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_content = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}


// fn get_logo_path() -> PathBuf {
//     let mut project_root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
//     project_root_dir.push("assets");
//     project_root_dir.push("logo.txt");
//     project_root_dir
// }

// fn read_logo(path: PathBuf) -> String {
//     fs::read_to_string(path).unwrap_or_else(|_| "Logo not found".red().to_string())}

// fn format_uptime(uptime: u64) -> String {
//     let (secs, mins, hours, days) = (uptime % 60, (uptime / 60) % 60, (uptime / 3600) % 24, uptime / (3600 * 24));

//     if days > 0 {
//         format!("{} day{}", days, if days > 1 { "s" } else { "" })
//     } else if hours > 0 {
//         format!("{} hour{}", hours, if hours > 1 { "s" } else { "" })
//     } else if mins > 0 {
//         format!("{} minute{}", mins, if mins > 1 { "s" } else { "" })
//     } else {
//         format!("{} second{}", secs, if secs > 1 { "s" } else { "" })
//     }
// }

// fn format_network_data(mut data_bytes: u64) -> String {
//     if data_bytes == 0 {
//       return "0 B".to_string();
//     }

//     let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
//     let mut power = 0;

//     while data_bytes > 1024 && power < units.len() - 1 {
//       data_bytes /= 1024;
//       power += 1;
//     }

//     format!("{:.1} {}", data_bytes as f64 / 1024.0f64.powf(power as f64), units[power])
//   }

fn fetch_username() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("whoami")
    .output()
    .expect("Failed to execute `whoami` command");
    let username = String::from_utf8(output.stdout)?;
    let username = username.trim();
    return Ok(username.to_string());
}

fn fetch_hostname() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("hostname")
        .output()?;
    let hostname = String::from_utf8(output.stdout)?;
    let hostname = hostname.trim();
    return Ok(hostname.to_string());
}

fn fetch_osname() -> Result<String, Box<dyn std::error::Error>> {
    let output= Command::new("/bin/bash")
        .args(["-c", "/bin/head -n 1 /etc/os-release | awk -F '=' '{print $2}'"])
        .output()?;
    let osname = String::from_utf8(output.stdout)?;
    let trimmed_osname = osname.trim().trim_matches('"');
    return Ok(trimmed_osname.to_string());
}

fn fetch_kernel() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("uname")
        .arg("-r")
        .output()?;
    let kernel = String::from_utf8(output.stdout)?;
    let kernel = kernel.trim();
    return  Ok(kernel.to_string());
}

fn fetch_total_mem() -> Result<u64, Box<dyn std::error::Error>>{
    let tmem = Command::new("/bin/bash")
        .args(["-c","/bin/cat /proc/meminfo | /bin/grep MemTotal | awk '{print $2}'"])
        .output()?;
    let stdout = String::from_utf8(tmem.stdout)?;
    let tmem: u64 = stdout.trim().parse()?;

    return Ok(tmem);
}




fn print_basic_info(config_file:Config){
    println!("{}", "Basic System Information".yellow());

    if config_file.basic.username{
        match fetch_username() {
            Ok(username) => {
                println!("{}: {}", "Username".blue(), username);
            }
            Err(err) => println!("Error: {}", err),
        }
    }
    if config_file.basic.hostname{
        match fetch_hostname() {
            Ok(hostname) => {
                println!("{}: {}", "Hostname".blue(), hostname);
            }
            Err(err) => println!("Error: {}", err),
        }
    }

    if config_file.basic.osname{
        match fetch_osname() {
            Ok(osname) => {
                println!("{}: {}", "OS".blue(), osname);
            }
            Err(err) => println!("Error: {}", err),
        }
    }

    if config_file.basic.kernel{
        match fetch_kernel() {
            Ok(kernel) => {
                println!("{}: {}", "Kernel".blue(), kernel);
            }
            Err(err) => println!("Error: {}", err),
        }
    }

    if config_file.basic.total_memory{
        match fetch_total_mem() {
            Ok(total_mem) => {
                let total_mem_mb = total_mem / (1024*1024);
                println!("{}: {} GB", "Total memory".blue(), total_mem_mb);
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn main() {

    let home = env::var("HOME").expect("HOME variable not set");
    let config_path = format!("{home}/.config/ospect/config.toml") ;
    let config = load_config(&config_path).expect("Failed to load config");

    let args:Vec<String> = env::args().collect();

    // for itr in &args {
    //     println!("==> {itr}");
    // }

    match args.len() {
        1 => {
            // default option to run
            print_basic_info(config);
        },
        2 => {
            // specialized output
        },
        _ => {
            println!("Invalid Number of parameters passed");
        },
    }

    // let mut system = System::new_all();

    // let matches = Command::new("ospect")
    // .about("OSpect: Comprehensive System Insights utility tool")
    // .version("0.1.0")
    // .subcommand(
    //     Command::new("all")
    //         .about("Display all system information")
    // )
    // .subcommand(
    //     Command::new("hardware")
    //         .about("Display hardware information")
    // )
    // .subcommand(
    //     Command::new("network")
    //         .about("Display network information")
    // )
    // .subcommand(
    //     Command::new("os")
    //         .about("Display OS information")
    // )
    // .get_matches();


    // // Read ASCII logo
    // let logo_path = get_logo_path();
    // let logo = read_logo(logo_path);
    // println!("{}", logo);  // Display the logo


    // // Handle subcommands or default behavior
    // match matches.subcommand() {
    //     Some(("all", _)) => {
    //         // println!("'all' subcommand used");
    //         print_basic_info(&mut system);
    //         print_hardware_info(&mut system);
    //         print_network_info();
    //         print_os_info();
    //     },
    //     Some(("hardware", _)) => {
    //         // println!("'hardware' subcommand used");
    //         print_hardware_info(&mut system);
    //     },
    //     Some(("network", _)) => {
    //         // println!("'network' subcommand used");
    //         print_network_info();
    //     },
    //     Some(("os", _)) => {
    //         // println!("'os' subcommand used");
    //         print_os_info();
    //     },
    //     None => print_basic_info(&mut system), // Default behavior
    //     _ => println!("Invalid subcommand"), // Handle unknown subcommands (optional)
    // }

}

// fn print_basic_info(system: &mut System) {
//     println!("{}", "Basic System Information".yellow());
//     println!("{}: {}", "Username".blue(), whoami::username());
//     println!("{}: {}", "Hostname".blue(), sysinfo::System::host_name().unwrap_or_default());
//     println!("{}: {}", "OS".blue(), sysinfo::System::name().unwrap_or_default());
//     println!("{}: {}", "Kernel version".blue(), sysinfo::System::kernel_version().unwrap_or_default());
//     println!("{}: {} GB", "Total RAM".blue(), system.total_memory() /1024 /1024 /1024);
//     println!("{}: {}", "CPU Architecture".blue(), std::env::consts::ARCH);
// }

// fn print_hardware_info(system: &mut System) {
//     println!("{}", "Detailed Hardware Information".yellow());
//     println!("{}: {} GB", "Total Memory".blue(), system.total_memory() / 1024 / 1024);
//     println!("{}: {}", "Total CPU Cores".blue(), system.physical_core_count().unwrap_or_default());
//     let s = System::new_with_specifics(
//         RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
//     );
//     let mut i=0;
//     let mut is_first_cpu = true;
//     for cpu in s.cpus() {
//         if is_first_cpu {
//             println!("{}: {}", "CPU Model".blue(), cpu.brand());
//             is_first_cpu = false;
//         }
//         println!("{} {}: {} MHz", "CPU Speed".blue(), i, cpu.frequency());
//         println!("{} {}: {}%", "CPU Usage".blue(), i, cpu.cpu_usage());
//         i+=1;
//     }
// }

// fn print_network_info() {
//     println!("{}", "Network Information".yellow());
//     let networks = Networks::new_with_refreshed_list();
//     for (interface_name , network) in &networks {
//         println!("[{interface_name}]");
//         println!("MAC Address: {}", network.mac_address());
//         println!("Total Packets Received: {}", format_network_data(network.total_packets_received()));
//         println!("Total Packets Transmitted: {}", format_network_data(network.total_packets_transmitted()));
//         println!();
//     }
// }

// fn print_os_info() {
//     println!("{}", "Detailed OS Information".yellow());
//     println!("{}: {}", "OS Name".blue(), sysinfo::System::name().unwrap_or_default());
//     println!("{}: {}", "Kernel Version".blue(), sysinfo::System::kernel_version().unwrap_or_default());
//     println!("{}: {}", "System Uptime".blue(), format_uptime(sysinfo::System::uptime()));
// }
