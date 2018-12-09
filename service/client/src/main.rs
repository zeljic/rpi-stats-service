extern crate crypto;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;
use std::env;

use std::fs::File;
use std::io::prelude::*;

#[inline(always)]
fn get_machine_id() -> String {
	match env::var("CLIENT_MACHINE_ID") {
		Ok(v) => v,
		Err(_) => {
			if cfg!(unix) {
				let mut f = File::open("/etc/machine-id").unwrap();
				let mut content: String = String::new();

				f.read_to_string(&mut content).unwrap();

				content
			} else {
				eprintln!("Unable to get unique machine id, add env variable CLIENT_MACHINE_ID if there isn't /etc/machine-id on the system.");
				unimplemented!()
			}
		}
	}
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
	dotenv().ok();

	println!("MACHINE_ID: {}; UUID: {};", *MACHINE_ID, *UUID);
}
