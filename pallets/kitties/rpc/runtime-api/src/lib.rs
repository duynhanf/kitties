#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait KittyApi
	{
		fn query_kitty_info() -> u32;
		fn get_kitty_count() -> u64;
	}
}
