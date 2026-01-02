pub mod options {
    use std::process::exit;

	const VERSION: &str = "v1.0.2";

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
					println!("usage: eui64tool [options] [input...]");
					println!("  ...where 'input' is a MAC address, EUI64 IPv6 address, EUI64 suffix, or interface name");
					println!("  The tool will know what to do based on what input you give it\n");
					println!("  options:");
					println!("      -h, --help      Show this help page");
					println!("      -v, --version   Show version number and information");
                    exit(0);
				},
				Flag::Version => {
					println!("EUI-64 intelligent conversion tool ({})", VERSION);
					println!("by Seth Adkins (https://github.com/sethechosenone/eui64tool)");
                    exit(0);
				}
			}
		} else {
			println!("Error: unknown option -- {}", flag);
		}
	}
}
