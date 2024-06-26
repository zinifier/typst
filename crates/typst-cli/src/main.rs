use std::process::ExitCode;

use typst::diag::HintedStrResult;

use typst_cli::args::Command;
use typst_cli::timings::Timer;
use typst_cli::{ARGS, EXIT, print_error, set_failed};

/// Entry point.
fn main() -> ExitCode {
    // Handle SIGPIPE
    // https://stackoverflow.com/questions/65755853/simple-word-count-rust-program-outputs-valid-stdout-but-panicks-when-piped-to-he/65760807
    sigpipe::reset();

    let res = dispatch();

    if let Err(msg) = res {
        set_failed();
        print_error(msg.message()).expect("failed to print error");
    }

    EXIT.with(|cell| cell.get())
}

/// Execute the requested command.
fn dispatch() -> HintedStrResult<()> {
    let mut timer = Timer::new(&ARGS);

    match &ARGS.command {
        Command::Compile(command) => typst_cli::compile::compile(&mut timer, command)?,
        Command::Watch(command) => typst_cli::watch::watch(&mut timer, command)?,
        Command::Init(command) => typst_cli::init::init(command)?,
        Command::Query(command) => typst_cli::query::query(command)?,
        Command::Fonts(command) => typst_cli::fonts::fonts(command),
        Command::Update(command) => typst_cli::update::update(command)?,
    }

    Ok(())
}
