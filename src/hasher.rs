use crypto::digest::Digest;
use crypto::sha1::Sha1;

use header::HashHeader;

pub struct Hasher {
    pub header: HashHeader,
}

impl Hasher {
    pub fn new(header: HashHeader) -> Hasher {
        Hasher {
            header: header,
        }
    }

    pub fn next(&mut self) -> [u8; 20] {
        let hash = self.hash();

        self.header.increase_counter();
        hash
    }

    pub fn hash(&self) -> [u8; 20] {
        let mut sha = Sha1::new();
        let compiled_header = self.header.compile();

        sha.input_str(&compiled_header);

        let mut hex_array: [u8; 20] = [0; 20];
        sha.result(&mut hex_array);
        hex_array
    }
}

