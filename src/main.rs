mod model;
use model::SecurityClassification;
use model::Partition;
use model::ModelBuilder;
use model::semantic_version::SemanticVersion;

mod system;
use system::args::get_verbosity_arg;
use system::shell;
use system::dnf::get_repoquery_output;
use system::dnf::get_rpm_output_for_local_packages;
use system::dnf::get_updateinfo_output;
use system::args::get_skip_security_updates_arg;
use system::ui;
use system::args;
use system::logger;

mod commons;

use std::env;
use std::process;

use chrono::Utc;

fn evaluate_partition(partition: &Partition, target_release: &SemanticVersion, get_security_updates: bool) -> bool {
    let version_bump_check: bool = target_release >= partition.get_release_type();
    let security_release_check: bool = *partition.get_security_classification() > SecurityClassification::None;

    (get_security_updates && security_release_check) || version_bump_check
}

fn get_rpm_packages_details<'a>(repository_update_details: &Vec<&str>) -> Vec<&'a str> {
    info!("getting details from installed packages ...");
    
    let packages_names: Vec<&str> = get_rpm_output_for_local_packages(
        repository_update_details, 
        shell::run_command_and_read_stdout
    );
    
    if packages_names.is_empty() {
        info!("\nno installed packages fuond for suggested updates\n\n");
        process::exit(0);
    }
    info!("found details from {} installed packages", packages_names.len());
    packages_names
}

fn get_dnf_repoquery<'a>(dnf_updates_list: &Vec<&str>) -> Vec<&'a str> {
    info!("getting details from repository for updates ...");
    let repository_update_details: Vec<&str> = get_repoquery_output(
        dnf_updates_list, shell::run_command_and_read_stdout
    );

    if repository_update_details.is_empty() {
        info!("\nno details found for suggested updates in repository\n\n");
        process::exit(0);
    }
    info!("found {} unique updates details from remote repository", repository_update_details.len());
    repository_update_details
}

fn get_dnf_updateinfo<'a>() -> Vec<&'a str> {
    info!("getting updates list from remote...");
    let dnf_updates_list: Vec<&str> = get_updateinfo_output(shell::run_command_and_read_stdout);
    
    if dnf_updates_list.is_empty() {
        info!("\nno suggested updates found\n\n");
        process::exit(0);
    }
    info!("found {} updates from remote repository", dnf_updates_list.len());
    dnf_updates_list
}

fn main() {
    ui::display_stabl_logo();
    
    let input_args: Vec<String> = env::args().collect();
    
    let verbosity: logger::LoggingLevel = get_verbosity_arg(&input_args);
    crate::system::logger::set_logging_level(verbosity);
    debug!("stabl started!");
    
    args::look_for_help(&input_args);
    
    trace!("Total arg count provided to stabl: {}", input_args.len());

    let target_release: SemanticVersion = args::get_release_arg(&input_args);
    debug!("Release upper limit for version bump set to: {}", target_release);

    let get_security_updates: bool = get_skip_security_updates_arg(&input_args);
    debug!("Security update flag is set to: {}", get_security_updates);
    
    let dnf_updates_list = get_dnf_updateinfo();
    let repository_update_details = get_dnf_repoquery(&dnf_updates_list);
    let packages_names = get_rpm_packages_details(&repository_update_details);

    info!("enriching updates with additional informations...");

    let (partitions, updates) = {
        let mut data_model: ModelBuilder<'_> = ModelBuilder::new();

        dnf_updates_list.iter().for_each(|line| data_model.add_updateinfo_output_line(line));
        repository_update_details.iter().for_each(|line| data_model.add_repoquery_output(line));
        packages_names.iter().for_each(|line| data_model.add_rpm_output(line));

        data_model.build()
    };

    let (
        major, 
        minor, 
        patch, 
        release,
        selected_partitions,
        selected_partitions_id
    ) = {
        let mut patch: usize = 0;
        let mut minor: usize = 0;
        let mut major: usize = 0;
        let mut release: usize = 0;
        let mut selected_partitions: Vec<&Partition> = Vec::new();
        let mut selected_partitions_id: Vec<&str> = Vec::new();


        for partition in &partitions {
            let id = partition.get_id();
            let updates_count = updates.get(id).unwrap().len(); 

            match partition.get_release_type() {
                SemanticVersion::Major  => major += updates_count,
                SemanticVersion::Minor  => minor += updates_count,
                SemanticVersion::Patch  => patch += updates_count,
                SemanticVersion::Repack => release += updates_count,
            }

            if evaluate_partition(partition, &target_release, get_security_updates) {
                selected_partitions.push(partition);
                selected_partitions_id.push(id);
            }
        }

        (major, minor, patch, release, selected_partitions, selected_partitions_id)
    };

    for partition in &selected_partitions {
        let id: &String = partition.get_id();
        let update_type: String = format!("{}", partition.get_release_type());
        let security_class: String = format!("{}", partition.get_security_classification());

        let updates = updates.get(partition.get_id()).unwrap();
        let days_since_release: i64 = (Utc::now() - partition.get_date()).num_days();

        println!(
            "\n\x1b[1mPartition Id\x1b[0m: {:>25} \x1b[1mPublished\x1b[0m: {:>3} days ago\n\x1b[1mSeverity\x1b[0m: {:>29} \x1b[1mType\x1b[0m: {:>17}", 
            id,  days_since_release, security_class, update_type
        );

        for _update in updates {
            let package: &String = _update.get_name();
            let version: String = format!("{}", _update.get_version());

            println!(
                "    \x1b[1m{:<35}\x1b[0m {:<20}", package, version, 
            );
        }
    }

    println!(
        "\nFound:\n\t{} major updates\n\t{} minor updates\n\t{} bugfix upadtes\n\t{} repack updates\n", 
        major, minor, patch, release
    );

    if !selected_partitions_id.is_empty() {
        println!("\nsudo dnf update --advisory={}\n\n", selected_partitions_id.join(","));
    } else {
        println!("\nno suggested updates found\n\n");
    }
}

