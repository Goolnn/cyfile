use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Arguments {
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    if !args.path.exists() {
        anyhow::bail!("Path `{}` does not exist", args.path.display());
    }

    if !args.path.is_file() {
        anyhow::bail!("Path `{}` is not a file", args.path.display());
    }

    let project = cyfile::file::open_from_path(&args.path)?;

    println!("{:#?}", project);

    Ok(())
}
