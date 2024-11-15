# Battery Monitor

A Rust-based tool to monitor battery levels on Linux systems. Later versions will support user set threshold
And will prompt user once threshold is met with an option to bypass.

## Download & Installation

### 1. Download the `.deb` package
Download the latest release from https://github.com/HyflerSD/battery_monitor/releases

### 2. Install the package
`wget https://github.com/HyflerSD/battery_monitor/releases/download/v.0.0.3/battery-monitor_0.1.0-1_amd64.deb`

`sudo dpkg -i battery-monitor_0.1.0-1_amd64.deb`

To see where it was installed run:
`which battery_monitor`

If you need to:
`sudo apt --fix-broken install`

### 3. Turn executable into a service
Create service file names 'battery_monitor' inside /etc/systemd/system

`sudo vim /etc/systemd/system/battery_monitor.service`

Paste the content below inside the battery_monitor.service file:
(Be sure to update the path of ExecStart to where your exetuble was installed. Run "which battery_monitor" to find the path)

```
[Unit]
Description=Battery Monitor Service
After=network.target

[Service]
ExecStart=/path/to/your/binary/battery_monitor
Restart=on-failure

[Install]
WantedBy=multi-user.target
```
After saving enable and start the service

`sudo systemctl daemon-reload`
`sudo systemctl enable battery_monitor`
`sudo systemctl start battery_monitor`
