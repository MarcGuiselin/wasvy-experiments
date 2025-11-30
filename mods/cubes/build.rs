fn main() {
    // Tell Cargo to rebuild if the wit file changed
    println!("cargo:rerun-if-changed=wit/world.wit");
}
