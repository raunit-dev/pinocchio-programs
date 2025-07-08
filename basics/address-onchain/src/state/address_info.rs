use bytemuck::{Pod, Zeroable};

// #[repr(C)] ensures that the struct has a C-like memory representation,
// guaranteeing the order of fields for safe serialization.
#[repr(C)]
// #[derive(...)] automatically implements several useful traits.
// - `Clone`, `Copy`: Allow the struct to be duplicated easily. `Copy` enables the `*self = data` assignment.
// - `Pod` (Plain Old Data): A bytemuck trait indicating this struct is a simple block of data,
//   allowing for safe, zero-copy conversions between the struct and a byte slice.
// - `Zeroable`: A bytemuck trait indicating that an all-zero byte pattern is a valid state for this struct.
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AddressInfo {
    // Fixed-size byte arrays are used for string-like data to ensure a predictable on-chain size.
    pub name: [u8; 50],
    pub house_number: u8,
    pub street: [u8; 50],
    pub city: [u8; 50],
}

impl AddressInfo {
    // `LEN` provides a single source of truth for the on-chain account data size.
    // `core::mem::size_of` calculates the struct's size at compile time.
    pub const LEN: usize = core::mem::size_of::<AddressInfo>();

    // This method updates the struct's fields from another instance.
    pub fn set_inner(&mut self, data: Self) -> Self {
        // Because `AddressInfo` implements the `Copy` trait, we can directly assign
        // one instance to another. This performs a bit-for-bit copy of the data.
        *self = data;
        *self
    }
}