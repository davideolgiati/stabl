mod system;
use system::os;
use system::dnf;

mod model;
use model::update::Update;
use model::partitions::builder::PartitionBuilder;

mod commons;
use commons::string::split_string_using_delimiter;

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
    
    println!("[i] gruoping updates in partititons...");
    
    let mut partition_builder: PartitionBuilder = PartitionBuilder::new();
    let updates: Vec<Update> = available_updates
                                .into_iter()
                                .map(|line| Update::from_dnf_output(line))
                                .collect();
    let signatures: Vec<String> = updates
                                    .clone()
                                    .into_iter()
                                    .map(|update| update.get_signature().clone())
                                    .collect();
    let remote_details: Vec<String> = dnf::get_updates_details(signatures);
    let processed_details: Vec<Vec<String>> = remote_details
                                            .clone()
                                            .into_iter()
                                            .map(|line| split_string_using_delimiter(line, "|#|"))
                                            .collect();

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
