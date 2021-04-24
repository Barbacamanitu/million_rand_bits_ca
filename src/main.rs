mod bit_grid;

extern crate compression;
extern crate bv;
use bv::{BitSliceable, BitVec, Bits, BitsMut};


fn main() {

 

    
    let data:&'static [u8]= include_bytes!("..//data/AMillionRandomDigits.bin");
    let data_bv = bit_grid::BitGrid::from_data(data);
    println!("Grid Size: {}",data_bv.size);
    println!("20 Bits: {:?}", data_bv.bit_vec.bit_slice(0..20));
    println!("Initial size: {:?}", data_bv.starting_size);

}
