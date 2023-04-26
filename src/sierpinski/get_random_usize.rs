/// Get a random `usize` value.
use uefi::proto::rng::Rng;
use core::mem;

pub fn get_random_usize(rng: &mut Rng) -> usize {
    let mut buf: [u8] = [ 0; mem::size_of::<usize>()];
    rng.get_rng(None, &mut buf).expect("get_rng failed");
    usize::from_le_bytes(buf)
}