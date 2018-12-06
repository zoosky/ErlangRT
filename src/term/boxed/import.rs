use bif::{find_bif, BifFn};
use emulator::code::pointer::CodePtr;
use emulator::code_srv::CodeServer;
use emulator::heap::Heap;
use emulator::mfa::MFArity;
use fail::Error;
use fail::RtResult;
use rt_defs::storage_bytes_to_words;
use term::boxed::{BoxHeader, BOXTYPETAG_IMPORT};
use term::lterm::*;

use core::ptr;
use std::mem::size_of;

#[allow(dead_code)]
pub struct Import {
  header: BoxHeader,
  pub mfarity: MFArity,
  pub is_bif: bool,
}

impl Import {
  const fn storage_size() -> usize {
    storage_bytes_to_words(size_of::<Import>())
  }

  pub unsafe fn create_into(hp: &mut Heap, mfarity: MFArity, is_bif: bool) -> RtResult<LTerm> {
    let n_words = Import::storage_size();
    let this = hp.alloc::<Import>(n_words, false)?;

    ptr::write(
      this,
      Import {
        header: BoxHeader::new(BOXTYPETAG_IMPORT, n_words),
        mfarity,
        is_bif,
      },
    );
    Ok(LTerm::make_boxed(this))
  }


  pub unsafe fn const_from_term(t: LTerm) -> RtResult<*const Import> {
    helper_get_const_from_boxed_term::<Import>(t, BOXTYPETAG_IMPORT, Error::BoxedIsNotAnImport)
  }


  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: LTerm) -> RtResult<*mut Import> {
    helper_get_mut_from_boxed_term::<Import>(t, BOXTYPETAG_IMPORT, Error::BoxedIsNotAnImport)
  }


  /// Lookup a function, referred by this object and possibly attempt code
  /// loading if the module was missing. Return a code pointer.
  pub fn resolve(&self, code_server: &mut CodeServer) -> RtResult<CodePtr> {
    code_server.lookup_and_load(&self.mfarity)
  }


  /// Assuming that this object refers to a BIF function, perform a BIF lookup.
  pub fn resolve_bif(&self) -> RtResult<BifFn> {
    find_bif(&self.mfarity)
  }
}
