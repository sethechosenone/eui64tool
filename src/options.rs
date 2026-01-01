pub mod options {
    const VERSION: &str = "v1.0.1";

    enum Flag {
        Help,
        Version
    }

    fn handle_option(flag: &str) -> Option<Flag> {
        match flag {
            "-h" => Some(Flag::Help),
            "--help" => Some(Flag::Help),
            "-v" => Some(Flag::Version),
            "--version" => Some(Flag::Version),
            _ => None
        }
    }

    pub fn handle_flag(flag: &str) {
        if let Some(opt) = handle_option(flag) {
            match opt {
                Flag::Help => {
                    println!("usage: eui64tool [options] [ifname] [mac_address] [ipv6_eui64_address] [ipv6_eui64_suffix]");
                    println!("  options:");
                    println!("      -h, --help      Show this help page");
                    println!("      -v, --version   Show version number and information");
                },
                Flag::Version => {
                    println!("EUI-64 intelligent conversion tool ({})", VERSION);
                    println!("by Seth Adkins (https://github.com/sethechosenone/eui64tool)");
                }
            }
        } else {
            println!("Error: unknown option -- {}", flag);
        }
    }
}
