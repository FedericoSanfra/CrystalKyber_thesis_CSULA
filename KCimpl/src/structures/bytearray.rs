//! ByteArray
//!
//! ByteArray used for exchange and encoding/decoding

use rand::prelude::*;
/// A struct representing an array of bytes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ByteArray {
    /// Array of bytes
    pub data: Vec<u8>,
}

impl ByteArray{
    //Generate empty byte array
    pub const fn new()->Self{
        Self{
            data: Vec::new(),
        }
    }
    //generate a byte array from a slice of bytes
    pub fn from_bytes(data: &[u8])-> Self{
        Self {
            data: data.to_vec(),
        }
    }

    //generate a byte array of size len filled with random values
    pub fn random_fill(len: usize) -> Self {
        let mut data=vec![0;len];
        let mut rng=rand::thread_rng();
        rng.fill_bytes(&mut data);
        //rng is the handle of the thread that fills the array
        Self { data }
    }

    //append 2 slices together in a byte array

    pub fn append(&self, other: &Self) -> Self {
        let mut data=Vec::with_capacity(self.data.len() + other.data.len());

        data.extend_from_slice(&self.data);
        data.extend_from_slice(&other.data);

        Self { data }
    }

    //append an array of bytearrays together
    pub fn concat(items: &[&Self]) -> Self {
        let len = items.iter().map(|slice| slice.data.len()).sum();
        let mut data= Vec::with_capacity(len);

        for item in items.iter() {
            data.extend_from_slice(&item.data)
        }
        Self { data }
    }

    //get the value of the bit at position pos
    pub fn get_bit(&self, pos: usize) -> bool {
        let (index, offset) = (pos/8, pos % 8);
        let mask = 1 << offset;

        !((self.data[index] & mask)==0) //different from zero means 1
    }

    //trim the bytearray from the first num bytes
    pub fn skip(&self, num: usize) -> Self{
        let data= if num < self.data.len() {
            Vec::from(&self.data[..num])
        } else {
            Vec::new()
        };
        Self { data }
    }

    //Split the bytearray at the position pos
    pub fn split_at(&self, pos: usize) -> (Self, Self){
        let (d1,d2) = self.data.split_at(pos);
        (Self { data: d1.to_vec() }, Self { data: d2.to_vec() })
    }

    //truncate the byte array to size len
    pub fn truncate_at(&self, len: usize) -> Self{
        let mut data= self.data.clone();
        data.truncate(len);
        Self { data }
    }


}