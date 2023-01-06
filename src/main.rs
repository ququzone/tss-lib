use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        // Constants
        Subcommands::Server => {
            println!("{}", "hello tss server");
        }
    }

    Ok(())
}
