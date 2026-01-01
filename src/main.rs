use std::env::args;
use std::process::exit;
use getifs::interfaces;
use regex::Regex;

fn to_eui64(mac_addr: String) -> String {
	let mut mac_segments: Vec<&str> = mac_addr.split(':').collect();
	let mut suffix_segments: [String; 4] = [const { String::new() }; 4]; 
	let mut result: String;
	
	// flip 7th bit of first MAC segment
	let mut first_segment_int: u8 = u8::from_str_radix(mac_segments[0], 16).expect("Invalid first MAC segment, somehow");
	first_segment_int ^= 0x02;
	let new_first_segment = &format!("{:02x}", first_segment_int);
	mac_segments[0] = new_first_segment;

	// assemble suffix
	suffix_segments[0] = mac_segments[0].to_owned() + mac_segments[1];
	suffix_segments[1] = mac_segments[2].to_owned() + "ff";
	suffix_segments[2] = String::from("fe") + mac_segments[3];
	suffix_segments[3] = mac_segments[4].to_owned() + mac_segments[5];

	result = suffix_segments.join(":");
	result.insert(0, ':');

	result
}

fn from_eui64(eui64_suffix: String) -> String {
	let suffix_segments: Vec<&str> = eui64_suffix.split(':').collect();

	// split into 8-bit colon-separated bytes
	let mut bytes: Vec<String> = Vec::new();
	for segment in suffix_segments.iter().skip(1) { // first element is :, so skip
		bytes.push(segment[0..2].to_string());
		bytes.push(segment[2..4].to_string());
	}

	// remove ff:fe
	bytes.remove(3);
	bytes.remove(3);

	// flip 7th bit of first byte
	let mut first_byte_int: u8 = u8::from_str_radix(&bytes[0], 16).expect("Invalid IPv6 suffix segment, somehow");
	first_byte_int ^= 0x02;
	bytes[0] = format!("{:02x}", first_byte_int);

	bytes.join(":")
}

fn expand_ipv6(ipv6: &str) -> String {
	if !ipv6.contains("::") {
		return ipv6.to_string();
	}

	// split on :: to get left and right parts
	let parts: Vec<&str> = ipv6.split("::").collect();
	let left = if parts[0].is_empty() { vec![] } else { parts[0].split(':').collect::<Vec<&str>>() };
	let right = if parts.len() > 1 && !parts[1].is_empty() { parts[1].split(':').collect::<Vec<&str>>() } else { vec![] };

	// calculate how many zero groups we need
	let num_groups = left.len() + right.len();
	let zeros_needed = 8 - num_groups;

	// build the expanded address
	let mut expanded = left.clone();
	for _ in 0..zeros_needed {
		expanded.push("0");
	}
	expanded.extend(right);

	expanded.join(":")
}

fn main() {
	let args: Vec<String> = args().collect();
	let interfaces = interfaces().unwrap();

	if args.len() > 1 {
		// argument provided
		let input = &args[1];

		if let Some(iface) = interfaces.iter().find(|i| i.name() == input) {
			// argument is iface, run on iface
			if let Some(mac_addr) = iface.mac_addr() {
				println!("IPv6 EUI-64 suffix for {}: {}", iface.name(), to_eui64(mac_addr.to_string()));
			} else {
				eprintln!("Error: Interface '{}' has no MAC address", input);
				exit(1);
			}
		} else {
			// argument not iface, check if it's a valid MAC address or IPv6
			let mac_regex = Regex::new(r"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$").unwrap();
			let eui64_suffix_regex = Regex::new(r"^:([0-9a-fA-F]{4}:){3}[0-9a-fA-F]{4}$").unwrap();
			let ipv6_regex = Regex::new(r"^([0-9a-fA-F]{0,4}:){2,7}[0-9a-fA-F]{0,4}$").unwrap();

			if mac_regex.is_match(input) {
				// argument is MAC address, convert to EUI-64
				println!("IPv6 EUI-64 suffix for {}: {}", input, to_eui64(input.clone()));
			} else if eui64_suffix_regex.is_match(input) || ipv6_regex.is_match(input) {
				// argument is IPv6 or EUI-64 suffix, convert to MAC
				let suffix = if input.starts_with(':') {
					// pad segments to 4 chars
					let parts: Vec<&str> = input.split(':').skip(1).collect();
					let padded = parts.iter()
						.map(|s| format!("{:0>4}", s))
						.collect::<Vec<_>>()
						.join(":");

					format!(":{}", padded)
				} else {
					// convert full IPv6 to just suffix
					let expanded = expand_ipv6(input);
					let parts: Vec<&str> = expanded.split(':').collect();
					// pad each segment to 4 characters and take last 4
					let suffix_part = parts.iter()
						.rev()
						.take(4)
						.rev()
						.map(|s| format!("{:0>4}", s))
						.collect::<Vec<_>>()
						.join(":");

					format!(":{}", suffix_part)
				};

				// validate that this is actually an EUI-64 address before converting
				let suffix_parts: Vec<&str> = suffix.split(':').skip(1).collect();
				if suffix_parts.len() != 4 {
					eprintln!("Error: '{}' is not a valid EUI-64 address", input);
					exit(1);
				} else {
					// check for ff:fe marker in the middle (across parts[1] and parts[2])
					let byte3 = &suffix_parts[1][2..4];
					let byte4 = &suffix_parts[2][0..2];
					if byte3.to_lowercase() != "ff" || byte4.to_lowercase() != "fe" {
						eprintln!("Error: '{}' is not a valid EUI-64 address (missing ff:fe marker)", input);
						exit(1);
					} else {
						println!("MAC address for {}: {}", input, from_eui64(suffix));
					}
				}
			} else {
				// check for help flag
                if input == "-h" || input == "--help" {
                    println!("usage: eui64tool [ifname] [mac_address] [ipv6_eui64_address] [ipv6_eui64_suffix]");
                } else {
                    // no other possible way to interpret this input -- it's completely invalid
				    eprintln!("Error: '{}' is not a valid interface, MAC address, or IPv6 address", input);
				    exit(1);
                }
			}
		}
	} else {
		// no argument provided, run on every iface
		for iface in interfaces {
			if let Some(mac_addr) = iface.mac_addr() {
				println!("IPv6 EUI-64 suffix for {}: {}", iface.name(), to_eui64(mac_addr.to_string()));
			}
		}
	}
}
