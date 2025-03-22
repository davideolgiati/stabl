mod model;
use model::data_model_builder::DataModelBuilder;
use model::enums::release_type::get_super;
use model::enums::severity::Severity;
use model::enums::release_type::ReleaseType;

mod system;
use model::partition::Partition;
use system::ui;
use system::dnf;
use system::args;

mod commons;

use std::env;
use std::process;

use chrono::Utc;

fn evaluate_partition(partition: &Partition, target_release: &ReleaseType) -> bool {
    let super_release_type: ReleaseType = get_super(target_release);
    let additional_date_check: bool = (Utc::now() - *partition.get_date()).num_days() > 60;

    let is_release_type_valid: bool = target_release >= partition.get_release_type();
    let is_super_release_type_valid: bool = &super_release_type >= partition.get_release_type();

    let is_partition_a_security_update: bool = partition.get_severity() > &Severity::None;
    let is_partition_ammissible_in_time_range: bool = is_super_release_type_valid && additional_date_check;

    is_partition_a_security_update || is_release_type_valid || is_partition_ammissible_in_time_range
}

fn main() {
    let input_args: Vec<String> = env::args().collect();
    
    ui::display_stabl_logo();
    args::look_for_help(&input_args);
    
    let max_release: ReleaseType = args::get_release_arg(&input_args);
    
    ui::display_system_informations();
    
    let dnf_updates_list: Vec<String> = dnf::get_updates_list();

    if dnf_updates_list.is_empty() {
        println!("\nno suggested updates found\n\n");
        process::exit(0);
    }

    let updates_refs: Vec<&str> = dnf_updates_list.iter().map(|s| s.as_ref()).collect();
    let repository_update_details: Vec<String> = dnf::get_updates_details(&updates_refs);
    let details_refs: Vec<&str> = repository_update_details.iter().map(|s| s.as_ref()).collect();
    let packages_names: Vec<String> = dnf::get_installed_details(&details_refs);
    
    println!("[i] enriching updates with additional informations...");

    let mut data_model_builder = DataModelBuilder::new();

    dnf_updates_list.iter().for_each(|line| data_model_builder.add_updateinfo_output(line));
    repository_update_details.iter().for_each(|line| data_model_builder.add_repoquery_output(line));
    packages_names.iter().for_each(|line| data_model_builder.add_rpm_output(line));

    let (partitions, updates) = data_model_builder.build();

    let mut selected_part_id: Vec<String> = Vec::new();
    let mut buffer = String::from("");

    for partition in &partitions {
        if evaluate_partition(partition, &max_release) {
            selected_part_id.push(partition.get_id().clone());

            let update_type_str = format!("{}", partition.get_release_type());

            buffer = buffer + &format!(
                "\n\n\nPartition Id: {:30} Type: {:15} Security grade: {}\n\n", 
                partition.get_id(), update_type_str, partition.get_severity()
            );

            for _update in updates.get(partition.get_id()).unwrap().iter() {
                buffer = buffer + &format!(
                    "\t{:<35} {:^25} ({:^3} days ago)\n", 
                    _update.get_name(),
                    format!("{}", _update.get_version()),
                    (Utc::now() - partition.get_date()).num_days()
                );
            }
        }
    }

    println!("{}", buffer);
    //ui::display_suggested_upgrades(&update_builder, buffer);

    if !selected_part_id.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_part_id.join(","));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}
