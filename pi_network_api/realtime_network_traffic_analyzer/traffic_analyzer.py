import pyshark

class TrafficAnalyzer:
    def __init__(self, interface):
        self.interface = interface

    def capture_traffic(self):
        capture = pyshark.LiveCapture(interface=self.interface)
        for packet in capture.sniff_continuously():
            print(f"Packet captured: {packet}")

if __name__ == "__main__":
    analyzer = TrafficAnalyzer("wlan0")
    analyzer.capture_traffic()
