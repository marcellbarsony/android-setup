mod adb;
mod rethink;

fn main() {
    adb::setup::settings();
    adb::debloat::uninstall();
    adb::debloat::disable();
    // let result = rethink::config::copy();
    // println!("{:?}", result);
}
