use battery::{Manager, units::ratio::percent};
use std::{io,thread, fs};
use std::time::Duration;
use dotenv;
use std::path::Path;
use system_shutdown::shutdown;

const BAT_MIN: f32 = 5.0;
const BAT_MAX: f32 = 50.0;
const BATTERY_MIN_THRESHOLD: &str = "BATTERY_MIN_THRESHOLD";

fn main() -> battery::Result<()> {

    let manager = Manager::new()?;

    let battery_threshold: f32 = init_threshold(); 

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

    loop{
            
        let b_percentage: f32 = battery.state_of_charge().get::<percent>();

        if b_percentage <= battery_threshold as f32 {
            match shutdown() {
                Ok(_) => println!("You Battery Is too low. I am Shutting down!"),
                Err(e) => println!("Error during power down: {}", e),
            }
        }

        println!("Battery Level is={}%", b_percentage);
        thread::sleep(Duration::from_secs(60));
        manager.refresh(&mut battery)?;
    }
}

fn init_threshold() -> f32 {

    match get_battery_threshold() {
        Ok(val) => {
            val
        }
        Err(_) => {
            loop {
                match set_environment_var() {
                    Ok(val) => {
                        break val;
                    }
                    Err(e) => println!("{:#?}", e)
                }
            }
        }
    }
}

fn set_environment_var() -> Result<f32, String> {


    println!("Please enter a your battery limit (Must be between 5 and 50 inclusive)");
    let mut input = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut input) {
        return Err(format!("Failed to read input: {}", e));
    }

    let input: f32 = input.trim().parse().map_err(|e| {
        format!("Invalid input. Please enter a valid integer. Error: {}", e)
    })?;

    if input > BAT_MAX || input  < BAT_MIN {
        return Err(String::from("Error: 'BATTERY_MIN_THESHOLD' must be greater than 4 but less than 50"));
    } 


    if !Path::new(".env").exists() {
       let _ =  fs::File::create_new(".env");
    }

    if let Err(e) = fs::write(".env", format!("{BATTERY_MIN_THRESHOLD}={}", input.to_string())) {
        return Err(e.to_string());
    }

    Ok(input)
}

fn get_battery_threshold() -> Result<f32, String>{

    dotenv::dotenv().ok();
    let battery_threshold = dotenv::var("BATTERY_MIN_THRESHOLD")
        .map_err(|_| String::from("Error: Environment variable 'BATTERY_MIN_THRESHOLD' is not set"))?;

    let battery_threshold: f32 = battery_threshold
        .parse()
        .map_err(|_| String::from("Error: 'BATTERY_MIN_THRESHOLD' is not a valid float."))?;
    if battery_threshold > BAT_MAX || battery_threshold < BAT_MIN {
        return Err(String::from("Error: 'BATTERY_MIN_THESHOLD' must be greater than 4 but less than 50"));
    } 
    Ok(battery_threshold)
}

