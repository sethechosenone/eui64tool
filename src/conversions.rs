pub mod conversions {
	pub fn to_eui64(mac_addr: String) -> String {
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

	pub fn from_eui64(eui64_suffix: String) -> String {
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
}
