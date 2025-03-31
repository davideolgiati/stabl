pub fn display_stabl_logo() {
        let logo:&str = "          
                                    __________________________________
    \x1b[92;1m     _        _     _ \x1b[0m         \\__________________________________/
    \x1b[92;1m ___| |_ __ _| |__ | |\x1b[0m           )_(   )_(              )_(   )_(
    \x1b[92;1m/ __| __/ _` | '_ \\| |\x1b[0m           | |   | |              | |   | |
    \x1b[92;1m\\__ \\ || (_| | |_) | |\x1b[0m           | |   | |              | |   | | 
    \x1b[92;1m|___/\\__\\__,_|_.__/|_|\x1b[0m           |_|   |_|              |_|   |_|
    
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