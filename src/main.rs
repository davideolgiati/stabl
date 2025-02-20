mod system;
use system::os;
use system::dnf;

mod model;
use model::release_type::ReleseType;
use model::update::Update;

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

    let _available_updates: String = dnf::get_available_updates();

    let test: Update = Update::new(
        String::from("pippo"),
        ReleseType::Patch,
        String::from("test"),
        String::from("pluto"),
        String::from("paperino")
    );

    print!("{:?}", test);
}
