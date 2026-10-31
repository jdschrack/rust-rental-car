#[derive(Debug, PartialEq, Eq, Clone)]
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

    #[doc(hidden)]
    pub(crate) fn reconstitute(id: u32, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

#[cfg(test)]
mod customer_tests {
    use super::*;

    #[test]
    fn reject_invalid_emaill() {
        let result = Customer::new(1, "Foo Bar".into(), "".into());
        assert!(matches!(result, Err(CustomerError::InvalidEmail)))
    }

    #[test]
    fn reject_invalid_name() {
        let result = Customer::new(1, "".into(), "foo@bar.com".into());
        assert!(matches!(result, Err(CustomerError::EmptyName)))
    }

    #[test]
    fn valid_cuustomer_returns_expected_values() {
        let name: String = "foo bar".into();
        let email: String = "foo@bar.com".into();

        let result = match Customer::new(1, name.clone(), email.clone()) {
            Ok(result) => result,
            Err(error) => {
                panic!("Expected Ok, but got error: {:?}", error);
            }
        };

        assert!(result.email().eq(&email.to_string()));
        assert!(result.name().eq(&name.to_string()));
        assert!(result.id().eq(&1));
    }
}
