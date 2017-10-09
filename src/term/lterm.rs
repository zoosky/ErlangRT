//!
//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use term::immediate;
use term::immediate::{IMM2_SPECIAL_NIL_RAW, IMM2_SPECIAL_NONVALUE_RAW};
use term::primary;
use term::primary::header::HeaderTag;
use term::raw::{RawCons, RawConsMut, RawTuple, RawTupleMut};
use emulator::atom;

use defs;
use defs::{Word, SWord, MAX_UNSIG_SMALL, MIN_SIG_SMALL, MAX_SIG_SMALL};
//type Word = defs::Word;

use std::cmp::Ordering;
use std::fmt;


/// A low-level term, packed conveniently in a Word, or containing a
/// pointer to heap.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LTerm {
  pub value: Word
}


impl Ord for LTerm {
  fn cmp(&self, other: &LTerm) -> Ordering {
    self.value.cmp(&other.value)
  }
}


impl PartialOrd for LTerm {
  fn partial_cmp(&self, other: &LTerm) -> Option<Ordering> {
    Some(self.value.cmp(&other.value))
  }
}


// TODO: Remove deadcode directive later and fix
#[allow(dead_code)]
impl LTerm {
  /// Access the raw Word value of the low-level term.
  #[inline(always)]
  pub fn raw(&self) -> Word { self.value }

  /// Create a NIL value.
  #[inline(always)]
  pub fn nil() -> LTerm { LTerm { value: IMM2_SPECIAL_NIL_RAW } }

  /// Check whether a value is a NIL \[ \]
  #[inline]
  pub fn is_nil(&self) -> bool {
    self.value == IMM2_SPECIAL_NIL_RAW
  }

  /// Create a NON_VALUE.
  #[inline(always)]
  pub fn non_value() -> LTerm {
    LTerm { value: IMM2_SPECIAL_NONVALUE_RAW }
  }

  /// Check whether a value is a NON_VALUE.
  #[inline]
  pub fn is_non_value(&self) -> bool {
    self.value == IMM2_SPECIAL_NONVALUE_RAW
  }

  /// Check whether a value is NOT a NON_VALUE.
  #[inline]
  pub fn is_value(&self) -> bool {
    ! self.is_non_value()
  }

  /// Check whether a value is a local pid.
  #[inline]
  pub fn is_local_pid(&self) -> bool {
    return immediate::is_pid_raw(self.value)
  }

  /// Get primary tag bits from a raw term
  #[inline(always)]
  pub fn primary_tag(&self) -> primary::Tag {
    primary::get_tag(self.value)
  }

  /// Check whether primary tag of a value is `Tag::Immediate`.
  #[inline(always)]
  pub fn is_imm(&self) -> bool {
    self.primary_tag() == primary::Tag::Immediate
  }

  /// Check whether primary tag of a value is `Tag::Box`.
  #[inline(always)]
  pub fn is_box(&self) -> bool {
    self.primary_tag() == primary::Tag::Box
  }

  /// Check whether primary tag of a value is `Tag::Cons`.
  #[inline(always)]
  pub fn is_cons(&self) -> bool {
    self.primary_tag() == primary::Tag::Cons
  }

  /// Check whether primary tag of a value is `Tag::Header`.
  #[inline(always)]
  pub fn is_header(&self) -> bool {
    self.primary_tag() == primary::Tag::Header
  }

  /// Retrieve the raw value of a `LTerm`.
  #[inline(always)]
  pub fn get_raw(&self) -> Word { self.value }

  //
  // Construction
  //

  /// Any raw word becomes a term, possibly invalid
  #[inline]
  pub fn from_raw(w: Word) -> LTerm {
    LTerm { value: w }
  }

  /// From internal process index create a pid. To create a process use vm::create_process
  #[inline]
  pub fn make_pid(pindex: Word) -> LTerm {
    LTerm { value: immediate::make_pid_raw(pindex) }
  }

