pub struct sIDGenerator {
    next_id: u128,
}

impl sIDGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn generate_id(&mut self) -> u128 {
        let next_id = self.next_id;

        self.next_id += 1;
        
        next_id
    }
}
