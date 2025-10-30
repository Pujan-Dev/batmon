use notify_rust::Notification;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let base = Path::new("/sys/class/power_supply");

    let mut total_percent = 0.0;
    let mut count = 0;

    for entry in fs::read_dir(base).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let type_file = path.join("type");
        if type_file.exists() {
            let typ = fs::read_to_string(type_file).unwrap_or_default();
            if typ.trim() != "Battery" {
                continue;
            }
        } else {
            continue;
        }

        let capacity_file = path.join("capacity");
        if capacity_file.exists() {
            let cap_str = fs::read_to_string(capacity_file).unwrap_or_default();
            if let Ok(cap) = cap_str.trim().parse::<f64>() {
                total_percent += cap;
                count += 1;
            }
        }
    }

    if count == 0 {
        println!("No batteries found.");
        return;
    }

    let avg = total_percent / count as f64;
    println!("Total battery: {:.1}%", avg);

    if avg >= 100.0 {
        // Notification
        let _ = Notification::new()
            .summary("Battery Full")
            .body("Your battery is fully charged ⚡")
            .show();

        // Sound (play default system sound)
        let _ = Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
            .output();
    } else if avg <= 10.0 {
        // Notification
        let _ = Notification::new()
            .summary("Battery Low")
            .body(&format!("Battery is low ({:.0}%) 🔋", avg))
            .show();

        // Sound (play warning sound)
        let _ = Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga")
            .output();
    }
}
