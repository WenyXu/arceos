//! Trap handling.

use crate_interface::{call_interface, def_interface};

/// Trap handler interface.
///
/// This trait is defined with the [`#[def_interface]`][1] attribute. Users
/// should implement it with [`#[impl_interface]`][2] in any other crate.
///
/// [1]: crate_interface::def_interface
/// [2]: crate_interface::impl_interface
#[def_interface]
pub trait TrapHandler {
    /// Handles interrupt requests for the given IRQ number.
    fn handle_irq(irq_num: usize);
    // more e.g.: handle_page_fault();
    #[cfg(feature = "user")]
    // 需要分离用户态使用
    fn handle_syscall(syscall_id: usize, args: [usize; 6]) -> isize;
    // 最多接受十个参数
}

/// Call the external IRQ handler.
#[allow(dead_code)]
pub(crate) fn handle_irq_extern(irq_num: usize) {
    call_interface!(TrapHandler::handle_irq, irq_num);
}

/// Call the syscall handler
#[allow(dead_code)]
#[cfg(feature = "user")]
/// 分割token流
pub(crate) fn handle_syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    call_interface!(TrapHandler::handle_syscall, syscall_id, args)
}
