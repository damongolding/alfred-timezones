#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use serde::Deserialize;
use serde_json::json;

use regex::Regex;
use std::env;
use std::fs::File;
use std::path::Path;

use chrono::{Datelike, TimeZone, Utc};
use chrono_tz::America::Cancun;
use chrono_tz::Europe::Copenhagen;
use chrono_tz::Europe::London;

use chrono_tz::US::Eastern;
use chrono_tz::US::Mountain;
use chrono_tz::US::Pacific;

#[derive(Serialize, Deserialize, Debug)]
struct AlfredItem {
    title: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AlfredItemWithSubtitle {
    title: String,
    subtitle: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AlfredItems {
    items: Vec<AlfredItem>,
}

/**
 * Check if the given string is a valid 12 hour date
 * e.g. "10:00am GMT"
 */
fn is_12_hour_format(time_str: &str) -> bool {
    let re = Regex::new(r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+(am|pm)+\s([a-zA-Z]+)").unwrap();
    return re.is_match(time_str);
}

/**
 * Check if the given string is a valid 24 hour date
 * e.g. "12:00 GMT"
 */
fn is_24_hour_format(time_str: &str) -> bool {
    let re = Regex::new(r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+\s([a-zA-Z]+)").unwrap();
    return re.is_match(time_str);
}

/**
 * Returns chrono time object and bool for the given timezone as a tuple
 * @returns (bool, chrono time object)
 */
fn get_time_object_from_timezone(timezone: &str) -> (bool, &chrono_tz::Tz) {
    return match timezone.to_uppercase().as_str() {
        "ACDT" => (true, &chrono_tz::Australia::Adelaide),
        "ACST" => (true, &chrono_tz::Australia::Darwin),
        "AEDT" => (true, &chrono_tz::Australia::Sydney),
        "AEST" => (true, &chrono_tz::Australia::Queensland),
        "AKST" => (true, &chrono_tz::US::Alaska),
        "AST" => (true, &chrono_tz::America::Halifax),
        "AWST" => (true, &chrono_tz::Australia::Perth),
        "CAT" => (true, &chrono_tz::Africa::Gaborone),
        "CET" => (true, &chrono_tz::Europe::Berlin),
        "CST" => (true, &chrono_tz::US::Central),
        "EAT" => (true, &chrono_tz::Africa::Asmara),
        "EET" => (true, &chrono_tz::Europe::Helsinki),
        "EST" => (true, &chrono_tz::US::Eastern),
        "GMT" => (true, &chrono_tz::Europe::London),
        "HKT" => (true, &chrono_tz::Asia::Hong_Kong),
        "HST" => (true, &chrono_tz::US::Hawaii),
        "IST" => (true, &chrono_tz::Asia::Kolkata),
        "JST" => (true, &chrono_tz::Asia::Tokyo),
        "KST" => (true, &chrono_tz::Asia::Seoul),
        "MET" => (true, &chrono_tz::Europe::Amsterdam),
        "MSK" => (true, &chrono_tz::Europe::Moscow),
        "MST" => (true, &chrono_tz::US::Mountain),
        "NST" => (true, &chrono_tz::Canada::Newfoundland),
        "NZDT" => (true, &chrono_tz::Antarctica::McMurdo),
        "PKT" => (true, &chrono_tz::Asia::Karachi),
        "PST" => (true, &chrono_tz::US::Pacific),
        "SAST" => (true, &chrono_tz::Africa::Johannesburg),
        "SST" => (true, &chrono_tz::US::Samoa),
        "UTC" => (true, &chrono_tz::UTC),
        "WAT" => (true, &chrono_tz::Africa::Luanda),
        "WET" => (true, &chrono_tz::Europe::Lisbon),
        "WIB" => (true, &chrono_tz::Asia::Jakarta),
        "WIT" => (true, &chrono_tz::Asia::Jayapura),
        "WITA" => (true, &chrono_tz::Asia::Makassar),
        _ => (false, &chrono_tz::UTC),
    };
}

/**
 * Prints all the available timezones
 */
fn list_all_timezones(all_timezones: [&str; 34]) {
    let mut output = AlfredItems { items: Vec::new() };

    for timezone in all_timezones.iter() {
        let item = AlfredItem {
            title: timezone.to_string(),
        };
        output.items.push(item);
    }

    let payload = serde_json::to_string(&output).unwrap();

    println!("{}", payload);
}

/**
 * Prints submitted time in the timezones specified in preferences.json using in 12 hour format
 */
fn display_12_hour_times(time_arg: &str, user_prefs: Vec<String>) {
    let current_time = Utc::now();
    let re = Regex::new(r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+(am|pm)+\s([a-zA-Z]+)").unwrap();
    let regex_groups = re.captures(time_arg).unwrap();

    let hour = regex_groups.get(1).map_or("", |m| m.as_str());
    let minute = regex_groups.get(2).map_or("", |m| m.as_str());
    let _meridian = regex_groups.get(3).map_or("", |m| m.as_str());
    let submitted_timezone = regex_groups.get(4).map_or("", |m| m.as_str());

    let timezone_to_use = get_time_object_from_timezone(submitted_timezone);

    if timezone_to_use.0 == false {
        println!(
            "{}",
            json!({
                "items": [
                    {
                        "title": "Invalid timezone",
                        "subtitle": format!("{} is not a valid timezone", submitted_timezone),
                    }
                ]
            })
        );
        return;
    }

    let time = timezone_to_use
        .1
        .ymd(
            current_time.year(),
            current_time.month(),
            current_time.day(),
        )
        .and_hms(
            hour.parse::<u32>().unwrap(),
            minute.parse::<u32>().unwrap(),
            0,
        );
    let mut output = AlfredItems { items: Vec::new() };
    for pref in user_prefs {
        let time_to_add = time
            .with_timezone(get_time_object_from_timezone(&pref).1)
            .format("%l:%M%P %Z")
            .to_string();

        let alfred_item = AlfredItem {
            title: time_to_add.trim().to_string(),
        };
        output.items.push(alfred_item);
    }

    println!("{}", serde_json::to_string(&output).unwrap());
}

/**
 * Prints submitted time in the timezones specified in preferences.json using in 24 hour format
 */
fn display_24_hour_times(time_arg: &str, user_prefs: Vec<String>) {
    let current_time = Utc::now();
    let re = Regex::new(r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+\s([a-zA-Z]+)").unwrap();
    let regex_groups = re.captures(time_arg).unwrap();

    let hour = regex_groups.get(1).map_or("", |m| m.as_str());
    let minute = regex_groups.get(2).map_or("", |m| m.as_str());
    let submitted_timezone = regex_groups.get(3).map_or("", |m| m.as_str());

    let timezone_to_use = get_time_object_from_timezone(submitted_timezone);

    if timezone_to_use.0 == false {
        println!(
            "{}",
            json!({
                "items": [
                    {
                        "title": "Invalid timezone",
                        "subtitle": format!("{} is not a valid timezone", submitted_timezone),
                    }
                ]
            })
        );
        return;
    }

    let time = timezone_to_use
        .1
        .ymd(
            current_time.year(),
            current_time.month(),
            current_time.day(),
        )
        .and_hms(
            hour.parse::<u32>().unwrap(),
            minute.parse::<u32>().unwrap(),
            0,
        );
    let mut output = AlfredItems { items: Vec::new() };
    for pref in user_prefs {
        let time_to_add = time
            .with_timezone(get_time_object_from_timezone(&pref).1)
            .format("%H:%M %Z")
            .to_string();

        let alfred_item = AlfredItem {
            title: time_to_add.trim().to_string(),
        };
        output.items.push(alfred_item);
    }

    println!("{}", serde_json::to_string(&output).unwrap());
}

/**
 * Main function
 */
fn main() {
    let all_timezones: [&str; 34] = [
        "ACDT", "ACST", "AEDT", "AEST", "AKST", "AST", "AWST", "CAT", "CET", "CST", "EAT", "EET",
        "EST", "GMT", "HKT", "HST", "IST", "JST", "KST", "MET", "MSK", "MST", "NST", "NZDT", "PKT",
        "PST", "SAST", "SST", "UTC", "WAT", "WET", "WIB", "WIT", "WITA",
    ];

    // If time arg is missing or empty, exit
    if env::args().len() < 2 {
        println!(
            "{}",
            json!({
                "items": [
                    {
                        "title": "Error: Missing or empty time argument",
                        "subtitle": "example: 10:34am gmt or 18:30 pst",
                    }
                ]
            })
        );
        std::process::exit(1);
    }
    // if arg is "list" or "ls", print all timezones
    else if env::args().nth(1).unwrap() == "list" || env::args().nth(1).unwrap() == "ls" {
        list_all_timezones(all_timezones);
        std::process::exit(1);
    }

    // Get the submitted time argument
    let time_arg = env::args().nth(1).unwrap();

    // Are we running in alfred
    // let running_in_alfred = match env::var("alfred_version") {
    //     Ok(_) => true,
    //     Err(_) => false,
    // };

    if !is_12_hour_format(&time_arg) && !is_24_hour_format(&time_arg) {
        println!(
            "{}",
            json!({
                "items": [
                    {
                        "title": "Invalid time format",
                        "subtitle": "example: 10:34am gmt or 18:30 pst",
                    }
                ]
            })
        );
        std::process::exit(1);
    }

    // load in the user's preferences from json file
    let user_preferences_file_path = Path::new("preferences.json");
    let user_preferences_file_contents =
        File::open(user_preferences_file_path).expect("file not found");
    let user_preferences: Vec<String> =
        serde_json::from_reader(user_preferences_file_contents).expect("error while reading");

    // Show times depending on the format of the submitted time
    match is_12_hour_format(&time_arg) {
        true => display_12_hour_times(&time_arg, user_preferences),
        false => display_24_hour_times(&time_arg, user_preferences),
    };

    std::process::exit(0);
}
