fn main() {
    let dkp_path = std::env::var("DEVKITPRO").unwrap();
    
    println!("cargo:rustc-link-search=native={}/MLlib/lib", dkp_path);
    println!("cargo:rustc-link-search=native={}/libogc/lib/wii", dkp_path);
    println!(
        "cargo:rustc-link-search=native={}/devkitPPC/powerpc-eabi/lib",
        dkp_path
    );
    println!("cargo:rustc-link-lib=static=MLlib");
    println!("cargo:rustc-link-lib=static=wiiuse");
    println!("cargo:rustc-link-lib=static=ogc");
    println!("cargo:rustc-link-lib=static=png");
    println!("cargo:rustc-link-lib=static=bte");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-lib=static=asnd");
    println!("cargo:rustc-link-lib=static=sysbase");
}
