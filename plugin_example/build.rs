fn main() {
    // $ORIGIN: origin path of bin
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}