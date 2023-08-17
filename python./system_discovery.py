import os
import sys
import platform
import subprocess
import psutil
import netifaces
from usb.core import find as find_usb_devices
from datetime import datetime

def get_user_input():
    print("Choose an option:")
    print("1. Display System Information")
    print("2. Display Network Information")
    print("3. Display USB Devices")
    print("4. Exit")
    choice = input()
    return choice

def display_system_info():
    print("\x1b[1mSystem Information:\x1b[0m")
    print("Operating System:", platform.system())
    print("Host Name:", platform.node())
    print("Kernel Version:", platform.release())
    print("CPU:", platform.processor())
    print("Number of CPU Cores:", psutil.cpu_count(logical=False))
    print("Number of Processors:", psutil.cpu_count(logical=True))

    # ... (Rest of the script)

def display_network_info():
    print("\x1b[1mNetwork Information:\x1b[0m")
    interfaces = netifaces.interfaces()
    for interface in interfaces:
        print("Interface:", "\x1b[33m" + interface + "\x1b[0m")
        try:
            addrs = netifaces.ifaddresses(interface)
            ipv4_addr = addrs[netifaces.AF_INET][0]['addr']
            mac_addr = addrs[netifaces.AF_LINK][0]['addr']
            print("  IPv4 Address:", "\x1b[32m" + ipv4_addr + "\x1b[0m")
            print("  MAC Address:", "\x1b[32m" + mac_addr + "\x1b[0m")
        except KeyError:
            pass

def display_usb_devices():
    print("\x1b[1mConnected USB Devices:\x1b[0m")
    usb_devices = find_usb_devices(find_all=True)
    for device in usb_devices:
        print("Device:", "\x1b[33m" + device.product + "\x1b[0m")
        print("  Vendor ID:", "\x1b[35m" + hex(device.idVendor) + "\x1b[0m")
        print("  Product ID:", "\x1b[35m" + hex(device.idProduct) + "\x1b[0m")

def exit_script():
    print("\nThank you for using the Ultra Advanced System Discovery Tool!")
    sys.exit(0)

def main():
    while True:
        print("\x1b[1;34mUltra Advanced System Discovery Tool\x1b[0m")

        choice = get_user_input()

        if choice == "1":
            display_system_info()
        elif choice == "2":
            display_network_info()
        elif choice == "3":
            display_usb_devices()
        elif choice == "4":
            exit_script()
        else:
            print("Invalid choice. Please select a valid option.")

if __name__ == "__main__":
    main()
