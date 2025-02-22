mod system;
use system::os;
use system::dnf;

mod model;
use model::update::Update;

mod commons;

use std::collections::HashMap;

fn display_stabl_logo() {
    let logo:&str = r"
     _        _     _ 
 ___| |_ __ _| |__ | |
/ __| __/ _` | '_ \| |
\__ \ || (_| | |_) | |
|___/\__\__,_|_.__/|_|

A DNF wrapper to selectively choose what packages to upgrade

    ";
    println!("{}", logo);
}

fn main() {
    display_stabl_logo();
    let system_details:String = os::get_os_name();
    println!("[i] running on: {}\n", system_details);
    println!("[i] process started!");
    println!("[i] getting updates list from remote...");

    let available_updates: Vec<String> = dnf::get_available_updates();
    let mut updates_by_partition = HashMap::new();

    for line in available_updates {
        assert!(line != "");

        let value: Update = Update::from_dnf_output(line);
        let key: String = value.get_partition_id().clone();

        updates_by_partition.insert(key, value.clone()); // BUG: value deve essere un array
    }

    for (key, value) in map.into_iter() {
        println!("partition: \"{}\"", value.get_partition_id());
        println!("update type: \"{}\"", value.get_release_type());
        println!("security grade: \"{}\"", value.get_severity());
        println!("signature: \"{}\"\n", value.get_signature());
    }
}
