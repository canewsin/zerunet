use serde::Serialize;

#[derive(Serialize)]
pub struct OptionalLimitStats {
	pub limit: String,
	pub used: isize,
	pub free: isize,
}
