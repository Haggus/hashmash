extern crate hashmash;
extern crate base64;

use hashmash::header::HashHeader;
use hashmash::hasher::Hasher;

fn main() {
    let header = HashHeader::new();
    let mut hasher = Hasher::new(header);

    loop {
        println!("{}", hasher.header.compile());
        let hash = hasher.next();

        if hash[0].leading_zeros() == 8 &&
           hash[1].leading_zeros() == 8 &&
           hash[2].leading_zeros() >= 4 {
            println!("====================");
            println!("Correct hash found!");
            println!("{}", hasher.header.compile());
            break;
        }
    }
}
