#[derive(Debug, Clone)]
pub enum EvictionPolicy {
	/** Every cache hit the item will move to the beginning of the data structure */
	ClassicLRU,
	/** LRU will be checked based of timestamp of the last hit and items won't move */
	TimestampLRU,
	Lifo,
	Lfu,
	Fifo,
}
