mod system;
use system::os;
use system::dnf;

mod model;
use model::update::Update;
use model::partitions::builder::PartitionBuilder;


mod commons;

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
    let mut partition_builder = PartitionBuilder::new();

    for line in available_updates {
        assert!(line != "");

        let value: Update = Update::from_dnf_output(line);

        partition_builder.add_update(value.clone());
    }

    let partitions = partition_builder.build();

    for (partition, updates) in &partitions {
        println!("partition: \"{}\" \n", partition);
        for _update in updates.into_iter() {
            println!("\tsignature: \"{}\"", _update.get_signature());
            println!("\tupdate type: \"{}\"", _update.get_release_type());
            println!("\tsecurity grade: \"{}\"\n", _update.get_severity());
        }
    }
}
