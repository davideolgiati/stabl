mod model;
use model::partitions::partition::Partition;
use model::updates::update::Update;
use model::enums::severity::Severity;
use model::updates::builder::UpdateBuilder;
use model::enums::release_type::ReleaseType;
use model::partitions::builder::PartitionBuilder;

mod system;
use system::ui;
use system::dnf;
use system::args;

mod commons;
use commons::string::split_string_using_delimiter;

use std::collections::HashMap;
use std::env;

fn extract_version_and_release_map(details_from_repository: &[String]) -> HashMap<String, Vec<String>> {
    details_from_repository
        .iter()
        .cloned()
        .map(|line| split_string_using_delimiter(line, "|#|"))
        .flat_map(|tokens| Vec::from([
            (tokens[3].to_owned(), tokens[..=2].to_owned()), 
            (tokens[4].to_owned(), tokens[..=2].to_owned())
        ]))
        .collect::<HashMap<String, Vec<String>>>()
}

fn build_partitions(updates: &Vec<Update>) -> HashMap<String, Partition> {
    println!("[i] building update partitions...");
    
    let mut partition_builder:PartitionBuilder = PartitionBuilder::new();
    
    for update in updates {
        partition_builder.register_update(update.clone());
    }
    
    partition_builder.build()
}

fn main() {
    let input_args: Vec<String> = env::args().collect();
    
    ui::display_stabl_logo();
    args::look_for_help(&input_args);
    
    let max_release: ReleaseType = args::get_release_arg(&input_args);
    
    ui::display_system_informations();
    
    let dnf_updates_list: Vec<String> = dnf::get_updates_list();
    let repository_update_details: Vec<String> = dnf::get_updates_details(&dnf_updates_list);
    let packages_names: HashMap<String, Vec<String>> = dnf::get_installed_details(&repository_update_details);
    let processed_details: HashMap<String, Vec<String>> = extract_version_and_release_map(&repository_update_details);
    
    println!("[i] enriching updates with additional informations...");

    let mut update_builder: UpdateBuilder = UpdateBuilder::new(
        &processed_details, &packages_names
    );

    for line in dnf_updates_list {
        if update_builder.check_dnf_output_valididty(&line) {
            update_builder.add_dnf_output(&line)
        }
    }

    let updates: &Vec<Update> = update_builder.get_updates();

    let partitions: HashMap<String, Partition> = build_partitions(updates);
    let mut selected_part_id: Vec<String> = Vec::new();
    let mut buffer = String::from("");


    for (partition_id, partition) in &partitions {
        if *partition.get_release_type() <= max_release || *partition.get_severity() > Severity::None {
            selected_part_id.push(partition_id.clone());

            buffer = buffer + &format!(
                "\nPartition Id: {:30} Type: {:15} Security grade: {}\n", 
                partition_id, partition.get_release_type(), partition.get_severity()
            );

            for _update in partition.get_updates().iter() {
                buffer = buffer + &format!(
                    "\t{:55} {} -> {}\n", 
                    _update.get_name(), 
                    _update.get_installed_version(), 
                    _update.get_updated_version()
                );
            }
        }
    }

    ui::display_suggested_upgrades(&update_builder, buffer);

    if !selected_part_id.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_part_id.join(","));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}
