use clap::Parser;
use cyfile::Asset;
use cyfile::Note;
use cyfile::Page;
use cyfile::Project;
use cyfile::Text;
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

    let project = Project::new()
        .with_cover(Asset::new("cover.webp", Vec::new()))
        .with_title("Project Title")
        .with_overview("Project Overview")
        .with_page(
            Page::new(Asset::new("pages/page_1.webp", Vec::new()))
                .with_note(
                    Note::new().with_position(0.2, 0.5).with_text(
                        Text::new()
                            .with_content("Page 1 Text 1 Content")
                            .with_comment("Page 1 Text 1 Comment"),
                    ),
                )
                .with_note(
                    Note::new().with_position(0.15, 0.35).with_text(
                        Text::new()
                            .with_content("Page 1 Text 2 Content")
                            .with_comment("Page 1 Text 2 Comment"),
                    ),
                ),
        )
        .with_page(
            Page::new(Asset::new("pages/page_2.webp", Vec::new()))
                .with_note(
                    Note::new().with_position(0.3, 0.6).with_text(
                        Text::new()
                            .with_content("Page 2 Text 1 Content")
                            .with_comment("Page 2 Text 1 Comment"),
                    ),
                )
                .with_note(
                    Note::new().with_position(0.25, 0.45).with_text(
                        Text::new()
                            .with_content("Page 2 Text 2 Content")
                            .with_comment("Page 2 Text 2 Comment"),
                    ),
                ),
        );

    cyfile::file::save_to_path(&args.path, &manifest, &project)?;

    Ok(())
}