  #[inline]
  pub fn make_xreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_xreg_raw(n) }
  }

  #[inline]
  pub fn make_yreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_yreg_raw(n) }
  }

  #[inline]
  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_fpreg_raw(n) }
  }

  #[inline]
  pub fn make_label(n: Word) -> LTerm {
    LTerm { value: immediate::make_label_raw(n) }
  }

  /// From a pointer to heap create a generic box
  #[inline]
  pub fn make_box(ptr: *const Word) -> LTerm {
    LTerm { value: primary::make_box_raw(ptr) }
  }

  //
  // Cons, lists, list cells, heads, tails
  //

  /// From a pointer to heap create a cons box
  #[inline]
  pub fn make_cons(ptr: *const Word) -> LTerm {
    LTerm { value: primary::make_cons_raw(ptr) }
  }


  /// Get a proxy object for read-only accesing the cons contents.
  pub unsafe fn raw_cons(&self) -> RawCons {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::Tag::Cons);
    let boxp = primary::pointer(v);
    RawCons::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  pub unsafe fn raw_cons_mut(&self) -> RawConsMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::Tag::Cons);
    let boxp = primary::pointer_mut(v);
    RawConsMut::from_pointer(boxp)
  }

  //
  // Atom services - creation, checking
  //

  /// From atom index create an atom. To create from string use vm::new_atom
  #[inline]
  pub fn make_atom(index: Word) -> LTerm {
    LTerm { value: immediate::make_atom_raw(index) }
  }


  /// Check whether a value is a runtime atom.
  #[inline]
  pub fn is_atom(&self) -> bool {
    return immediate::is_atom_raw(self.value)
  }


  /// For an atom value, get index.
  pub fn atom_index(&self) -> Word {
    assert!(self.is_atom());
    immediate::imm2_value(self.value)
  }

  //
  // Box services - boxing, unboxing, checking
  //

  #[inline]
  pub fn box_ptr(&self) -> *const Word {
    primary::pointer(self.value)
  }

  //
  // Small integer handling
  //

  /// Check whether a value is a small integer.
  #[inline]
  pub fn is_small(&self) -> bool {
    return immediate::is_small_raw(self.value)
  }


  #[inline]
  pub fn make_small_u(n: Word) -> LTerm {
    assert!(n < MAX_UNSIG_SMALL);
    LTerm { value: immediate::make_small_raw(n) }
  }


  #[inline]
  pub fn make_small_i(n: SWord) -> LTerm {
    // TODO: Do the proper min neg small
    assert!(n < MAX_SIG_SMALL && n > MIN_SIG_SMALL);
    let un = defs::unsafe_sword_to_word(n);
    LTerm { value: immediate::make_small_raw(un) }
  }


  #[inline]
  pub fn small_get(&self) -> SWord {
    let n = immediate::imm1_value(self.value);
    return defs::unsafe_word_to_sword(n);
  }

  //
  // Headers, tuples etc, boxed stuff on heap and special stuff in code
  //

  #[inline]
  pub fn make_tuple_header(arity: Word) -> LTerm {
    LTerm { value: primary::header::make_tuple_header_raw(arity) }
  }


  pub fn header_arity(&self) -> Word {
    assert!(self.is_header());
    primary::get_value(self.value)
  }


  /// Get a proxy object for read-only accesing the cons contents.
  pub unsafe fn raw_tuple(&self) -> RawTuple {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::Tag::Header);
    assert_eq!(primary::header::get_header_tag(v), HeaderTag::Tuple);
    let boxp = primary::pointer(v);
    RawTuple::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  pub unsafe fn raw_tuple_mut(&self) -> RawTupleMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::Tag::Header);
    assert_eq!(primary::header::get_header_tag(v), HeaderTag::Tuple);
    let boxp = primary::pointer_mut(v);
    RawTupleMut::from_pointer(boxp)
  }

}


// Printing low_level Terms as "{}"
impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = self.value;

    match primary::get_tag(v) {
      primary::Tag::Box => write!(f, "Box({:?})", self.box_ptr()),

      primary::Tag::Cons => unsafe {
        let raw_cons = self.raw_cons();
        unsafe { write!(f, "Cons@{:?}=[{} | {}]",
                        raw_cons.raw_pointer(), raw_cons.hd(), raw_cons.tl()) }
      },

      primary::Tag::Immediate =>
        match immediate::get_imm1_tag(v) {
          immediate::Immediate1::Small =>
            write!(f, "{}", self.small_get()),

          immediate::Immediate1::Pid =>
            write!(f, "Pid({})", immediate::imm1_value(v)),

          immediate::Immediate1::Port =>
            write!(f, "Port({})", immediate::imm1_value(v)),

          immediate::Immediate1::Immed2 =>

            match immediate::get_imm2_tag(v) {
              immediate::Immediate2::Catch =>
                write!(f, "Catch({})", immediate::imm2_value(v)),

              immediate::Immediate2::Special =>
                write!(f, "Special({})", immediate::imm2_value(v)),

              immediate::Immediate2::Atom =>
                write!(f, "'{}'", atom::to_str(*self)),

              immediate::Immediate2::Immed3 =>

                match immediate::get_imm3_tag(v) {
                  immediate::Immediate3::XReg =>
                    write!(f, "X({})", immediate::imm3_value(v)),

                  immediate::Immediate3::YReg =>
                    write!(f, "Y({})", immediate::imm3_value(v)),

                  immediate::Immediate3::FPReg =>
                    write!(f, "FP({})", immediate::imm3_value(v)),

                  immediate::Immediate3::Label =>
                    write!(f, "Label(0x{:04x})", immediate::imm3_value(v))
                }
            },
        },
      primary::Tag::Header => {
        let hptr = primary::pointer(v);
        let h = unsafe { *hptr };

        match primary::header::get_header_tag(h) {
          HeaderTag::Tuple => {
            let raw_tuple = RawTuple::from_pointer(hptr);
            write!(f, "{{").unwrap();
            let arity = unsafe { raw_tuple.arity() };
            for i in 0..arity {
              let item = primary::get_value(v);
            }
            write!(f, "}}")
          },
          HeaderTag::BigNegative => write!(f, "BigNeg"),
          HeaderTag::BigPositive => write!(f, "BigPos"),
          HeaderTag::Reference => write!(f, "Ref"),
          HeaderTag::Fun => write!(f, "Fun"),
          HeaderTag::Float => write!(f, "Float"),
          HeaderTag::Export => write!(f, "Export"),
          HeaderTag::RefcBinary => write!(f, "RefcBin"),
          HeaderTag::HeapBinary => write!(f, "HeapBin"),
          HeaderTag::SubBinary => write!(f, "SubBin"),
          HeaderTag::ExternalPid => write!(f, "ExtPid"),
          HeaderTag::ExternalPort => write!(f, "ExtPort"),
          HeaderTag::ExternalRef => write!(f, "ExtRef"),
          _ => write!(f, "Header({})", primary::get_value(v))
        }
      },
    }
  }
}
