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

/*
pub fn display_suggested_upgrades(update_builder: &UpdateBuilder, buffer: String) {
        println!(
                "{}\nMajor   updates: {}\nMinor   updates: {}\nPatch   updates: {}\nRelease updates: {}\n\n", 
                buffer, update_builder.get_major_count(),
                update_builder.get_minor_count(), 
                update_builder.get_patch_count(),
                update_builder.get_release_count()
        );
}
*/