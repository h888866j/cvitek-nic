//! TODO: PLIC

use crate::irq::IrqHandler;
use lazy_init::LazyInit;
use riscv::register::sie;
use core::ptr::{read_volatile,write_volatile};
use crate::mem::*;

/// `Interrupt` bit in `scause`
pub(super) const INTC_IRQ_BASE: usize = 1 << (usize::BITS - 1);

/// Supervisor software interrupt in `scause`
#[allow(unused)]
pub(super) const S_SOFT: usize = INTC_IRQ_BASE + 1;

/// Supervisor timer interrupt in `scause`
pub(super) const S_TIMER: usize = INTC_IRQ_BASE + 5;

/// Supervisor external interrupt in `scause`
pub(super) const S_EXT: usize = INTC_IRQ_BASE + 9;

static TIMER_HANDLER: LazyInit<IrqHandler> = LazyInit::new();

/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 1024;

/// The timer IRQ number (supervisor timer interrupt in `scause`).
pub const TIMER_IRQ_NUM: usize = S_TIMER;

macro_rules! with_cause {
    ($cause: expr, @TIMER => $timer_op: expr, @EXT => $ext_op: expr $(,)?) => {
        match $cause {
            S_TIMER => $timer_op,
            S_EXT => $ext_op,
            _ => panic!("invalid trap cause: {:#x}", $cause),
        }
    };
}

/// Enables or disables the given IRQ.
pub fn set_enable(scause: usize, _enabled: bool) {
    info!("go into set enable\n");
    let PLIC_OFF:usize=0x7000_0000;
    let x:u32=(scause as u32)/32;
    let y:u32=(scause as u32)%32;
    let PLIC_H0_MIE0:usize=0x0000_2000;
    
    let PLIC_H:usize=PLIC_OFF+PLIC_H0_MIE0+x as usize;
    let PLIC_MIE:usize=1<<y;

    info!("start test plic read1");

    ///let PLIC_H = 0x1000_0000;
    unsafe{
        let addr=usize::from(phys_to_virt((PLIC_OFF+0x4).into()));
        info!("addr {:#x}",addr);
        let curr_reg_num=read_volatile(addr as *mut u32);
        info!("the plic h is {}",curr_reg_num);
    }
    if scause == S_EXT {
        // TODO: set enable in PLIC
        // let PLIC_OFF:usize=0x0c00_0000;
        
        
    }
}

/// Registers an IRQ handler for the given IRQ.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(scause: usize, handler: IrqHandler) -> bool {
    with_cause!(
        scause,
        @TIMER => if !TIMER_HANDLER.is_init() {
            TIMER_HANDLER.init_by(handler);
            true
        } else {
            false
        },
        @EXT => crate::irq::register_handler_common(scause & !INTC_IRQ_BASE, handler),
    )
}

/// Dispatches the IRQ.
///
/// This function is called by the common interrupt handler. It looks
/// up in the IRQ handler table and calls the corresponding handler. If
/// necessary, it also acknowledges the interrupt controller after handling.
pub fn dispatch_irq(scause: usize) {
    with_cause!(
        scause,
        @TIMER => {
            trace!("IRQ: timer");
            TIMER_HANDLER();
        },
        @EXT => crate::irq::dispatch_irq_common(0), // TODO: get IRQ number from PLIC
    );
}

pub(super) fn init_percpu() {
    // enable soft interrupts, timer interrupts, and external interrupts
    unsafe {
        sie::set_ssoft();
        sie::set_stimer();
        sie::set_sext();
    }
}
