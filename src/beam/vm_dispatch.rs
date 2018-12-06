//! Generated by `codegen/create_vm_dispatch.py`
//! Dispatch for all opcode types.
//! Config used: OTP20
#![allow(dead_code)]

use emulator::vm::VM;
use beam::gen_op;
use beam::opcodes::*;
use beam::disp_result::{DispatchResult};
use emulator::code::opcode::RawOpcode;
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use fail::{RtResult};


#[inline]
pub fn dispatch_op_inline(vm: &VM, op: RawOpcode, ctx: &mut Context, curr_p: &mut Process) -> RtResult<DispatchResult> {
  match op {
    gen_op::OPCODE_FUNC_INFO => return opcode_func_info(vm, ctx, curr_p),
    gen_op::OPCODE_CALL => return opcode_call(vm, ctx, curr_p),
    gen_op::OPCODE_CALL_ONLY => return opcode_call_only(vm, ctx, curr_p),
    gen_op::OPCODE_CALL_EXT => return opcode_call_ext(vm, ctx, curr_p),
    gen_op::OPCODE_BIF0 => return opcode_bif0(vm, ctx, curr_p),
    gen_op::OPCODE_BIF1 => return opcode_bif1(vm, ctx, curr_p),
    gen_op::OPCODE_BIF2 => return opcode_bif2(vm, ctx, curr_p),
    gen_op::OPCODE_ALLOCATE => return opcode_allocate(vm, ctx, curr_p),
    gen_op::OPCODE_ALLOCATE_ZERO => return opcode_allocate_zero(vm, ctx, curr_p),
    gen_op::OPCODE_TEST_HEAP => return opcode_test_heap(vm, ctx, curr_p),
    gen_op::OPCODE_DEALLOCATE => return opcode_deallocate(vm, ctx, curr_p),
    gen_op::OPCODE_RETURN => return opcode_return(vm, ctx, curr_p),
    gen_op::OPCODE_IS_LT => return opcode_is_lt(vm, ctx, curr_p),
    gen_op::OPCODE_IS_GE => return opcode_is_ge(vm, ctx, curr_p),
    gen_op::OPCODE_IS_EQ => return opcode_is_eq(vm, ctx, curr_p),
    gen_op::OPCODE_IS_EQ_EXACT => return opcode_is_eq_exact(vm, ctx, curr_p),
    gen_op::OPCODE_IS_NIL => return opcode_is_nil(vm, ctx, curr_p),
    gen_op::OPCODE_IS_NONEMPTY_LIST => return opcode_is_nonempty_list(vm, ctx, curr_p),
    gen_op::OPCODE_MOVE => return opcode_move(vm, ctx, curr_p),
    gen_op::OPCODE_GET_LIST => return opcode_get_list(vm, ctx, curr_p),
    gen_op::OPCODE_PUT_LIST => return opcode_put_list(vm, ctx, curr_p),
    gen_op::OPCODE_BADMATCH => return opcode_badmatch(vm, ctx, curr_p),
    gen_op::OPCODE_CALL_FUN => return opcode_call_fun(vm, ctx, curr_p),
    gen_op::OPCODE_CALL_EXT_ONLY => return opcode_call_ext_only(vm, ctx, curr_p),
    gen_op::OPCODE_MAKE_FUN2 => return opcode_make_fun2(vm, ctx, curr_p),
    gen_op::OPCODE_GC_BIF1 => return opcode_gc_bif1(vm, ctx, curr_p),
    gen_op::OPCODE_GC_BIF2 => return opcode_gc_bif2(vm, ctx, curr_p),
    gen_op::OPCODE_GC_BIF3 => return opcode_gc_bif3(vm, ctx, curr_p),
    other => unknown_opcode(other, ctx),
  }
  Ok(DispatchResult::Yield)
}

