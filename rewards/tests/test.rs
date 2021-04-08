use sp_arithmetic::{PerThing, Perbill, PerU16, Percent, Perquintill};

/// This test the precision and panics if error too big error.
///
/// error is asserted to be less or equal to 8/accuracy or 8*f64::EPSILON
fn test_precision<P: PerThing>(stake: P, ideal_stake: P, falloff: P) {
	let accuracy_f64 = Into::<u128>::into(P::ACCURACY) as f64;
	let res = pallet_staking_reward_fn::compute_inflation(stake, ideal_stake, falloff);
	let res = Into::<u128>::into(res.deconstruct()) as f64 / accuracy_f64;

	let expect = float_i_npos(stake, ideal_stake, falloff);

	let error = (res - expect).abs();

	if error > 8f64 / accuracy_f64 && error > 8.0 * f64::EPSILON {
		panic!(
			"stake: {:?}, ideal_stake: {:?}, falloff: {:?}, res: {}, expect: {}",
			stake, ideal_stake, falloff, res , expect
		);
	}
}

/// compute the inflation using floats
fn float_i_npos<P: PerThing>(stake: P, ideal_stake: P, falloff: P) -> f64 {
	let accuracy_f64 = Into::<u128>::into(P::ACCURACY) as f64;

	let ideal_stake = Into::<u128>::into(ideal_stake.deconstruct()) as f64 / accuracy_f64;
	let stake = Into::<u128>::into(stake.deconstruct()) as f64 / accuracy_f64;
	let falloff = Into::<u128>::into(falloff.deconstruct()) as f64 / accuracy_f64;

	let x_ideal = ideal_stake;
	let x = stake;
	let d = falloff;

	if x < x_ideal {
		x / x_ideal
	} else {
		2_f64.powf((x_ideal - x) / d)
	}
}

#[test]
fn test_precision_for_minimum_falloff() {
	fn test_falloff_precision_for_minimum_falloff<P: PerThing>() {
		for stake in 0..1_000 {
			let stake = P::from_rational(stake, 1_000);
			let ideal_stake = P::zero();
			let falloff = P::from_rational(1, 100);
			test_precision(stake, ideal_stake, falloff);
		}
	}

	test_falloff_precision_for_minimum_falloff::<Perquintill>();

	test_falloff_precision_for_minimum_falloff::<PerU16>();

	test_falloff_precision_for_minimum_falloff::<Perbill>();

	test_falloff_precision_for_minimum_falloff::<Percent>();
}

#[test]
fn compute_inflation_works() {
	fn compute_inflation_works<P: PerThing>() {
		for stake in 0..100 {
			for ideal_stake in 0..10 {
				for falloff in 1..10 {
					let stake = P::from_rational(stake, 100);
					let ideal_stake = P::from_rational(ideal_stake, 10);
					let falloff = P::from_rational(falloff, 100);
					test_precision(stake, ideal_stake, falloff);
				}
			}
		}
	}

	compute_inflation_works::<Perquintill>();

	compute_inflation_works::<PerU16>();

	compute_inflation_works::<Perbill>();

	compute_inflation_works::<Percent>();
}