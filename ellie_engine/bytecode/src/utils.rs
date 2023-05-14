use alloc::vec::Vec;
use core::mem;
use ellie_core::defs::PlatformArchitecture;

pub struct RawType {
    pub size: usize,
    pub data: isize,
}

/// Usize to platform le bytes
/// Even this is a 64 bit system, if ach is B32 it will return 4 bytes
/// # Params
/// * `integer` - usize to convert
/// * `arch` - [`PlatformArchitecture`] to convert
pub fn usize_to_le_bytes(integer: usize, arch: PlatformArchitecture) -> Vec<u8> {
    let mut bytes = Vec::new();
    let int_bytes = integer.to_le_bytes();
    for i in 0..arch.usize_len() as usize {
        bytes.push(int_bytes[i]);
    }
    bytes
}

/// Isize to platform le bytes
/// Even this is a 64 bit system, if ach is B32 it will return 4 bytes
/// # Params
/// * `integer` - isize to convert
/// * `arch` - [`PlatformArchitecture`] to convert
pub fn isize_to_le_bytes(integer: isize, arch: PlatformArchitecture) -> Vec<u8> {
    let mut bytes = Vec::new();
    let int_bytes = integer.to_le_bytes();
    for i in 0..arch.usize_len() as usize {
        bytes.push(int_bytes[i]);
    }
    bytes
}

pub fn f32_to_le_bytes(float: f32, arch: PlatformArchitecture) -> Vec<u8> {
    let mut bytes = Vec::new();
    let int_bytes = float.to_le_bytes();
    for i in 0..arch.usize_len() as usize {
        bytes.push(int_bytes[i]);
    }
    bytes
}

pub fn f64_to_le_bytes(float: f64, arch: PlatformArchitecture) -> Vec<u8> {
    let mut bytes = Vec::new();
    let int_bytes = float.to_le_bytes();
    for i in 0..arch.usize_len() as usize {
        bytes.push(int_bytes[i]);
    }
    bytes
}

pub fn limit_platform_size(integer: usize, arch: PlatformArchitecture) -> usize {
    let mut bytes = [0_u8; 8];
    let int_bytes = integer.to_le_bytes();
    bytes[0..arch.usize_len() as usize].copy_from_slice(&int_bytes[..arch.usize_len() as usize]);
    usize::from_le_bytes(bytes[0..mem::size_of::<usize>()].try_into().unwrap())
}
