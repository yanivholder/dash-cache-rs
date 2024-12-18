use crate::eviction_policy::EvictionPolicy;

#[derive(Debug, Clone)]
pub struct DashSettings {
	pub num_of_segments: usize,
	pub num_of_normal_buckets: usize,
	pub num_of_stash_buckets: usize,
	pub bucket_size: usize,
	pub eviction_policy: EvictionPolicy,
	pub debug_mode: usize,
}

impl Default for DashSettings {
	fn default() -> Self {
		DashSettings {
			num_of_segments: 1,
			num_of_normal_buckets: 28,
			num_of_stash_buckets: 4,
			bucket_size: 16,
			eviction_policy: EvictionPolicy::ClassicLRU,
			debug_mode: 1,
		}
	}
}

#[allow(dead_code)] // Remove this after stabilizing hit ratio
pub const DRAGON_SETTINGS: DashSettings = DashSettings {
	num_of_segments: 1,
	num_of_normal_buckets: 56,
	num_of_stash_buckets: 4,
	bucket_size: 14,
	eviction_policy: EvictionPolicy::ClassicLRU,
	debug_mode: 1,
};
