#[derive(Debug, Clone)]
pub struct Customer {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CustomerError {
    EmptyName,
    InvalidEmail,
}

impl Customer {
    pub fn new(id: u32, name: String, email: String) -> Result<Self, CustomerError> {
        if name.trim().is_empty() {
            return Err(CustomerError::EmptyName);
        }

        if email.trim().is_empty() {
            return Err(CustomerError::InvalidEmail);
        }

        Ok(Self { id, name, email })
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
}
