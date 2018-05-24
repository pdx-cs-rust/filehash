// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate crc;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let fname = std::env::args().nth(1).unwrap();
    let mut f = std::fs::File::open(fname)?;
    let mut fbytes = Vec::new();
    f.read_to_end(&mut fbytes)?;
    let csum = crc::crc32::checksum_ieee(&fbytes);
    println!("{:08x}", csum);
    Ok(())
}
