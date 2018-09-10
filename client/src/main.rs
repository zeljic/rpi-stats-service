extern crate crypto;
#[macro_use]
extern crate lazy_static;

#[cfg(linux)]
#[inline(always)]
fn get_machine_id() -> String {
	use std::fs::File;
	use std::io::prelude::*;

	let mut f = File::open("/etc/machine-id").unwrap();
	let mut content: String = String::new();

	f.read_to_string(&mut content).unwrap();

	content
}

#[cfg(not(linux))]
#[inline(always)]
fn get_machine_id() -> String {
	"windows_machine_id".to_string()
}

#[inline(always)]
fn get_uuid() -> String {
	use crypto::digest::Digest;
	use crypto::sha2::Sha256;

	let mut sha = Sha256::new();
	sha.input_str(&get_machine_id());

	sha.result_str()
}

lazy_static! {
	static ref MACHINE_ID: String = get_machine_id();
	static ref UUID: String = get_uuid();
}

fn main() {
	println!("MACHINE_ID: {}; UUID: {};", *MACHINE_ID, *UUID);
}
