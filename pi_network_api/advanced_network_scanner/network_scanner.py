import scapy.all as scapy
import ipaddress

class NetworkScanner:
    def __init__(self, interface):
        self.interface = interface

    def scan_network(self, ip_range):
        ip_network = ipaddress.ip_network(ip_range, strict=False)
        live_hosts = []

        for host in ip_network.hosts():
            response = scapy.sr1(scapy.Ether(dst="ff:ff:ff:ff:ff:ff") / scapy.ARP(pdst=str(host)), 
                                 timeout=1, 
                                 iface=self.interface, 
                                 verbose=False)

            if response:
                live_hosts.append(host)

        return live_hosts

if __name__ == "__main__":
    scanner = NetworkScanner("wlan0")
    print(scanner.scan_network("192.168.1.0/24"))
