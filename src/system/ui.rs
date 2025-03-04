use crate::{model::updates::builder::UpdateBuilder, system::os};

pub fn display_stabl_logo() {
        let logo:&str = r"          
                                    __________________________________
         _        _     _          \__________________________________/
     ___| |_ __ _| |__ | |           )_(   )_(              )_(   )_(
    / __| __/ _` | '_ \| |           | |   | |              | |   | |
    \__ \ || (_| | |_) | |           | |   | |              | |   | | 
    |___/\__\__,_|_.__/|_|           |_|   |_|              |_|   |_|
    
    A DNF wrapper to selectively choose what packages to upgrade
    
        ";
        println!("{}", logo);
}
    
pub fn display_system_informations() {
        println!("[*] running on: {}\n", os::get_os_name());
}

pub fn display_suggested_upgrades(update_builder: &UpdateBuilder) {
        println!("\nMajor   updates: {}", update_builder.get_major_count());
        println!("Minor   updates: {}", update_builder.get_minor_count());
        println!("Patch   updates: {}", update_builder.get_patch_count());
        println!("Release updates: {}\n\n", update_builder.get_release_count());
}