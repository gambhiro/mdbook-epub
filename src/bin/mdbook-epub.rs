use ::env_logger;
#[macro_use]
extern crate log;
use ::failure;
use ::mdbook;
use ::mdbook_epub;
use ::serde_json;
use ::structopt;

use failure::{Error, ResultExt};
use mdbook::renderer::RenderContext;
use mdbook::MDBook;
use std::env;
use std::io;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    info!("Booting EPUB generator...");
    let args = Args::from_args();
    debug!("generator args = {:?}", args);

    if let Err(e) = run(&args) {
        log::error!("{}", e);

        for cause in e.iter_causes() {
            eprintln!("\tCaused By: {}", cause);
        }

        if env::var("RUST_BACKTRACE").is_ok() {
            eprintln!();
            eprintln!("{}", e.backtrace());
        }

        process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), Error> {
    // get a `RenderContext`, either from stdin (because we're used as a plugin)
    // or by instrumenting MDBook directly (in standalone mode).
    let ctx: RenderContext = if args.standalone {
        let error = format!("book.toml root file is not found by a path {:?}",
                            &args.root.display());
        let md = MDBook::load(&args.root).expect(&error);
        let destination = md.build_dir_for("epub");

        RenderContext::new(md.root, md.book, md.config, destination)
    } else {
        serde_json::from_reader(io::stdin()).context("Unable to parse RenderContext")?
    };

    mdbook_epub::generate(&ctx)?;

    Ok(())
}

#[derive(Debug, Clone, StructOpt)]
struct Args {
    #[structopt(
        short = "s",
        long = "standalone",
        help = "Run standalone (i.e. not as a mdbook plugin)"
    )]
    standalone: bool,
    #[structopt(help = "The book to render.", parse(from_os_str), default_value = ".")]
    root: PathBuf,
}
