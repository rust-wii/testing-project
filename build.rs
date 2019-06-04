fn main() {
    println!("cargo:rustc-link-search=native=C:/devkitPro/devkitPPC/powerpc-eabi/lib");
//    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-search=native=C:/devkitPro/libogc/lib/wii");
    println!("cargo:rustc-link-lib=static=ogc");
    println!("cargo:rustc-link-lib=static=png");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:rustc-link-lib=static=wiiuse");
    println!("cargo:rustc-link-lib=static=wiikeyboard");
    println!("cargo:rustc-link-lib=static=tinysmb");
    println!("cargo:rustc-link-lib=static=modplay");
    println!("cargo:rustc-link-lib=static=mad");
    println!("cargo:rustc-link-lib=static=iso9660");
    println!("cargo:rustc-link-lib=static=fat");
    println!("cargo:rustc-link-lib=static=di");
    println!("cargo:rustc-link-lib=static=db");
    println!("cargo:rustc-link-lib=static=bte");
    println!("cargo:rustc-link-lib=static=asnd");
    println!("cargo:rustc-link-lib=static=aesnd");
    println!("cargo:rustc-link-lib=static=sysbase");
//    println!("cargo:rustc-link-lib=static=crtmain.o");
//    println!("cargo:rustc-link-lib=static=crt0.o");
}
