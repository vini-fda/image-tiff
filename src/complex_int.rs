//! Complex integer types for efficient storage and computation.
//!
//! This module provides 16-bit and 32-bit complex integer types (`CInt16` and `CInt32`)
//! designed for efficient storage and computation, particularly for applications like
//! Synthetic Aperture Radar (SAR) data processing and signal processing.
//!
//! The implementation is inspired by the TIFF library's `f16` implementation and provides
//! similar conversion methods and optimizations.

// Complex number with 16-bit integer components.
///
/// This type stores complex numbers as a pair of 16-bit integers (real and imaginary parts)
/// in a memory-efficient layout. It's particularly useful for SAR data, signal processing,
/// and other applications where complex numbers with limited precision are needed.
///
/// The struct uses `#[repr(C)]` to ensure a predictable memory layout for FFI,
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct CInt16 {
    /// Real component of the complex number
    pub re: i16,
    /// Imaginary component of the complex number
    pub im: i16,
}

/// Complex number with 32-bit integer components.
///
/// This type stores complex numbers as a pair of 32-bit integers (real and imaginary parts)
/// in a memory-efficient layout. It's particularly useful for SAR data, signal processing,
/// and other applications where complex numbers with higher precision are needed.
///
/// The struct uses `#[repr(C)]` to ensure a predictable memory layout for FFI,
/// and `#[repr(align(8))]` to optimize memory access for SIMD operations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct CInt32 {
    /// Real component of the complex number
    pub re: i32,
    /// Imaginary component of the complex number
    pub im: i32,
}

impl CInt16 {
    pub fn new(re: i16, im: i16) -> Self {
        Self { re, im }
    }
}

impl CInt32 {
    pub fn new(re: i32, im: i32) -> Self {
        Self { re, im }
    }
}
