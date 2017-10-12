#!/usr/bin/env python3
# takes: genop.tab from erlang/otp
# Prints a Rust source file with array which maps erlang asm opcodes to numbers
import erlangrt.genop as genop


def main():
    conf = genop.OTP20()
    tables = genop.OTPTables(conf)

    print("""\
//! Generated by `codegen/create_gen_op.py`
//! Maps genop table from Erlang/OTP source to Rust
//! Config used: {otp}
#![allow(dead_code)]

use defs::Word;
use emulator::code::opcode::RawOpcode;


pub const OPCODE_MAX: RawOpcode = {op_max};
""".format(op_max=conf.max_opcode, otp=conf.__class__.__name__))

    # print arity map
    print("pub static ARITY_MAP: &'static [RawOpcode] = &[\n"
          "    0, // opcode 0 does not exist")
    for opcode in range(conf.min_opcode, conf.max_opcode + 1):
        op = tables.ops[opcode]
        print("    %d, // opcode: %d (%s)" % (op.arity, opcode, op.name))
    print("""\
];

pub fn opcode_arity(opcode: RawOpcode) -> u8 {
  ARITY_MAP[opcode as usize]
}
""")

    #
    # ------ print opcode names map ------
    #

    # print("#[cfg(debug)]")
    print("""const OPCODE_NAME_MAP: &'static [&'static str] = &[
        \"\", // opcode 0 does not exist""")
    for opcode in range(conf.min_opcode, conf.max_opcode + 1):
        op = tables.ops[opcode]
        print("    \"%s\", // opcode: %d" % (op.name, opcode))
    print("""\
];

pub fn opcode_name(opcode: RawOpcode) -> &'static str {
  OPCODE_NAME_MAP[opcode as Word]
}
""")

    #
    # ------ print opcode enum ------
    #

    for opcode in range(conf.min_opcode, conf.max_opcode + 1):
        op = tables.ops[opcode]
        print("pub const OPCODE_%s: RawOpcode = %d;"
              % (op.name.upper(), opcode))
    print("\n")


if __name__ == "__main__":
    main()
