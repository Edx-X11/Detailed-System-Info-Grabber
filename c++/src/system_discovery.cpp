#include <iostream>
#include <cstdlib>
#include <string>
#include <sysinfo.h>
#include <ifaddrs.h>
#include <ifaces/Interface.hpp>
#include <chrono>

int get_user_input() {
    int choice;
    std::cout << "Choose an option:" << std::endl;
    std::cout << "1. Display System Information" << std::endl;
    std::cout << "2. Display Network Information" << std::endl;
    std::cout << "3. Display USB Devices" << std::endl;
    std::cout << "4. Exit" << std::endl;
    std::cin >> choice;
    return choice;
}

void display_system_info(sys_info::System &system) {
    try {
        system.refreshAll();
    } catch (const std::exception &err) {
        std::cerr << "Error refreshing system information: " << err.what() << std::endl;
        return;
    }

    std::cout << "\x1b[1mSystem Information:\x1b[0m" << std::endl;

    // Display OS, hostname, kernel version, CPU info, etc.

    // ... (Rest of the script)
}

void display_network_info(sys_info::System &system) {
    try {
        system.refreshNetworks();
    } catch (const std::exception &err) {
        std::cerr << "Error refreshing network information: " << err.what() << std::endl;
        return;
    }

    std::cout << "\x1b[1mNetwork Information:\x1b[0m" << std::endl;

    // Display network interfaces, IP addresses, MAC addresses, etc.

    // ... (Rest of the script)
}

void display_usb_devices() {
    auto usb_context = sys_info::UsbContext::newContext();
    if (!usb_context) {
        std::cerr << "Error initializing USB context" << std::endl;
        return;
    }

    auto devices = usb_context->devices();
    if (!devices) {
        std::cerr << "Error fetching USB devices" << std::endl;
        return;
    }

    std::cout << "\x1b[1mConnected USB Devices:\x1b[0m" << std::endl;

    // Display connected USB devices with vendor and product info

    // ... (Rest of the script)
}

void exit_script() {
    std::cout << "\nThank you for using E_DX's Ultra Advanced System Discovery Tool!" << std::endl;
    std::exit(0);
}

int main() {
    sys_info::System system;

    while (true) {
        std::cout << "\x1b[1;34mE_DX's Ultra Advanced System Discovery Tool\x1b[0m" << std::endl;

        int choice = get_user_input();

        switch (choice) {
            case 1:
                display_system_info(system);
                break;
            case 2:
                display_network_info(system);
                break;
            case 3:
                display_usb_devices();
                break;
            case 4:
                exit_script();
                break;
            default:
                std::cout << "Invalid choice. Please select a valid option." << std::endl;
        }
    }

    return 0;
}