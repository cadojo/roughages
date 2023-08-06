

pub struct DID {
    method: String,
    identifier: String,
}

impl DID {
    fn validate(&self) -> bool {
        return !self.method.is_empty()
            && !self.identifier.is_empty()
            && self
                .method
                .chars()
                .chain(self.identifier.chars())
                .all(|c: char| c.is_lowercase());
    }
    fn uri(&self) -> String {
        format!("did:{}:{}", self.method, self.identifier)
    }
}
