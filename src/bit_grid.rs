use bv::*;

#[derive(Clone)]
pub struct BitGrid {
    pub bit_vec: BitVec,
    pub size: u64,
    pub starting_size: u64,
    iteration: u64
}

impl BitGrid {
    pub fn from_data(data: &[u8]) -> BitGrid {
        let bv_a = bin_to_bitvec(data);
        let total_bit_len = bv_a.len();
        let bit_len_f: f64 = total_bit_len as  f64;
        let bit_l_sqrt = bit_len_f.sqrt().floor(); //Used to find approximate square size for the grid
        //add 1 to have enough room
        let grid_size : u64 = bit_l_sqrt as u64 + 1u64;
        //square it
        let grid_size_squared = grid_size * grid_size;
        //get difference in lengths
        let diff = grid_size_squared - total_bit_len;

        //Create new bitvec of false's to append to actual data
        let bv_b :BitVec = BitVec::new_fill(false, diff);
        let bv_c = bv_a.into_bit_concat(bv_b).to_bit_vec();
        BitGrid {
            bit_vec: bv_c,
            size: grid_size,
            starting_size: total_bit_len,
            iteration: 0,            
        }

    }
}


pub fn bin_to_bitvec(d: &[u8]) -> BitVec {
        let bv_size:u64 = d.len() as u64 * 8;
        let mut bv_data :BitVec = BitVec::new_fill(false, bv_size);
    
        for i in 0..d.len() {
            let t = d[i];
            let index = i as u64*8;
            bv_data.set(index,   (t & 0b00000001) > 0);
            bv_data.set(index+1, (t & 0b00000010) > 0);
            bv_data.set(index+2, (t & 0b00000100) > 0);
            bv_data.set(index+3, (t & 0b00001000) > 0);
            bv_data.set(index+4, (t & 0b00010000) > 0);
            bv_data.set(index+5, (t & 0b00100000) > 0);
            bv_data.set(index+6, (t & 0b01000000) > 0);
            bv_data.set(index+7, (t & 0b10000000) > 0);
         }
    
    
        bv_data
}
