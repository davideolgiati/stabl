use crate::system::os;

pub fn display_stabl_logo() {
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
    
pub fn display_system_informations() {
        println!("[*] running on: {}\n", os::get_os_name());
}