use clap::{
	app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings,
};

#[derive(Debug)]
pub struct Arg {
	pub command: String,
	pub arguments: Vec<String>,
}

pub fn parse_args() -> Arg {
	let matches = app_from_crate!()
		.version_short("v")
		.args_from_usage(
			"<command>	'Command to execute'
			[arguments]... 'Arguments to pass to the command'",
		)
		.setting(AppSettings::TrailingVarArg)
		.get_matches();

	let command = matches.value_of("command").unwrap().to_string();

	let mut arguments = vec![];
	if let Some(args) = matches.values_of("arguments") {
		arguments = args.map(|x| x.to_string()).collect();
	}

	Arg { command, arguments }
}
