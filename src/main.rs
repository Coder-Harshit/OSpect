// use colored::*;
// use clap::Command;
// use sysinfo::{CpuRefreshKind, RefreshKind, System ,Networks};
// use whoami;
// use std::{env, fs};
// use std::path::PathBuf;

use std::{env, error::Error, fs, path::PathBuf, process::{exit, Command}};
use colored::Colorize;
// use serde::Deserialize;
// use toml::Value;


// #[derive(Deserialize)]
// struct Config{
//     basic: BasicConfig,
// }
// #[derive(Deserialize)]
// struct BasicConfig{
//     hostname: Vec<String>,
//     username: Vec<String>,
//     osname: Vec<String>,
//     kernel: Vec<String>,
//     total_memory: Vec<String>,
// }


fn load_config(file_path: PathBuf, flag_type: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let config_content = fs::read_to_string(file_path)?;

    let mut options = Vec::new();
    let mut in_section = false;
    for line in config_content.lines(){
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("#"){continue;}
        if trimmed_line.starts_with("["){
            if trimmed_line == format!("[{}]",flag_type){
                // within the required section
                in_section=true;
            }
            else{
                // reached other section title
                in_section=false;
                continue;
            }
        }else if in_section{
            // if within the section then add options
            options.push(trimmed_line.to_string());
        }
    }
    Ok(options)
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
    let output= if cfg!(target_os="windows"){
        Command::new("/bin/sh")
        .args(["-c", "/bin/head -n 1 /etc/os-release | awk -F '=' '{print $2}'"])
        .output()?
    }else{
        Command::new("/bin/sh")
        .args(["-c", "/bin/head -n 1 /etc/os-release | awk -F '=' '{print $2}'"])
        .output()?
    };
    let osname = String::from_utf8(output.stdout)?;
    let trimmed_osname = osname.trim().trim_matches('"');
    return Ok(trimmed_osname.to_string());
}

fn fetch_kernel() -> Result<String, Box<dyn std::error::Error>> {
    let output = if cfg!(target_os="windows"){
        Command::new("powershell")
        .arg("-Command")
        .arg("Get-ItemProperty -Path 'HKLM:\\Software\\Microsoft\\Windows NT\\CurrentVersion' | Select-Object -ExpandProperty CurrentVersion")
        .output()?
    }else{
        Command::new("uname")
            .arg("-r")
            .output()?
    };
    let kernel = String::from_utf8(output.stdout)?;
    let kernel = kernel.trim();
    return  Ok(kernel.to_string());
}

fn fetch_total_mem() -> Result<u64, Box<dyn std::error::Error>>{
    let tmem = if cfg!(target_os="windows"){
        Command::new("/bin/sh")
        .args(["-c","/bin/cat /proc/meminfo | /bin/grep MemTotal | awk '{print $2}'"])
        .output()?
    } else{
        Command::new("/bin/sh")
        .args(["-c","/bin/cat /proc/meminfo | /bin/grep MemTotal | awk '{print $2}'"])
        .output()?
    };
    let stdout = String::from_utf8(tmem.stdout)?;
    let tmem: u64 = stdout.trim().parse()?;

    return Ok(tmem);
}

fn fetch_pvt_ip() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("/bin/sh")
        .args(["-c","ip -4 addr show | awk '/inet / && !/127.0.0.1/ {split($2, a, \"/\"); print a[1]}'"])
        .output()?;
    let pvt_ip = String::from_utf8(output.stdout)?;
    let pvt_ip = pvt_ip.trim();
    return Ok(pvt_ip.to_string());
}

fn fetch_pub_ip() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("/bin/curl")
        .arg("https://api.ipify.org/")
        .output()?;

    if output.status.success() {
        let pub_ip = String::from_utf8(output.stdout)?;
        let pub_ip = pub_ip.trim();
        Ok(pub_ip.to_string())
    } else {
        Err("Not connected to the internet or unable to fetch public IP.".into())
    }
}

fn fetch_mac_addr() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("/bin/sh")
        .args(["-c","ip link show | awk '/ether/ {print $2}'"])
        .output()?;
    let mac_addr = String::from_utf8(output.stdout)?;
    let mac_addr = mac_addr.trim();
    return Ok(mac_addr.to_string());
}


/*
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
*/

fn fetching_value(options:Vec<String>){
    for i in options{
        match i.as_str(){
            "username" => {
                match fetch_username() {
                    Ok(username) => {
                        println!("{}: {}", "Username".blue(), username);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
            },
            "hostname" => {
                match fetch_hostname() {
                    Ok(hostname) => {
                        println!("{}: {}", "Hostname".blue(), hostname);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
            },
            "osname" => {
                match fetch_osname() {
                    Ok(osname) => {
                        println!("{}: {}", "OS".blue(), osname);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
            },
            "kernel" => {
                match fetch_kernel() {
                    Ok(kernel) => {
                        println!("{}: {}", "Kernel".blue(), kernel);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
            },
            "total_memory" => {
                match fetch_total_mem() {
                    Ok(total_mem) => {
                        let total_mem = total_mem / (1024 * 1024);
                        println!("{}: {} GB", "Total Memory".blue(), total_mem);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
            },
            "ip" => {
                match fetch_pvt_ip() {
                    Ok(pvt_ip) => {
                        println!("{}: {}", "Private IP Address".blue(), pvt_ip);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
                match fetch_pub_ip() {
                    Ok(pub_ip) => {
                        println!("{}: {}", "Public IP Address".blue(), pub_ip);
                    }
                    Err(err) => println!("{}: {}", "Error".red(), err),
                }
            },
            "mac" => {
                match fetch_mac_addr() {
                    Ok(mac_addr) => {
                        println!("{}: {}", "MAC Address".blue(), mac_addr);
                    }
                    Err(err) => println!("{}: {}", "Error   ".red(), err),
                }

            },
            "interfaces" => {},
            "active_connections" => {},
            "cpu" => {},
            "gpu" => {},
            "disk" => {},
            _ => {
                println!("ERROR!!!");
                exit(1);
            }
        }
    }
}

fn main() {

    let home_dir = dirs::home_dir().expect("Failed to locate HOME dir");
    let config_path = if cfg!(target_os = "windows"){
        home_dir.join("Appdata\\Roaming\\ospect\\config.toml")
    }else{
        home_dir.join(".config/ospect/config.toml")
    };
    let args:Vec<String> = env::args().collect();

    // for itr in &args {
    //     println!("==> {itr}");
    // }

    match args.len() {
        1 => {
            // default option to run
            let options = load_config(config_path,"basic").expect("Failed to load config");
            // print_basic_info(options);
            // println!("{:?}",options);

            println!("{}", "Basic System Analytics".yellow());
            fetching_value(options);

        },
        2 => {
            let options = load_config(config_path,&args[1]).expect("Failed to load config");
            // specialized output
            match args[1].to_ascii_lowercase().as_str(){
                "network"=> {
                    println!("{}", "Network Analytics".yellow());
                },
                "hardware" => {
                    println!("{}", "Hardware Analytics".yellow());
                },
                "os" => {
                    println!("{}", "System Analytics".yellow());
                },
                _ =>{
                    println!("Invalid Option Passed!!\nBelow are the list of available options/flags:");
                    println!("network");
                    println!("hardware");
                    println!("os");
                }
            }
            fetching_value(options);

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
