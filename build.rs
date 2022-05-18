fn main() {
    println!("cargo:rustc-link-search=native=lib");
    // $ORIGIN: origin path of bin
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}