use std::fs;

fn main() {
    let path =
        "simplelink-lowpower-f2-sdk/source/ti/devices/cc13x2_cc26x2/driverlib/bin/ticlang/driverlib.lib";
    // Note that lld (default rust linker) will assume .a as exetension and prefix with lib. So the format is lib{name}.a
    // ref: https://releases.llvm.org/2.5/docs/CommandGuide/html/llvm-ld.html
    // Give the mule what he wants
    let copy_path = "target/libdriverlib.a";
    // Copy the library to the target directory so as to not dirty the submodule
    fs::copy(path, copy_path).expect("Didn't find driverlib, have you run `git submodule add https://github.com/TexasInstruments/simplelink-lowpower-f2-sdk.git`?");

    // Tell cargo to tell rustc to link the driverlib library.
    println!("cargo:rustc-link-search=target");
    println!("cargo:rustc-link-lib=static=driverlib");
}
