use crate::public::*;
use once_cell::sync::Lazy;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{Arc, Mutex},
    thread::spawn,
};

pub enum Bind {
    NormalBind(BindHandler),
    BlockBind(BlockBindHandler),
    BlockableBind(BlockableBindHandler),
}
pub enum BindWithI16 {
    NormalBind(BindHandlerI16),
    BlockBind(BlockBindHandlerI16),
    BlockableBind(BlockableBindHandlerI16),
}

pub type BindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockBindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockableBindHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;

pub type BindHandlerI16 = Arc<dyn Fn(i16) + Send + Sync + 'static>;
pub type BlockBindHandlerI16 = Arc<dyn Fn(i16) + Send + Sync + 'static>;
pub type BlockableBindHandlerI16 = Arc<dyn Fn(i16) -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseButtonBindMap = HashMap<MouseButton, Bind>;
pub type MouseWheelBindMap = HashMap<MouseWheel, BindWithI16>;

pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static MOUSE_BUTTON_BINDS: Lazy<Mutex<MouseButtonBindMap>> = Lazy::new(|| Mutex::new(MouseButtonBindMap::new()));
pub static MOUSE_WHEEL_BINDS: Lazy<Mutex<MouseWheelBindMap>> = Lazy::new(|| Mutex::new(MouseWheelBindMap::new()));
