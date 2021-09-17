use std::time;

pub struct Buffer {
    
}

pub struct Entry<T> {
    pub value: T,
    pub timestamp: time::Duration,
} 