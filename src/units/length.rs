use super::*;

pub trait Length {}

pub struct Meter(Number);

impl Length for Meter {}

impl Unit for Meter {
	const SI_CONVERSION_RATIO: Number = 1.0;
	
	fn value(&self) -> Number {
		self.0
	}
}

unit_macro![length_units, Length, Meter];
length_units![Millimeter, 0.001, Decimeter, 0.1];
