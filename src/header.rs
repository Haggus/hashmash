use chrono::Local;
use rand::{thread_rng, Rng};
use base64::encode;

#[allow(dead_code)]
pub struct HashHeader {
    version: u8,
    bits: u8,
    date: String,
    resource: String,
    extension: Option<String>,
    pub random: String,
    pub counter: u32,
}

impl HashHeader {
    pub fn new() -> HashHeader {
        let mut rng = thread_rng();

        HashHeader {
            version: 1,
            bits: 20,
            date: Local::now().format("%y%m%d%I%M%S").to_string(),
            resource: String::from("test@example.com"),
            extension: None,
            random: rng.gen_ascii_chars().take(12).collect(),
            counter: 137,
        }
    }

    pub fn increase_counter(&mut self) {
        self.counter = self.counter + 1;
    }

    pub fn compile(&self) -> String {
        let encoded_random = encode(&self.random);
        let encoded_counter = encode(&format!("{}", &self.counter));

        format!("{}:{}:{}:{}:{}:{}:{}",
            self.version,
            self.bits,
            self.date,
            self.resource,
            "", // possible extension
            encoded_random,
            encoded_counter)
    }
}

#[cfg(test)]
mod tests {
    use super::header::HashHeader;

    #[test]
    fn create_new_header() {
        let header = HashHeader::new();

        assert_eq!(12, header.random.len());
    }
}
