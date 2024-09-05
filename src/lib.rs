use clap::{ArgGroup, Parser};
use copypasta::{ClipboardContext, ClipboardProvider};
use shell_quote::{Bash, QuoteRefExt};
use std::io::{self, Read, Write};

/// This small script sends the first line of stdin to the system clipboard.
/// All stdin inputs will be pushed as is to stdout.
#[derive(Parser, Debug)]
#[command(version, about)]
#[clap(group(
    ArgGroup::new("quote")
        .args([
            "single_quote",
            "double_quote",
            "backtick_quote",
            "auto_bash_quote",
        ])))]
struct Cli {
    /// Add single quotes around the line to copy before copying.
    /// This is mutually exclusive with `-Qba`.
    #[arg(short = 'q', long = "quote-single", default_value_t = false)]
    single_quote: bool,
    /// Add double quotes around the line to copy before copying.
    /// This is mutually exclusive with `-qba`.
    #[arg(short = 'Q', long = "quote-double", default_value_t = false)]
    double_quote: bool,
    /// Add backticks around the line to copy before copying.
    /// This is mutually exclusive with `-qQa`.
    #[arg(short = 'b', long = "quote-backtick", default_value_t = false)]
    backtick_quote: bool,
    /// Add appropriate bash quotes around the line to copy before copying.
    /// This is mutually exclusive with `-qQb`.
    #[arg(short = 'a', long = "quote-bash-auto", default_value_t = false)]
    auto_bash_quote: bool,
    /// Don't trim the ending newline before copying.
    #[arg(short = 'n', long = "keep-newline", default_value_t = false)]
    keep_newline: bool,
}

#[derive(Debug)]
pub enum Error {
    Clipboard,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

fn copy(text: String) -> Result<(), Error> {
    let mut ctx = ClipboardContext::new().map_err(|_| Error::Clipboard)?;
    ctx.set_contents(text).map_err(|_| Error::Clipboard)?;
    Ok(())
}

/// Read one line from stdin, copy as specified, and echo the line.
fn read_line_and_copy_and_echo(cli: &Cli) -> Result<(), Error> {
    let mut cbuf = String::new();
    let nbytes = io::stdin().read_line(&mut cbuf)?;
    if nbytes > 0 {
        let cbuf = cbuf;
        let to_copy = if !cli.keep_newline {
            cbuf.trim_end_matches(|c| c == '\r' || c == '\n')
        } else {
            &cbuf
        };
        let to_copy = if cli.single_quote {
            format!("'{}'", to_copy)
        } else if cli.double_quote {
            format!("\"{}\"", to_copy)
        } else if cli.backtick_quote {
            format!("`{}`", to_copy)
        } else if cli.auto_bash_quote {
            to_copy.quoted(Bash)
        } else {
            to_copy.to_string()
        };
        copy(to_copy)?;
        print!("{}", cbuf);
    }
    Ok(())
}

/// Read stdin and echo until EOF.
fn read_until_eof_and_echo() -> Result<(), Error> {
    let mut cbuf = [0u8; 1024];
    loop {
        let nbytes = io::stdin().read(&mut cbuf)?;
        if nbytes == 0 {
            break Ok(());
        }
        io::stdout().write_all(&cbuf[..nbytes])?
    }
}

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Error> {
        let cli = Cli::parse();
        read_line_and_copy_and_echo(&cli)?;
        read_until_eof_and_echo()?;
        Ok(())
    }
}
