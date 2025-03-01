mod system;
use model::enums::release_type::ReleaseType;
use model::enums::severity::Severity;
use model::updates::builder::UpdateBuilder;
use system::ui;
use system::dnf;

mod model;
use model::updates::update::Update;
use model::partitions::builder::PartitionBuilder;

mod commons;
use commons::string::split_string_using_delimiter;

use std::collections::HashMap;

fn extract_version_and_release_map(details_from_repository: Vec<String>) -> HashMap<String, Vec<String>> {
    details_from_repository
        .clone()
        .into_iter()
        .map(|line| split_string_using_delimiter(line, "|#|"))
        .flat_map(|tokens| Vec::from([
            (tokens[3].to_owned(), tokens[..=2].to_owned()), 
            (tokens[4].to_owned(), tokens[..=2].to_owned())
        ]))
        .collect::<HashMap<String, Vec<String>>>()
}

fn main() {
    let mut partition_builder:PartitionBuilder = PartitionBuilder::new();
    
    ui::display_stabl_logo();
    ui::display_system_informations();
    
    let dnf_updates_list: Vec<String> = dnf::get_updates_list();
    let repository_update_details: Vec<String> = dnf::get_updates_details(&dnf_updates_list);
    let packages_names: HashMap<String, Vec<String>> = dnf::get_installed_details(&repository_update_details);

    let processed_details: HashMap<String, Vec<String>> = extract_version_and_release_map(repository_update_details.clone());
    
    println!("[i] enriching updates with additional informations...");

    let mut update_builder: UpdateBuilder = UpdateBuilder::new(
        &processed_details, &packages_names
    );

    let valid_updates: Vec<String> = dnf_updates_list
        .into_iter()
        .filter(|line| update_builder.check_dnf_output_valididty(line.clone()))
        .collect();

    let updates: Vec<Update> = valid_updates
        .into_iter()
        .map(|line| update_builder.add_dnf_output(line))
        .collect();

    println!("[i] building update partitions...");

    for update in updates {
        partition_builder.register_update(update.clone());
    }
    
    let partitions = partition_builder.build();
    let mut selected_part_id: Vec<String> = Vec::new();
    
    for (partition_id, partition) in &partitions {
        if *partition.get_release_type() <= ReleaseType::Patch || *partition.get_severity() > Severity::None {
            selected_part_id.push(partition_id.clone());

            println!(
                "\nPartition Id: {:30} Type: {:15} Security grade: {}", 
                partition_id, partition.get_release_type(), partition.get_severity()
            );

            for _update in partition.get_updates().iter() {
                println!("\t{:55} {} -> {}", _update.get_name(), _update.get_installed_version(), _update.get_updated_version());
            }
        }
    }

    ui::display_suggested_upgrades(&update_builder);

    if !selected_part_id.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_part_id.join(","));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}
