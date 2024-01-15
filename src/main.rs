#![windows_subsystem = "windows"]

use mslnk::ShellLink;
use std::{env::args, env::var_os, path::PathBuf};

//https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
pub fn get_appdata() -> Option<PathBuf> {
    var_os("APPDATA").map(PathBuf::from)
}

fn main() {
    let args: Vec<String> = args().collect();
    let target: &str;

    let app_data = get_appdata()
        .expect("no appdata")
        .into_os_string()
        .into_string()
        .unwrap();
    let location: &str;
    let file_name: &str;

    if args.len() == 1 {
        target = &args[0];
        location = r"SendTo\";
        file_name = "Start Menu"
    } else {
        target = &args[1];

        location = r"Start Menu\Programs\";

        let parts = target.split(r"\");
        let collection: Vec<&str> = parts.collect();
        let file = collection.last().unwrap();
        let parts_ext = file.split(".");
        let collection_ext: Vec<&str> = parts_ext.collect();
        file_name = collection_ext.first().unwrap();
    }

    //https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
    let lnk = format!(
        "{}{}{}{}{}",
        &app_data, r"\Microsoft\Windows\", location, file_name, ".lnk"
    );
    let sl = ShellLink::new(target).unwrap();
    sl.create_lnk(lnk).unwrap();
}
