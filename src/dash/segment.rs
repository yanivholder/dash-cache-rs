use std::hash::Hash;
use crate::dash::bucket::Bucket;
use crate::dash::data::Data;
use crate::dash::utils::{get_index, hash};
use crate::dash_settings::DashSettings;


#[derive(Debug)]
pub struct Segment<K, V>
where
    K: Hash + Eq + Clone,
    V: Eq + Clone,
{
    pub buckets: Vec<Bucket<K, V>>,
    pub segment_size: usize,
    pub stash_buckets: Vec<Bucket<K, V>>,
    pub stash_size: usize,
    // TODO: make this a reference with a lifetime
    settings: DashSettings
}

impl<K, V> Segment<K, V>
where
    K: Hash + Eq + Clone,
    V: Eq + Clone
{
    pub fn new(size: usize, bucket_size: usize, settings: DashSettings) -> Self {
        let mut buckets: Vec<Bucket<K, V>> = Vec::new();
        for _ in 0..size {
            // TODO: pass the settings as a reference
            buckets.push(Bucket::new(bucket_size, settings.clone()));
        }

        let mut stash_buckets: Vec<Bucket<K, V>> = Vec::new();
        for _ in 0..settings.stash_size {
            // TODO: pass the settings as a reference
            stash_buckets.push(Bucket::new(bucket_size, settings.clone()));
        }
        Segment {
            buckets,
            stash_buckets,
            segment_size: size,
            stash_size: settings.stash_size,
            settings
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&Data<K, V>> {
        let hash = hash(&key);
        let stash_bucket = &mut self.stash_buckets[get_index(hash, self.stash_size)];
        
        // The order assumes that the data is more likely to be in the stash bucket, this
        // assumption should be tested
        match stash_bucket.get(&key) {
            Some(data) => Some(data),
            None => {
                let bucket = &mut self.buckets[get_index(hash, self.segment_size)];
                match bucket.get(&key) {
                    Some(data) => Some(data),
                    None => None
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        let hash = hash(&key);
        let stash_bucket: &mut Bucket<K, V> = &mut self.stash_buckets[get_index(hash, self.stash_size)];
        let bucket: &mut Bucket<K, V> = &mut self.buckets[get_index(hash, self.segment_size)];
        // The order assumes that the data is more likely to be in the stash bucket, this
        // assumption should be tested
        match stash_bucket.get(&key) {
            Some(data) => {
                if data.value != val {
                    // TODO: consider what to do if data.val != val
                }
                // In this scenario we are promoting the data from the stash bucket to the main bucket
                stash_bucket.remove(&key);
                bucket.insert(key, val);
            }
            None => {
                match bucket.get(&key) {
                    Some(data) => {
                        bucket.insert(key, val);
                        return
                    }
                    None => {
                        stash_bucket.insert(key, val);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dash_settings::{DEFAULT_SETTINGS, EvictionPolicy};
    use super::*;

    #[test]
    fn test_segment_length() {
        let segment: Segment<i32, i32> = Segment::new(10, 10, DEFAULT_SETTINGS);
        assert_eq!(segment.buckets.len(), 10);
    }
}





