use validator::ValidationError;

#[derive(Debug, PartialEq)]
pub struct PositiveNumber(i64);

impl PositiveNumber {
    pub fn new(val: i64) -> Result<Self, ValidationError> {
        if val > 0 {
            Ok(PositiveNumber(val))
        } else {
            Err(ValidationError::new("must_be_positive"))
        }
    }
}

pub fn validate_positive_number(val: i64) -> Result<(), ValidationError> {
    PositiveNumber::new(val).map(|_| ())
}
