pub struct Customer {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl Customer {
    pub fn new(id: u64, name: String, email: String) -> Customer {
        Customer { id, name, email }
    }
}
