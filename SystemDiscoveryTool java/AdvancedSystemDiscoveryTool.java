import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.InetAddress;
import java.net.NetworkInterface;
import java.net.SocketException;
import java.util.Enumeration;

public class AdvancedSystemDiscoveryTool {
    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));

        while (true) {
            System.out.println("Advanced System Discovery Tool");
            System.out.println("Choose an option:");
            System.out.println("1. Display System Information");
            System.out.println("2. Display Network Information");
            System.out.println("3. Exit");

            String choice = reader.readLine();

            switch (choice) {
                case "1":
                    displaySystemInfo();
                    break;
                case "2":
                    displayNetworkInfo();
                    break;
                case "3":
                    exitScript();
                    break;
                default:
                    System.out.println("Invalid choice. Please select a valid option.");
            }
        }
    }

    private static void displaySystemInfo() {
        System.out.println("System Information:");
        System.out.println("Operating System: " + System.getProperty("os.name"));
        System.out.println("Host Name: " + getHostName());
        System.out.println("Kernel Version: " + System.getProperty("os.version"));
        System.out.println("CPU: " + System.getProperty("os.arch"));
        System.out.println("Number of CPU Cores: " + Runtime.getRuntime().availableProcessors());
        System.out.println("Number of Processors: " + Runtime.getRuntime().availableProcessors());

        // ... (Rest of the script)
    }

    private static String getHostName() {
        try {
            return InetAddress.getLocalHost().getHostName();
        } catch (Exception e) {
            return "N/A";
        }
    }

    private static void displayNetworkInfo() throws SocketException {
        System.out.println("Network Information:");
        Enumeration<NetworkInterface> networkInterfaces = NetworkInterface.getNetworkInterfaces();
        while (networkInterfaces.hasMoreElements()) {
            NetworkInterface networkInterface = networkInterfaces.nextElement();
            System.out.println("Interface: " + networkInterface.getDisplayName());
            Enumeration<InetAddress> addresses = networkInterface.getInetAddresses();
            while (addresses.hasMoreElements()) {
                InetAddress address = addresses.nextElement();
                if (!address.isLoopbackAddress()) {
                    System.out.println("  IPv4 Address: " + address.getHostAddress());
                }
            }
        }
    }

    private static void exitScript() {
        System.out.println("\nThank you for using the Advanced System Discovery Tool!");
        System.exit(0);
    }
}
