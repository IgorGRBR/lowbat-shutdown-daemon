#[macro_use]
extern crate serde_derive;
use std::fs;
mod config;
use config::Config;
use std::thread::sleep;
use std::time::Duration as Dur;
use std::str;
use std::process::Command;
mod battery_info;
use battery_info::BatteryInfo;

const CONF_PATH: &str = "./batmon-config.json";

fn get_battery_info() -> BatteryInfo {
    let output = Command::new("termux-battery-status")
        .output()
        .expect("Couldn't execute battery status command.");
    let battery_info: BatteryInfo = serde_json::from_str(str::from_utf8(&output.stdout).unwrap())
        .expect("Unable to parse command output");
    battery_info
}

fn send_notification(title: &str, message: &str) -> i32 {
    println!("Notification {}: {}", title, message);
    let output = Command::new("termux-notification")
            .arg("-t")
            .arg(title)
            .arg("-c")
            .arg(message)
            .status()
            .expect("Unable to display notification");
    output.code().unwrap()
}

fn shutdown() {
    let output = Command::new("/system/bin/reboot")
        .arg("-p")
        .status()
        .expect("Unable to shutdown the device");
    println!("Shutdown command status: {}", output.code().unwrap());
}

fn main() {
    println!("Running BatMon");
    let config_data: String = fs::read_to_string(CONF_PATH).expect("Unable to read config.json");
    let config: Config = serde_json::from_str(config_data.as_str()).expect("Unable to parse config.json");
    send_notification("BatMon", "BatMon is running");
    loop {
        sleep(Dur::new(config.polling_rate, 0));
        let battery_info = get_battery_info();
        println!("Battery level: {}", battery_info.percentage);
        if battery_info.percentage < config.charge_threshold {
            send_notification("BatMon", "Battery level dropped bellow the threshold. The device will shutdown soon.");
            sleep(Dur::new(config.warning_period, 0));
            let battery_info = get_battery_info();
            if battery_info.status != "CHARGING" {
                shutdown();
                break;
            }
        }
    }
}
