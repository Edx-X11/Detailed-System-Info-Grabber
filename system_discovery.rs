extern crate sysinfo;
extern crate ifaces;
extern crate chrono;

use sysinfo::{NetworkExt, ProcessorExt, System, SystemExt, UsbContext};
use ifaces::Interface;
use chrono::prelude::*;

fn main() {
    let mut system = System::new_all();

    loop {
        println!("\x1b[1;34mE_DX's Ultra Advanced System Discovery Tool\x1b[0m");

        println!("Choose an option:");
        println!("1. Display System Information");
        println!("2. Display Network Information");
        println!("3. Display USB Devices");
        println!("4. Exit");

        let choice = get_user_input();

        match choice {
            1 => display_system_info(&mut system),
            2 => display_network_info(&mut system),
            3 => display_usb_devices(),
            4 => exit_script(),
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

fn get_user_input() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().unwrap_or(0)
}

fn display_system_info(system: &mut System) {
    if let Err(err) = system.refresh_all() {
        eprintln!("Error refreshing system information: {}", err);
        return;
    }

    println!("\x1b[1mSystem Information:\x1b[0m");

    println!("Operating System: {}", sys_info::os_type().unwrap_or("N/A".to_string()));

    if let Some(host_name) = sys_info::host_name() {
        println!("Host Name: {}", host_name);
    }

    if let Some(kernel_version) = sys_info::kernel_version() {
        println!("Kernel Version: {}", kernel_version);
    }

    if let Some(cpu_brand) = sys_info::cpu_brand() {
        println!("CPU: {}", cpu_brand);
    }

    if let Some(cpu_cores) = sys_info::cpu_num() {
        println!("Number of CPU Cores: {}", cpu_cores);
    }

    println!("Number of Processors: {}", system.get_processors().len());

    // ... (Rest of the script)
}

fn display_network_info(system: &mut System) {
    if let Err(err) = system.refresh_networks() {
        eprintln!("Error refreshing network information: {}", err);
        return;
    }

    println!("\x1b[1mNetwork Information:\x1b[0m");

    if let Some(networks) = system.get_networks() {
        for (interface_name, network) in networks {
            println!("Interface: \x1b[33m{}\x1b[0m", interface_name);
            println!("  Received: {} B", network.get_received());
            println!("  Sent: {} B", network.get_transmitted());

            if let Ok(interface) = Interface::get_by_name(interface_name) {
                if let Some(ip) = interface.get_ipv4_addr() {
                    println!("  \x1b[32mIPv4 Address: {}\x1b[0m", ip);
                }
                if let Some(mac) = interface.get_mac_address() {
                    println!("  \x1b[32mMAC Address: {}\x1b[0m", mac);
                }
            }
        }
    }
}

fn display_usb_devices() {
    let usb_context = UsbContext::new();
    if let Ok(devices) = usb_context.devices() {
        println!("\x1b[1mConnected USB Devices:\x1b[0m");
        for device in devices {
            println!("Device: \x1b[33m{}\x1b[0m", device.product());
            println!("  Vendor ID: \x1b[35m{:04x}\x1b[0m", device.vendor_id());
            println!("  Product ID: \x1b[35m{:04x}\x1b[0m", device.product_id());
        }
    }
}

fn exit_script() {
    println!("\nThank you for using E_DX's Ultra Advanced System Discovery Tool!");
    std::process::exit(0);
}
