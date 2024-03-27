extern crate adb_client;

pub mod config {
    use adb_client::{AdbTcpConnection, RustADBError};
    use std::net::Ipv4Addr;
    use std::fs::File;
    use std::path::Path;

    pub fn copy() -> Result<(), RustADBError> {
        let filename: &str = "rethink.rbk";
        let path: &str = "sdcard/Download";
        let mut connection = AdbTcpConnection::new(Ipv4Addr::from([127, 0, 0, 1]), 5037).unwrap();
        let mut input = File::open(Path::new(&filename)).unwrap();
        connection.send(None::<&str>, &mut input, &path)?;
        Ok(())
    }
}
