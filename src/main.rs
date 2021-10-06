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



static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	//Raw pointer pointing to VGA buffer
	let vga_buffer = 0xb8000 as *mut u8;
	//Iterating through bytes of string
	for (i, &byte) in HELLO.iter().enumerate() {
			// Rust compiler can't tell if raw pointer is valid
			//so unsafe is used meaning we are sure that the raw pointer is valid
	        unsafe {
	        	//Writting the string byte
	            *vga_buffer.offset(i as isize * 2) = byte;
				//Writting string byte color (Light cyan)
	            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
	        }
	    }
	loop{}
}