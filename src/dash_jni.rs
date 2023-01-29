use jni::{
    objects::JClass,
    sys::{jlong},
    JNIEnv,
};
use once_cell::sync::OnceCell;
use crate::dash::Dash;
use crate::dash_settings::{DashSettings, EvictionPolicy};

static mut CACHE: OnceCell<Dash<i64, i64>> = OnceCell::new();

fn shared_cache() -> &'static mut Dash<i64, i64> {
    unsafe { CACHE.get_mut().expect("The cache is not initialized") }
}

#[no_mangle]
pub extern "system" fn Java_com_github_benmanes_caffeine_cache_simulator_policy_dash_DashRustPolicy_initCache(
    _env: JNIEnv,
    _class: JClass,
) {
    unsafe { CACHE.set(Dash::new(DashSettings {
        dash_size: 1,
        segment_size: 1,
        bucket_size: 50,
        eviction_policy: EvictionPolicy::LRU,
        debug_mode: 0,
    })).expect(""); }
}

#[no_mangle]
pub extern "system" fn Java_com_github_benmanes_caffeine_cache_simulator_policy_dash_DashRustPolicy_getFromCacheIfPresent(
    _env: JNIEnv,
    _class: JClass,
    key: jlong,
) -> jlong {
    let res = shared_cache().get(&key);
    match res {
        None => -1,
        Some(n) => n.clone()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_benmanes_caffeine_cache_simulator_policy_dash_DashRustPolicy_putToCache(
    _env: JNIEnv,
    _class: JClass,
    key: jlong,
    value: jlong,
) {
    shared_cache().put(key as i64, value as i64);
}