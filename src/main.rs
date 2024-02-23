mod adb;

fn main() {
    adb::setup::settings();
    adb::debloat::uninstall();
    adb::debloat::disable();
}
