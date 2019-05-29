fn main() {
	println!("cargo:rustc-link-search=native=/home/daxorinator/rust-wii/");
	println!("cargo:rustc-link-lib=static=MLlib");
}
