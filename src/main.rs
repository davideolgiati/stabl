mod model;
use model::SecurityClassification;
use model::Partition;
use model::ModelBuilder;
use model::semantic_version::SemanticVersion;

mod system;
use system::shell;
use system::dnf::get_repoquery_output;
use system::dnf::get_rpm_output_for_local_packages;
use system::dnf::get_updateinfo_output;
use system::ui;
use system::args;

mod commons;

use std::env;
use std::process;

use chrono::Utc;

fn evaluate_partition(partition: &Partition, target_release: &SemanticVersion) -> bool {
    let version_bump_check: bool = target_release >= partition.get_release_type();
    let security_release_check: bool = *partition.get_security_classification() > SecurityClassification::None;

    security_release_check || version_bump_check
}

fn main() {
    let input_args: Vec<String> = env::args().collect();
    
    ui::display_stabl_logo();
    args::look_for_help(&input_args);
    
    let max_release: SemanticVersion = args::get_release_arg(
        &input_args, SemanticVersion::Patch
    );
    
    let dnf_updates_list: Vec<&str> = get_updateinfo_output(shell::run_command_and_read_stdout);

    if dnf_updates_list.is_empty() {
        println!("\nno suggested updates found\n\n");
        process::exit(0);
    }

    let repository_update_details: Vec<&str> = get_repoquery_output(
        &dnf_updates_list, shell::run_command_and_read_stdout
    );

    if repository_update_details.is_empty() {
        println!("\nno suggested updates found\n\n");
        process::exit(0);
    }

    let packages_names: Vec<&str> = get_rpm_output_for_local_packages(
        &repository_update_details, 
        shell::run_command_and_read_stdout
    );
    
    if packages_names.is_empty() {
        println!("\nno suggested updates found\n\n");
        process::exit(0);
    }

    println!("[i] enriching updates with additional informations...");

    let (partitions, updates) = {
        let mut data_model: ModelBuilder<'_> = ModelBuilder::new();

        dnf_updates_list.iter().for_each(|line| data_model.add_updateinfo_output_line(line));
        repository_update_details.iter().for_each(|line| data_model.add_repoquery_output(line));
        packages_names.iter().for_each(|line| data_model.add_rpm_output(line));

        data_model.build()
    };

    let selected_partitions: Vec<&Partition> = partitions
        .iter()
        .filter(|partition| evaluate_partition(partition, &max_release))
        .collect();

    let selected_partitions_id: Vec<&str> = selected_partitions
        .iter()
        .map(|partition| partition.get_id().as_str())
        .collect();

    for partition in &selected_partitions {
        let id: &String = partition.get_id();
        let update_type: String = format!("{}", partition.get_release_type());
        let security_class: String = format!("{}", partition.get_security_classification());

        let updates = updates.get(partition.get_id()).unwrap();

        print!(
            "\nPartition Id: {:30} Type: {:15} Security grade: {}\n\n", 
            id, update_type, security_class
        );

        for _update in updates {
            let package: &String =  _update.get_name();
            let version: String =  format!("{}", _update.get_version());
            let days_since_release: i64 = (Utc::now() - partition.get_date()).num_days();


            println!(
                "    {:<35} {:^25} ({:^3} days ago)", 
                package, version, days_since_release
            );
        }
    }

    if !selected_partitions_id.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_partitions_id.join(", "));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}
