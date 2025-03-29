mod model;
use model::SecurityClassification;
use model::Partition;
use model::ModelBuilder;
use model::semantic_version::get_super;
use model::semantic_version::SemanticVersion;

mod system;
use system::shell;
use system::shell_cmd_facade::get_repoquery_output;
use system::shell_cmd_facade::get_rpm_output_for_local_packages;
use system::shell_cmd_facade::get_updateinfo_output;
use system::ui;
use system::args;

mod commons;

use std::env;
use std::process;

use chrono::Utc;

fn evaluate_partition(partition: &Partition, target_release: &SemanticVersion) -> bool {
    let super_release_type: SemanticVersion = get_super(target_release);
    let additional_date_check: bool = (Utc::now() - *partition.get_date()).num_days() > 60;

    let is_release_type_valid: bool = target_release >= partition.get_release_type();
    let is_super_release_type_valid: bool = &super_release_type >= partition.get_release_type();

    let is_partition_a_security_update: bool = partition.get_security_classification() > &SecurityClassification::None;
    let is_partition_ammissible_in_time_range: bool = is_super_release_type_valid && additional_date_check;

    is_partition_a_security_update || is_release_type_valid || is_partition_ammissible_in_time_range
}

fn main() {
    let input_args: Vec<String> = env::args().collect();
    
    ui::display_stabl_logo();
    args::look_for_help(&input_args);
    
    let max_release: SemanticVersion = args::get_release_arg(&input_args);
    
    let dnf_updates_list: Vec<&str> = get_updateinfo_output(shell::run_command_and_read_stdout);

    if dnf_updates_list.is_empty() {
        println!("\nno suggested updates found\n\n");
        process::exit(0);
    }

    let repository_update_details: Vec<&str> = get_repoquery_output(
        &dnf_updates_list, shell::run_command_and_read_stdout
    );
    let packages_names: Vec<&str> = get_rpm_output_for_local_packages(
        &repository_update_details, 
        shell::run_command_and_read_stdout
    );
    
    println!("[i] enriching updates with additional informations...");

    let (partitions, updates) = {
        let mut data_model: ModelBuilder<'_> = ModelBuilder::new();

        dnf_updates_list.iter().for_each(|line| data_model.add_updateinfo_output_line(line));
        repository_update_details.iter().for_each(|line| data_model.add_repoquery_output(line));
        packages_names.iter().for_each(|line| data_model.add_rpm_output(line));

        data_model.build()
    };

    let mut selected_partition_ids: Vec<String> = Vec::new();
    let mut stdout_buffer = String::from("");

    for partition in &partitions {
        if evaluate_partition(partition, &max_release) {
            selected_partition_ids.push(partition.get_id().clone());

            let update_type_str = format!("{}", partition.get_release_type());

            stdout_buffer = stdout_buffer + &format!(
                "\n\n\nPartition Id: {:30} Type: {:15} Security grade: {}\n\n", 
                partition.get_id(), update_type_str, partition.get_security_classification()
            );

            for _update in updates.get(partition.get_id()).unwrap().iter() {
                stdout_buffer = stdout_buffer + &format!(
                    "\t{:<35} {:^25} ({:^3} days ago)\n", 
                    _update.get_name(),
                    format!("{}", _update.get_version()),
                    (Utc::now() - partition.get_date()).num_days()
                );
            }
        }
    }

    let stdout_buffer = stdout_buffer;
    let selected_partition_ids = selected_partition_ids;

    println!("{}", stdout_buffer);
    //ui::display_suggested_upgrades(&update_builder, buffer);

    if !selected_partition_ids.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_partition_ids.join(","));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}
