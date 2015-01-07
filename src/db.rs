pub fn f64_to_db(gain: f64) -> Option<f64> {
	// decibels = 20 * log10(gain)
	if gain > 0f64 {
		Some(20f64 * ::std::num::Float::log10(gain))
	}
	else {
		None
	}
}
pub fn db_to_f64(dbgain: f64) -> f64 {
	// gain = 10^(decibels / 20)
	::std::num::Float::powf(10f64, dbgain / 20f64)
}

mod tests {
	use super::*;
	use std::f64::EPSILON;

	#[test]
	fn test_f64_to_db() {
		let x1 = f64_to_db(0.5f64).expect("Failed conversion");
		assert!(-6.0206f64 - x1 < EPSILON);

		let x2 = f64_to_db(1.5f64).expect("Failed conversion");
		assert!(3.521825f64 - x2 < EPSILON);
		
		assert_eq!(Some(0f64), f64_to_db(1f64));

		assert_eq!(None, f64_to_db(0f64));
		assert_eq!(None, f64_to_db(-0.5f64));		
	}

	#[test]
	fn test_db_to_f64() {
		let x1 = db_to_f64(-6f64);
		assert!(0.501187f64 - x1 < EPSILON);

		let x2 = db_to_f64(6f64);
		assert!(1.99526f64 - x2 < EPSILON);

		assert_eq!(1f64, db_to_f64(0f64));
	}
}