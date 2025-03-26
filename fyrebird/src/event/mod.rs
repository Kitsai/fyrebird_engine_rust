use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Mutex,
};

use specs::Component;
use specs::{VecStorage, WriteStorage};
use specs_derive::Component;

pub trait Event: Any + Send + Sync {}

impl<T: Any + Send + Sync> Event for T {}

pub type EventCallback = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventSystem {
    listeners: Mutex<HashMap<TypeId, Vec<EventCallback>>>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<E: Event>(&self, callback: impl Fn(&E) + Send + Sync + 'static) {
        let type_id = TypeId::of::<E>();

        let callback = Box::new(move |event: &dyn Any| {
            if let Some(e) = event.downcast_ref::<E>() {
                callback(e);
            }
        });

        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(type_id).or_default().push(callback);
    }

    pub fn dispatch<E: Event>(&self, event: E) {
        let type_id = TypeId::of::<E>();

        let listeners = self.listeners.lock().unwrap();

        if let Some(callbacks) = listeners.get(&type_id) {
            for callback in callbacks {
                callback(&event);
            }
        }
    }

    pub fn clear<E: Event>(&self) {
        let type_id = TypeId::of::<E>();

        let mut listeners = self.listeners.lock().unwrap();
        listeners.remove(&type_id);
    }

    pub fn clear_all(&self) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.clear();
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
        }
    }
}

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct EventQueue {
    pending_events: Mutex<Vec<Box<dyn Event>>>,
}

impl EventQueue {
    pub fn push<E: Event>(&self, event: E) {
        let mut pending = self.pending_events.lock().unwrap();
        pending.push(Box::new(event));
    }

    pub fn pop(&self) -> Option<Box<dyn Event>> {
        let mut pending = self.pending_events.lock().unwrap();
        pending.pop()
    }
}
