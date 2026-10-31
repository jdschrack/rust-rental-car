use crate::vehicle::Vehicle;

pub enum FleetError {
    VehicleNotFound,
}

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

    pub fn find(&self, id: u32) -> Option<&Vehicle> {
        self.vehicles.iter().find(|vehicle| vehicle.id() == id)
    }

    pub fn require(&self, id: u32) -> Result<&Vehicle, FleetError> {
        self.find(id).ok_or(FleetError::VehicleNotFound)
    }

    pub fn available(&self) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|vehicle| vehicle.is_rentable())
            .collect()
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Vehicle> {
        self.vehicles.iter_mut().find(|vehicle| vehicle.id() == id)
    }

    pub fn remove(&mut self, id: u32) -> Option<Vehicle> {
        let index = self
            .vehicles
            .iter()
            .position(|vehicle| vehicle.id() == id)?;
        Some(self.vehicles.remove(index)) //this is ok in rust since it won't panic
    }
}
