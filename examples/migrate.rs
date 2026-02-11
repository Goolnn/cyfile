use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use cyfile::Asset;
use cyfile::file::Manifest;
use image::codecs::webp::WebPEncoder;
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;
use indicatif::ProgressStyle;
use rayon::ThreadPoolBuilder;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug)]
struct MigrationResult {
    file_name: String,

    success: bool,

    error: Option<String>,
}

#[derive(Debug, Parser)]
struct Arguments {
    #[clap(short, long)]
    source: PathBuf,
    #[clap(short, long)]
    target: PathBuf,

    #[clap(short = 'j', long = "threads", default_value_t = 4)]
    threads: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    if args.target.exists() && !args.target.is_dir() {
        anyhow::bail!("Target path `{}` is not a directory", args.target.display());
    }

    if !args.target.exists() {
        std::fs::create_dir_all(&args.target).with_context(|| {
            format!(
                "Failed to create target directory `{}`",
                args.target.display()
            )
        })?;
    }

    let sources = {
        if args.source.exists() {
            if args.source.is_file() {
                vec![args.source]
            } else if args.source.is_dir() {
                std::fs::read_dir(&args.source)
                    .with_context(|| {
                        format!(
                            "Failed to read source directory `{}`",
                            args.source.display()
                        )
                    })?
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file())
                    .collect()
            } else {
                anyhow::bail!(
                    "Source path `{}` is neither a file nor a directory",
                    args.source.display()
                );
            }
        } else {
            Vec::new()
        }
    };

    ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()?;

    let multi_progress = Arc::new(MultiProgress::new());

    multi_progress.set_draw_target(ProgressDrawTarget::stdout());

    let target = Arc::new(args.target);

    let results = Arc::new(Mutex::new(Vec::new()));

    sources.par_iter().for_each(|path| {
        let file_name = match path.file_name() {
            Some(val) => val.to_string_lossy().to_string(),
            None => {
                add_result(
                    &results,
                    MigrationResult::failure(
                        path.display().to_string(),
                        "Failed to get file name".to_string(),
                    ),
                );

                return;
            }
        };

        let target = Arc::clone(&target);
        let results = Arc::clone(&results);

        let result = migrate(path, &file_name, &multi_progress, &target);

        add_result(&results, result);
    });

    let result = Arc::try_unwrap(results)
        .map_err(|_| anyhow::anyhow!("Failed to unwrap results Arc"))?
        .into_inner()
        .map_err(|_| anyhow::anyhow!("Failed to lock results Mutex"))?;

    let success_count = result.iter().filter(|r| r.is_success()).count();
    let failure_count = result.iter().filter(|r| r.is_failure()).count();

    println!("\n");

    println!("    Total: {}", result.len());
    println!("  Success: {}", success_count);
    println!("  Failure: {}", failure_count);

    if failure_count > 0 {
        println!("\nFailed Files:");
        for r in result.iter().filter(|r| r.is_failure()) {
            println!(
                "  - {}: {}",
                r.file_name,
                r.error.as_ref().unwrap_or(&"Unknown error".to_string())
            );
        }
    }

    Ok(())
}

fn migrate(
    path: &Path,
    file_name: &str,
    multi_progress: &MultiProgress,
    target: &Path,
) -> MigrationResult {
    let result: Result<()> = (|| -> Result<()> {
        let file =
            File::open(path).with_context(|| format!("Failed to open file `{}`", file_name))?;

        let old = cyfile_old::File::open(file)?;
        let len = old.pages().len();
        let progress_bar = create_progress_bar(multi_progress, len);

        update_progress_bar(&progress_bar, file_name);

        let file_stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| anyhow::anyhow!("Failed to get file stem for `{}`", file_name))?;

        let new = migrate_project(&old, file_stem.to_string(), file_name, &progress_bar)
            .with_context(|| format!("Failed to migrate project `{}`", file_name))?;

        let manifest = Manifest::new();

        cyfile::file::save_to_path(target.join(file_name), &manifest, &new)?;

        progress_bar.set_prefix("●");

        let counter = format_counter(len as u64, len as u64);

        progress_bar.finish_with_message(format!("{:>5} {}", counter, file_name));

        Ok(())
    })();

    match result {
        Ok(_) => MigrationResult::success(file_name.to_string()),
        Err(err) => MigrationResult::failure(file_name.to_string(), format!("{:#}", err)),
    }
}

