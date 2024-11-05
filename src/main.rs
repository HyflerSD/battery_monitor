use battery::{Manager, units::ratio::percent};
use std::io;
use std::thread;
use std::time::Duration;
use dotenv;
use std::process;

fn main() -> battery::Result<()> {
    dotenv::dotenv().ok();
    let battery_threshold = dotenv::var("BATTERY_THRESHOLD")
        .and_then(|v| {
            v.parse::<f32>().map_err(|e| {
                eprintln!("Error converting BATTERY_THRESHOLD to f32: {:#?}", e);
                eprintln!("Please ensure the BATTERY_THRESHOLD env variable is a valid float.");
                process::exit(1);
            })
        })
        .unwrap_or_else(|e|{
            eprintln!("Error retrieving env BATTERY_THRESHOLD: {:#?}", e);
            process::exit(1);
        });

    let manager = Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    loop {
            
        let b_percentage: f32 = battery.state_of_charge().get::<percent>();
        if b_percentage >= battery_threshold as f32 {
            println!("Battery Level is={}%", b_percentage);
        }
        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}
