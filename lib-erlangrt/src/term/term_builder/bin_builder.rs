use crate::{
  defs::sizes::ByteSize,
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{self, binary::trait_interface::TBinary},
    lterm::LTerm,
  },
};

pub struct BinaryBuilder {
  p: *mut TBinary,
  write_pos: *mut u8,
  limit: *mut u8,
  size: ByteSize, // used in debug only
}

impl BinaryBuilder {
  #[inline]
  pub fn with_size(size: ByteSize, hp: &mut Heap) -> RtResult<Self> {
    let p = unsafe { boxed::Binary::create_into(hp, size) }?;
    let write_pos = unsafe { (*p).get_data_mut() };
    Ok(Self {
      p,
      write_pos,
      limit: unsafe { write_pos.add(size.bytes()) },
      size,
    })
  }

  pub unsafe fn write_byte(&mut self, b: u8) {
    debug_assert!(
      self.write_pos < self.limit,
      "binary_builder: writing beyond {} bytes",
      self.size
    );
    core::ptr::write(self.write_pos, b);
    self.write_pos = self.write_pos.add(1);
  }

  pub fn make_term(self) -> LTerm {
    unsafe { (*self.p).make_term() }
  }
}