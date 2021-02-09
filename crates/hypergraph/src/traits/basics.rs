impl super::Weight for String {
    fn get_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        String::from_utf8_lossy(bytes).to_string()
    }
}
