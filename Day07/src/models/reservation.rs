use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationStatus {
    Confirmed,
    Cancelled,
}

impl ReservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "confirmed" => Some(Self::Confirmed),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservationError {
    InvalidDateRange,
    InvalidRate,
    PriceOverflow,
}

#[derive(Debug, Clone)]
pub struct Reservation {
    id: u32,
    vehicle_id: u32,
    customer_id: u32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total_cents: i64,
    status: ReservationStatus,
}

impl Reservation {
    pub fn calculate_total(
        daily_rate_cents: i64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<i64, ReservationError> {
        if daily_rate_cents <= 0 {
            return Err(ReservationError::InvalidRate);
        }

        let days = (end_date - start_date).num_days();
        if days <= 0 {
            return Err(ReservationError::InvalidDateRange);
        }

        daily_rate_cents
            .checked_mul(days)
            .ok_or(ReservationError::PriceOverflow)
    }

    pub fn new(
        id: u32,
        vehicle_id: u32,
        customer_id: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        daily_rate_cents: i64,
    ) -> Result<Self, ReservationError> {
        let total_cents = Self::calculate_total(daily_rate_cents, start_date, end_date)?;

        Ok(Self {
            id,
            vehicle_id,
            customer_id,
            start_date,
            end_date,
            total_cents,
            status: ReservationStatus::Confirmed,
        })
    }

    pub fn reconstitute(
        id: u32,
        vehicle_id: u32,
        customer_id: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        total_cents: i64,
        status: ReservationStatus,
    ) -> Self {
        Self {
            id,
            vehicle_id,
            customer_id,
            start_date,
            end_date,
            total_cents,
            status,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn vehicle_id(&self) -> u32 {
        self.vehicle_id
    }
    pub fn customer_id(&self) -> u32 {
        self.customer_id
    }
    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }
    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }
    pub fn total_cents(&self) -> i64 {
        self.total_cents
    }
    pub fn status(&self) -> ReservationStatus {
        self.status
    }

    pub fn overlaps(&self, start: NaiveDate, end: NaiveDate) -> bool {
        self.status == ReservationStatus::Confirmed
            && self.start_date < end
            && self.end_date > start
    }

    pub fn cancel(&mut self) {
        self.status = ReservationStatus::Cancelled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, day).expect("test date")
    }

    #[test]
    fn price_and_adjacent_intervals() {
        let booking = Reservation::new(1, 1, 1, date(1), date(4), 6500).expect("valid booking");
        assert_eq!(booking.total_cents(), 19500);
        assert!(!booking.overlaps(date(4), date(7)));
        assert!(booking.overlaps(date(3), date(5)));
    }

    #[test]
    fn invalid_ranges_rejected() {
        assert!(matches!(
            Reservation::new(1, 1, 1, date(4), date(4), 6500),
            Err(ReservationError::InvalidDateRange)
        ));
    }

    #[test]
    fn cancellation_removes_conflict() {
        let mut booking = Reservation::new(1, 1, 1, date(1), date(4), 6500).unwrap();
        booking.cancel();
        assert!(!booking.overlaps(date(2), date(3)));
    }
}
