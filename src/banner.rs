use terminal_size::{terminal_size, Width};

const BANNER_CHAR: char = '=';

pub fn gen_banner() -> String {
	let mut len = 80usize;
	if let Some((Width(width), _)) = terminal_size() {
		len = width as usize;
	}

	String::from(BANNER_CHAR).repeat(len)
}
