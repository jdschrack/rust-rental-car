use crate::customer::Customer;

#[derive(Debug, Default)]
pub struct CustomerStore {
    customers: Vec<Customer>,
}

impl CustomerStore {
    pub fn all(&self) -> &[Customer] {
        &self.customers
    }

    pub fn next_id(&self) -> Option<u32> {
        self.customers
            .iter()
            .map(Customer::id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
    }

    pub fn has_email(&self, email: &str) -> bool {
        self.customers
            .iter()
            .any(|customer| customer.email() == email)
    }

    pub fn add(&mut self, customer: Customer) {
        self.customers.push(customer);
    }
}