fn create_progress_bar(multi_progress: &MultiProgress, len: usize) -> ProgressBar {
    let progress_bar = multi_progress.add(if len == 0 {
        multi_progress.add(ProgressBar::new_spinner())
    } else {
        multi_progress.add(ProgressBar::new(len as u64))
    });

    let style = if len == 0 {
        ProgressStyle::with_template("{prefix:.green} {spinner} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
    } else {
        ProgressStyle::with_template("{prefix:.green} [{bar:40.blue}] {percent:>3}% {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("=>-")
    };

    progress_bar.set_style(style);
    progress_bar.set_prefix("○");

    if len == 0 {
        progress_bar.enable_steady_tick(Duration::from_secs_f32(0.1));
    }

    progress_bar
}

fn update_progress_bar(progress_bar: &ProgressBar, file_name: &str) {
    let total = progress_bar.length().unwrap_or(0);
    let current = progress_bar.position();

    let counter = format_counter(current, total);

    progress_bar.set_message(format!("{:>5} {}", counter, file_name));
}

fn format_counter(current: u64, total: u64) -> String {
    if total == 0 {
        "0/0".to_string()
    } else {
        format!("{}/{}", current, total)
    }
}

fn add_result(results: &Arc<Mutex<Vec<MigrationResult>>>, result: MigrationResult) {
    match results.lock() {
        Ok(guard) => guard,

        Err(poisoned) => {
            panic!("Failed to acquire lock on results: {}", poisoned);
        }
    }
    .push(result);
}

pub fn migrate_project(
    project: &cyfile_old::Project,
    title: String,
    file_name: &str,
    progress: &ProgressBar,
) -> Result<cyfile::Project> {
    let mut next = cyfile::Project::new().with_title(title);

    for (index, page) in project.pages().iter().enumerate() {
        let page = migrate_page(page, index + 1)
            .with_context(|| format!("转换第 {} 页失败", index + 1))?;

        next = next.with_page(page);

        progress.inc(1);

        update_progress_bar(progress, file_name);
    }

    Ok(next)
}

pub fn migrate_page(page: &cyfile_old::Page, index: usize) -> Result<cyfile::Page> {
    let image_data = (|| -> Result<Vec<u8>> {
        let image = image::load_from_memory(page.data()).with_context(|| "无法加载图片数据")?;

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        let encoder = WebPEncoder::new_lossless(&mut cursor);
        encoder
            .encode(
                rgba.as_raw(),
                width,
                height,
                image::ExtendedColorType::Rgba8,
            )
            .with_context(|| "WebP 编码失败")?;

        Ok(cursor.into_inner())
    })()
    .with_context(|| format!("处理第 {} 页图片失败", index))?;

    let mut next = cyfile::Page::new()
        .with_image(Asset::new(format!("pages/page_{}.webp", index), image_data));

    for note in page.notes() {
        next = next.with_note(migrate_note(note));
    }

    Ok(next)
}

pub fn migrate_note(note: &cyfile_old::Note) -> cyfile::Note {
    let mut next = cyfile::Note::new().with_position(note.x() as f32, note.y() as f32);

    for text in note.texts() {
        next = next.with_text(migrate_text(text));
    }

    next
}

pub fn migrate_text(text: &cyfile_old::Text) -> cyfile::Text {
    cyfile::Text::new()
        .with_content(text.content())
        .with_comment(text.comment())
}

impl MigrationResult {
    fn success(file_name: String) -> Self {
        Self {
            file_name,

            success: true,

            error: None,
        }
    }

    fn failure(file_name: String, error: String) -> Self {
        Self {
            file_name,

            success: false,

            error: Some(error),
        }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn is_failure(&self) -> bool {
        !self.success
    }
}
