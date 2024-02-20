struct KeyNoncePair {
    key: [u8; 32],
    nonce: [u8; 12],
    path: String,
}

impl KeyNoncePair {
    fn from_file(path: String) -> Result<Self> {

    }
}

