use std::env;
use std::path::PathBuf;

fn main() {
    let sysroot = PathBuf::from(env::var("BELA_SYSROOT").unwrap_or("/".to_string()));
    if !sysroot.join("root").join("Bela").exists() {
        panic!("Set BELA_SYSROOT to the location of your extracted Bela image");
    }

    #[cfg(feature = "static")]
    {
        println!("cargo:rustc-link-lib=static=bela");
        println!("cargo:rustc-link-lib=stdc++");
    }
    #[cfg(not(feature = "static"))]
    {
        println!("cargo:rustc-link-lib=bela");
    }
    println!("cargo:rustc-link-lib=cobalt");
    println!("cargo:rustc-link-lib=prussdrv");
    println!(
        "cargo:rustc-link-search=all={}",
        sysroot
            .join("root")
            .join("Bela")
            .join("lib")
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search=all={}",
        sysroot
            .join("usr")
            .join("xenomai")
            .join("lib")
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search=all={}",
        sysroot
            .join("usr")
            .join("local")
            .join("lib")
            .to_str()
            .unwrap()
    );

    #[cfg(feature = "midi")]
    {
        #[cfg(feature = "static")]
        println!("cargo:rustc-link-lib=static=belaextra");
        #[cfg(not(feature = "static"))]
        println!("cargo:rustc-link-lib=belaextra");
    }

    #[cfg(feature = "trill")]
    {
        let trill_root = sysroot
            .join("root")
            .join("Bela")
            .join("libraries")
            .join("Trill");

        cc::Build::new()
            .cpp(true)
            .std("c++11")
            .flag("-Wno-narrowing") // added by warnings()
            .include(sysroot.join("root").join("Bela").join("include"))
            .files([
                trill_root.join("Trill.cpp"),
                trill_root.join("CentroidDetection.cpp"),
            ])
            .compile("trill");
        println!("cargo:rustc-link-lib=trill");
    }
}
