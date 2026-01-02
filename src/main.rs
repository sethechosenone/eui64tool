use std::env::args;
use std::process::exit;
use getifs::interfaces;
use regex::Regex;

use crate::options::options::handle_flag;
use crate::conversions::conversions::{to_eui64, from_eui64};

mod options;
mod conversions;

enum InputType {
	Interface(String),
	MacAddress(String),
	Ipv6Address(String),
	Eui64Suffix(String),
	Flag(String),
	Invalid(String),
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

fn classify_input(input: &str) -> InputType {
	// check for flags first
	if input.starts_with('-') {
		return InputType::Flag(input.to_string());
	}

	// check for interface name
	let interfaces = interfaces().unwrap();
	if interfaces.iter().any(|i| i.name() == input) {
		return InputType::Interface(input.to_string());
	}

	// check regex patterns
	let mac_regex = Regex::new(r"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$").unwrap();
	let eui64_suffix_regex = Regex::new(r"^:([0-9a-fA-F]{4}:){3}[0-9a-fA-F]{4}$").unwrap();
	let ipv6_regex = Regex::new(r"^([0-9a-fA-F]{0,4}:){2,7}[0-9a-fA-F]{0,4}$").unwrap();

	if mac_regex.is_match(input) {
		InputType::MacAddress(input.to_string())
	} else if eui64_suffix_regex.is_match(input) {
		InputType::Eui64Suffix(input.to_string())
	} else if ipv6_regex.is_match(input) {
		InputType::Ipv6Address(input.to_string())
	} else {
		InputType::Invalid(input.to_string())
	}
}

fn handle_interface(name: &str) {
	let interfaces = interfaces().unwrap();

	if let Some(iface) = interfaces.iter().find(|i| i.name() == name) {
		if let Some(mac_addr) = iface.mac_addr() {
			println!("IPv6 EUI-64 suffix for {}: {}", iface.name(), to_eui64(mac_addr.to_string()));
		} else {
			eprintln!("Error: Interface '{}' has no MAC address", name);
			exit(1);
		}
	}
}

fn handle_mac_address(mac: &str) {
	println!("IPv6 EUI-64 suffix for {}: {}", mac, to_eui64(mac.to_string()));
}

fn handle_ipv6_or_suffix(input: &str) {
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
	}

	// check for ff:fe marker in the middle (across parts[1] and parts[2])
	let byte3 = &suffix_parts[1][2..4];
	let byte4 = &suffix_parts[2][0..2];
	if byte3.to_lowercase() != "ff" || byte4.to_lowercase() != "fe" {
		eprintln!("Error: '{}' is not a valid EUI-64 address (missing ff:fe marker)", input);
		exit(1);
	}

	println!("MAC address for {}: {}", input, from_eui64(suffix));
}

fn handle_input(input_type: InputType) {
	match input_type {
		InputType::Interface(name) => handle_interface(&name),
		InputType::MacAddress(mac) => handle_mac_address(&mac),
		InputType::Ipv6Address(ipv6) => handle_ipv6_or_suffix(&ipv6),
		InputType::Eui64Suffix(suffix) => handle_ipv6_or_suffix(&suffix),
		InputType::Flag(flag) => handle_flag(&flag),
		InputType::Invalid(input) => {
			eprintln!("Error: '{}' is not a valid interface, MAC address, or IPv6 address", input);
			exit(1);
		}
	}
}

fn main() {
	let args: Vec<String> = args().collect();

	if args.len() > 1 {
		// argument provided, classify and handle it
		for i in 1..args.len() {
			handle_input(classify_input(&args[i]));
		}
	} else {
		// no argument provided, list all interfaces
		for iface in interfaces().unwrap() {
			if let Some(mac_addr) = iface.mac_addr() {
				println!("IPv6 EUI-64 suffix for {}: {}", iface.name(), to_eui64(mac_addr.to_string()));
			}
		}
	}
}
