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

        let name = name.trim().to_owned();
        let email = email.trim().to_lowercase();
        let valid = !email.contains(char::is_whitespace)
            && matches!(email.split_once('@'),
                Some((local, domain)) if !local.is_empty() && !domain.is_empty() && !domain.contains('@'));
        if !valid {
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

    #[test]
    fn normalises_name_and_email() {
        let c = Customer::new(1, "  Foo Bar ".into(), " Foo@Bar.COM ".into()).expect("valid");
        assert_eq!(c.name(), "Foo Bar");
        assert_eq!(c.email(), "foo@bar.com");
    }

    #[test]
    fn rejects_email_without_at() {
        for bad in ["x", "@bar.com", "foo@", "a@b@c", "foo bar@x.com"] {
            assert_eq!(
                Customer::new(1, "Foo".into(), bad.into()),
                Err(CustomerError::InvalidEmail),
                "{bad}"
            );
        }
    }
}
