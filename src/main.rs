#[path = "windows_process/process.rs"]
mod process;

use serde_json::Result;
use serde_json::Value;

mod config;

fn parse_priority_level(priority_level_option: Option<&str>) -> u32 {
    let mut priority: u32 = 0;

    match priority_level_option {
        Some(priority_level) => priority = process::enumerate_priority_level(priority_level),
        None => println!("Could not find priority for current process"),
    };

    priority
}

fn parse_process_name(process_name_option: Option<&str>) -> std::string::String {
    let mut process_name: &str = "";
    match process_name_option {
        Some(process_name_str) => process_name = process_name_str,
        None => println!("Invalid process name received"),
    };

    if process_name.is_empty() {
        println!("Process name is empty");
    }

    process_name.to_string()
}

fn set_priority(process_name: &str, priority: u32) {
    let process_id = process::set_cpu_priority_for_process(process_name, priority);

    if process_id != 0 {
        println!(
            "Process {} with id ({}) set to priority: {}",
            process_name,
            process_id,
            priority //_str.unwrap()
        );
    } else {
        println!(
            "ERROR: Process {} failed to set priority: {}",
            process_name,
            priority //_str.unwrap()
        );
    }
}

fn set_priority_for_each_process_in_list(process_list: &Vec<Value>) {
    for current_process in process_list {
        let process_name_option = current_process["process_name"].as_str();
        let priority_str = current_process["process_priority"].as_str();

        let priority: u32 = parse_priority_level(priority_str);

        if priority == 0 {
            println!("No priority set for current process");
            continue;
        } else {
            let process_name = parse_process_name(process_name_option);
            if !process_name.is_empty() {
                set_priority(&process_name, priority);
            }
        }
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
            set_priority_for_each_process_in_list(process_list);
        }
        None => println!("No processes returned from configuration"),
    };

    Ok(())
}
