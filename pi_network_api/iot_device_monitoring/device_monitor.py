import influxdb
import requests

class DeviceMonitor:
    def __init__(self, influx_host, influx_port, grafana_host, grafana_port):
        self.influx_client = influxdb.InfluxDBClient(host=influx_host, port=influx_port)
        self.grafana_host = grafana_host
        self.grafana_port = grafana_port

    def send_metrics(self, device_data):
        json_body = [
            {
                "measurement": "device_metrics",
                "tags": {
                    "device_id": device_data["device_id"]
                },
                "fields": {
                    "temperature": device_data["temperature"],
                    "humidity": device_data["humidity"]
                }
            }
        ]
        self.influx_client.write_points(json_body)

    def create_grafana_dashboard(self):
        dashboard_json = {
            "title": "Device Monitoring",
            "rows": [
                {
                    "title": "Temperature",
                    "panels": [
                        {
                            "title": "Temperature Chart",
                            "type": "graph",
                            "targets": [
                                {
                                    "target": "SELECT temperature FROM device_metrics"
                                }
                            ]
                        }
                    ]
                }
            ]
        }
        response = requests.post(f"http://{self.grafana_host}:{self.grafana_port}/api/dashboards/db", json=dashboard_json)
        if response.status_code == 200:
            print("Grafana dashboard created successfully!")

if __name__ == "__main__":
    monitor = DeviceMonitor("localhost", 8086, "localhost", 3000)
    device_data = {"device_id": "device-1", "temperature": 25.5, "humidity": 60}
    monitor.send_metrics(device_data)
    monitor.create_grafana_dashboard()
