#[path = "windows_process/process.rs"]
mod process;

#[path = "windows_process/process_info.rs"]
mod process_info;

use serde_json::Result;
use serde_json::Value;

mod config;

fn parse_process_list(process_list: &Vec<Value>) {
    for process in process_list {
        let process_name_option = process["process_name"].as_str();
        let priority_str = process["process_priority"].as_str();

        let mut priority: u32 = 0;

        match priority_str {
            Some(priority_level) => priority = process::enumerate_priority_level(priority_level),
            None => println!("Could not find priority for current process"),
        };

        if priority == 0 {
            println!("Not priority set for current process");
            continue;
        } else {
            let mut process_name: &str = "";
            match process_name_option {
                Some(process_name_str) => process_name = process_name_str,
                None => println!("Invalid process name received"),
            };

            if process_name.is_empty() {
                println!("Process name is empty");
            } else {
                let process_id = process::set_cpu_priority_for_process(process_name, priority);

                if process_id != 0 {
                    println!(
                        "Process {} with id ({}) set to priority: {}",
                        process_name,
                        process_id,
                        priority_str.unwrap()
                    );
                } else {
                    println!(
                        "ERROR: Process {} failed to set priority: {}",
                        process_name,
                        priority_str.unwrap()
                    );
                }
            }
        }

        // convert priority_str to priority_const (u32)

        // call set_cpu_priority_for_process
    }
}

fn main() -> Result<()> {
    const CONFIG_FILENAME: &str = "./process_config.json";

    let mut configuration = config::Config {
        config: "".to_string(),
        config_json: unsafe { std::mem::zeroed() },
    };
    configuration.load_config(CONFIG_FILENAME);
    configuration
        .parse_json_from_config()
        .expect("Unexpected error has occurred while parsing JSON");
    let process_list_option = configuration.get_list_of_processes();

    match process_list_option {
        Some(process_list) => {
            parse_process_list(process_list);
        }
        None => println!("No processes returned from configuration reading"),
    };

    Ok(())
}
