use core::mem::MaybeUninit;

use crate::Error;

static mut PRNG_VAL: u64 = 1984;

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    unsafe {
        for chunk in dest.chunks_mut(8) {
            let val = PRNG_VAL;
            PRNG_VAL = val.wrapping_mul(0x5DEECE66D);
            let src = (&val as *const u64).cast();
            chunk.copy_from_slice(core::slice::from_raw_parts(src, chunk.len()));
        }
    }
    Ok(())
}
