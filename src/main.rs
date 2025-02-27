mod system;
use model::enums::release_type::ReleaseType;
use model::enums::severity::Severity;
use model::updates::builder::UpdateBuilder;
use system::os;
use system::dnf;

mod model;
use model::updates::update::Update;
use model::partitions::builder::PartitionBuilder;

mod commons;
use commons::string::split_string_using_delimiter;

use std::collections::HashMap;
use std::collections::HashSet;

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

fn extract_version_and_release_map(details_from_repository: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut version_and_release_map = HashMap::new();
    let details: Vec<Vec<String>> = details_from_repository
        .clone()
        .into_iter()
        .map(|line| split_string_using_delimiter(line, "|#|"))
        .collect();

    for detail in details {
        version_and_release_map.insert(
            detail[4].clone(), 
            Vec::from([detail[0].clone(), detail[1].clone(), detail[2].clone()])
        );

        version_and_release_map.insert(
            detail[5].clone(), 
            Vec::from([detail[0].clone(), detail[1].clone(), detail[2].clone()])
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
    let processed_details: HashMap<String, Vec<String>> = extract_version_and_release_map(remote_details.clone());
    let packages_names: HashMap<String, Vec<String>> = processed_details
        .clone()
        .into_values()
        .map(|item| item[0].clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .map(|package_name| dnf::get_installed_details(package_name))
        .map(|line| split_string_using_delimiter(line, "|#|"))
        .map(|details| (details[0].clone(), Vec::from([details[1].clone(), details[2].clone()])))
        .collect::<HashMap<String, Vec<String>>>();
    
    let update_builder: UpdateBuilder = UpdateBuilder::new(
        processed_details, packages_names
    );

    let updates: Vec<Update> = available_updates
                                .into_iter()
                                .map(|line| update_builder.from_dnf_output(line))
                                .collect();


    for update in updates {
        partition_builder.register_update(update.clone());
    }
    
    let partitions = partition_builder.build();
    
    for (partition_id, partition) in &partitions {
        if *partition.get_release_type() <= ReleaseType::Patch || *partition.get_severity() > Severity::None {
            println!(
                "\nPartition Id: \"{}\" \nType: {} \nSecurity grade: {}", 
                partition_id, partition.get_release_type(), partition.get_severity()
            );
            for _update in partition.get_updates().into_iter() {
                let update_info: &Vec<String> = processed_details.get(&_update.get_signature().clone()).unwrap();
                let installed_info: &Vec<String> = packages_names.get(&update_info[0]).unwrap();
                println!("\t{:55} {}-{} -> {}-{}", update_info[0], installed_info[0], installed_info[1], update_info[1], update_info[2]);
            }
        }
    }
}
