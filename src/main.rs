use std::process::Command;

mod banner;
mod cli;
mod timer;

use crate::banner::gen_banner;
use crate::cli::Arg;
use crate::timer::Timer;

fn main() {
	let Arg { command, arguments } = cli::parse_args();
	let mut child_proc = Command::new(command);
	child_proc.args(arguments);

	let timer = Timer::new();

	let exit_status = child_proc
		.spawn()
		.expect("Failed to spawn process")
		.wait()
		.expect("Process not running");

	let time_elapsed = timer.stop();

	print!("{}\n", gen_banner());
	print!("Process exited after {} ms with ", time_elapsed);
	if let Some(code) = exit_status.code() {
		println!("return code {}", code);
	} else {
		println!("no return code");
	}
}
