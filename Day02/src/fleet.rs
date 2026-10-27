use crate::vehicle::Vehicle;

#[derive(Default, Debug)]
pub struct Fleet {
    pub vehicles: Vec<Vehicle>,
}

impl Fleet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    pub fn all(&self) -> &[Vehicle] {
        &self.vehicles
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Vehicle> {
        self.vehicles.iter().find(|vehicle| vehicle.id() == id)
    }

    pub fn available(&self) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|vehicle| vehicle.is_rentable())
            .collect()
    }
}
