#!/usr/bin/env python3
# takes: atoms.tab
# Prints a Rust source file predefined bif names/arities (also they are
# preregistered by emulator::atom during the startup)
import erlangrt.genop as genop


def main():
    conf = genop.OTP20()
    tables = genop.OTPTables(conf)

    print("""\
//! Generated by codegen/create_gen_bif.py
//! Creates a lookup table of BIF functions
//! Config used: {otp} 
#![allow(dead_code)]

use defs::Arity;
use emulator::funarity::FunArity;
use emulator::gen_atoms;
use term::immediate;
use term::lterm::LTerm;
use bif;


type BifTabItem = (LTerm, LTerm, Arity, bif::BifFn);

pub static BIF_TABLE: &'static [BifTabItem] = &[
""".format(otp=conf.__class__.__name__))

    for bif in tables.bif_tab:
        print("    (gen_atoms::{mod}, "
              "gen_atoms::{fun}, {arity}, "
              "bif::{biftype}_{fun_name}_{arity}),"
              "".format(cname=bif.cname,
                        mod=genop.enum_name(bif.mod).upper(),
                        fun=genop.enum_name(bif.cname).upper(),
                        fun_name=genop.c_fun_name(bif.cname),
                        biftype=bif.biftype,
                        arity=bif.arity))

    print("];\n")


if __name__ == "__main__":
    main()
