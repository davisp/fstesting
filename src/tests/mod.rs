#![cfg(test)]

pub mod directories;
pub mod file_open_close;
pub mod file_other;
pub mod file_pread;
pub mod file_pwrite;
pub mod file_read;
pub mod file_write;
pub mod properties;
pub mod special;

const DATA_SIZE: usize = 1024 * 1024 * 15;
