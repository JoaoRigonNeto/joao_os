//Tutorial developed by Philipp Oppermann - https://os.phil-opp.com/freestanding-rust-binary/

//Disabling std library from build.
#![no_std]
//Overwriting Entry Point
#![no_main]

use core::panic::PanicInfo;

//Function to be called when panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop{}
}
/*
	Overwriting OS entry point with our own
	No mangle disables name mangling so rust compiler outputs func name as _start - 
		otherwise the compiler would name it with a criptic name so it would be ensured - 
		as a unique name.
	Also marking with (extern "C") so the compiler uses the C calling conversion and not the - 
		Rust one.
	Naming the function (_start) because its an entry point name for most systems
	The ! return type means that the function is diverging, which means it is not allowed to ever return - 
		this is required due to the fact that the entry point is invoked directly by the OS or bootloader -
		and not by a function. Instead of returning, the (exit) system call should be invoked.
*/
#[no_mangle]
pub extern "C" fn _start() -> ! {
	loop{}
}