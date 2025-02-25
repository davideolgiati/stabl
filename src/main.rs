mod system;
use system::os;
use system::dnf;

mod model;
use model::update::Update;
use model::partitions::builder::PartitionBuilder;

mod commons;
use commons::string::split_string_using_delimiter;

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

fn extract_signature_list(available_updates: Vec<String>) -> Vec<String> {
    return available_updates
        .into_iter()
        .map(|line| split_string_using_delimiter(line, " "))
        .map(|tokens| tokens[3].clone())
        .collect();
}

fn extract_version_and_release_map(details_from_repository: Vec<String>) -> HashMap<String, (String, String, String)> {
    let mut version_and_release_map = HashMap::new();
    let details: Vec<Vec<String>> = details_from_repository
        .clone()
        .into_iter()
        .map(|line| split_string_using_delimiter(line, "|#|"))
        .collect();

    for detail in details {
        version_and_release_map.insert(
            detail[4].clone(), 
            (detail[0].clone(), detail[1].clone(), detail[2].clone())
        );

        version_and_release_map.insert(
            detail[5].clone(), 
            (detail[0].clone(), detail[1].clone(), detail[2].clone())
        );
    }

    return version_and_release_map
}

fn main() {
    display_stabl_logo();
    let system_details:String = os::get_os_name();
    println!("[i] running on: {}\n", system_details);
    println!("[i] process started!");
    println!("[i] getting updates list from remote...");

    let available_updates: Vec<String> = dnf::get_available_updates();
    
    println!("[i] gruoping updates in partititons...");
    
    let mut partition_builder: PartitionBuilder = PartitionBuilder::new();
    let signatures: Vec<String> = extract_signature_list(available_updates.clone());
    let remote_details: Vec<String> = dnf::get_updates_details(signatures);
    
    let updates: Vec<Update> = available_updates
                                .into_iter()
                                .map(|line| Update::from_dnf_output(line))
                                .collect();

    let processed_details: HashMap<String, (String, String, String)> = extract_version_and_release_map(remote_details.clone());

    for update in updates {
        partition_builder.register_update(update.clone());
    }
    
    let partitions = partition_builder.build();
    
    for (partition_id, partition) in &partitions {
        println!(
            "\npartition: \"{}\" (type: {}, security grade: {})", 
            partition_id, partition.get_release_type(), partition.get_severity()
        );
        for _update in partition.get_updates().into_iter() {
            println!("\t\"{}\" {}-{}", _update.get_signature(), _update.get_version(), _update.get_release());
        }
    }
}
