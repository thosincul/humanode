// DO NOT EDIT!
//! Autogenerated weights for `pallet_multisig`

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
  /// The range of component `z` is `[0, 10000]`.
  fn as_multi_threshold_1(_z: u32, ) -> Weight {
    // Minimum execution time: 14_000 nanoseconds.
    Weight::from_ref_time(16_000_000)
  }
  /// The range of component `z` is `[0, 10000]`.
  /// The range of component `s` is `[2, 128]`.
  fn as_multi_create(s: u32, z: u32, ) -> Weight {
    // Minimum execution time: 34_000 nanoseconds.
    Weight::from_ref_time(26_888_888)
      // Standard Error: 0
      .saturating_add(Weight::from_ref_time(55_555).saturating_mul(s.into()))
      // Standard Error: 0
      .saturating_add(Weight::from_ref_time(1_000).saturating_mul(z.into()))
      .saturating_add(T::DbWeight::get().reads(2))
      .saturating_add(T::DbWeight::get().writes(1))
  }
  /// The range of component `s` is `[3, 128]`.
  /// The range of component `z` is `[0, 10000]`.
  fn as_multi_approve(s: u32, z: u32, ) -> Weight {
    // Minimum execution time: 25_000 nanoseconds.
    Weight::from_ref_time(19_368_000)
      // Standard Error: 20_784
      .saturating_add(Weight::from_ref_time(44_000).saturating_mul(s.into()))
      // Standard Error: 259
      .saturating_add(Weight::from_ref_time(1_050).saturating_mul(z.into()))
      .saturating_add(T::DbWeight::get().reads(1))
      .saturating_add(T::DbWeight::get().writes(1))
  }
  /// The range of component `s` is `[2, 128]`.
  /// The range of component `z` is `[0, 10000]`.
  fn as_multi_complete(s: u32, _z: u32, ) -> Weight {
    // Minimum execution time: 39_000 nanoseconds.
    Weight::from_ref_time(47_349_206)
      // Standard Error: 48_112
      .saturating_add(Weight::from_ref_time(75_396).saturating_mul(s.into()))
      .saturating_add(T::DbWeight::get().reads(2))
      .saturating_add(T::DbWeight::get().writes(2))
  }
  /// The range of component `s` is `[2, 128]`.
  fn approve_as_multi_create(_s: u32, ) -> Weight {
    // Minimum execution time: 30_000 nanoseconds.
    Weight::from_ref_time(32_000_000)
      .saturating_add(T::DbWeight::get().reads(2))
      .saturating_add(T::DbWeight::get().writes(1))
  }
  /// The range of component `s` is `[2, 128]`.
  fn approve_as_multi_approve(_s: u32, ) -> Weight {
    // Minimum execution time: 20_000 nanoseconds.
    Weight::from_ref_time(24_000_000)
      .saturating_add(T::DbWeight::get().reads(1))
      .saturating_add(T::DbWeight::get().writes(1))
  }
  /// The range of component `s` is `[2, 128]`.
  fn cancel_as_multi(_s: u32, ) -> Weight {
    // Minimum execution time: 25_000 nanoseconds.
    Weight::from_ref_time(35_000_000)
      .saturating_add(T::DbWeight::get().reads(1))
      .saturating_add(T::DbWeight::get().writes(1))
  }
}
