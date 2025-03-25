mod model;
use model::data_model_builder::DataModelBuilder;
use model::enums::release_type::get_super;
use model::enums::severity::Severity;
use model::enums::release_type::ReleaseType;

mod system;
use model::partition::Partition;
use system::shell;
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
    
    let dnf_updates_list: Vec<&str> = dnf::get_updates_list(shell::run_command_and_read_stdout);

    if dnf_updates_list.is_empty() {
        println!("\nno suggested updates found\n\n");
        process::exit(0);
    }

    let repository_update_details: Vec<&str> = dnf::get_updates_details(
        &dnf_updates_list, shell::run_command_and_read_stdout
    );
    let packages_names: Vec<&str> = dnf::get_installed_details(
        &repository_update_details, 
        shell::run_command_and_read_stdout
    );
    
    println!("[i] enriching updates with additional informations...");

    let (partitions, updates) = {
        let mut data_model_builder: DataModelBuilder<'_> = DataModelBuilder::new();

        dnf_updates_list.iter().for_each(|line| data_model_builder.add_updateinfo_output(line));
        repository_update_details.iter().for_each(|line| data_model_builder.add_repoquery_output(line));
        packages_names.iter().for_each(|line| data_model_builder.add_rpm_output(line));

        data_model_builder.build()
    };

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

    let buffer = buffer;
    let selected_part_id = selected_part_id;

    println!("{}", buffer);
    //ui::display_suggested_upgrades(&update_builder, buffer);

    if !selected_part_id.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_part_id.join(","));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}
