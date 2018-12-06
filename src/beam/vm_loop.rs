use beam::disp_result::{DispatchResult};
use beam::gen_op;
use beam::vm_dispatch::dispatch_op_inline;
use emulator::code::{opcode, CodePtr};
use emulator::disasm;
use emulator::runtime_ctx::{Context};
use emulator::scheduler::{SliceResult};
use emulator::vm::{VM};
use fail::{RtResult, Error};

//fn module() -> &'static str { "vm_loop: " }


impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  pub fn dispatch(&mut self) -> RtResult<bool> {
    let mut ctx = Context::new(CodePtr::null());

    let mut scheduler = self.scheduler.borrow_mut();
    let curr_p = match scheduler.next_process() {
      None => return Ok(false),
      Some(p) => scheduler.lookup_pid_mut(p).unwrap()
    };
    ctx.copy_from(&curr_p.context); // swapin

    loop {
      if cfg!(debug_assertions) {
        print!("[exec] ");
        unsafe { disasm::disasm_op(ctx.ip.get(),
                                   self.code_server.borrow().as_ref()); }
      }

      // Take next opcode
      let op = opcode::from_memory_word(ctx.fetch());
      assert!(op <= gen_op::OPCODE_MAX,
              "Opcode too big (wrong memory address?) got 0x{:x}", op.get());

      // Handle next opcode
      let disp_result = dispatch_op_inline(self, op, &mut ctx, curr_p);
      if let Err(Error::Exception(exc_type, exc_reason)) = disp_result {
        curr_p.exception(exc_type, exc_reason);
        curr_p.context.copy_from(&ctx); // swapout
        curr_p.timeslice_result = SliceResult::Exception;
        return Ok(true)
      }

      match disp_result? {
        DispatchResult::Yield => {
          curr_p.context.copy_from(&ctx); // swapout
          curr_p.timeslice_result = SliceResult::Yield;
          return Ok(true)
        },
        DispatchResult::Normal => {
          curr_p.timeslice_result = SliceResult::None;
        }, // keep looping
      }
    } // end loop
  }
}
