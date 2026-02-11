use clap::Parser;
use cyfile::Project;
use cyfile::file::Manifest;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Arguments {
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    if args.path.exists() {
        anyhow::bail!("Path `{}` already exists", args.path.display());
    }

    let manifest = Manifest::default();
    let project = Project::new();

    cyfile::file::save_to_path(&args.path, &manifest, &project)?;

    Ok(())
}
