use bytemuck::{Pod, Zeroable};



#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; 50],
    pub house_number: u8,
    pub street: [u8; 50],
    pub city: [u8; 50],
}

impl AddressInfo {
    pub const LEN: usize = core::mem::size_of::<AddressInfo>();

    pub fn set_inner(&mut self, data: Self) -> Self {
        self.name = data.name;
        self.house_number = data.house_number;
        self.street = data.street;
        self.city = data.city;
        *self
    }
}

