use cyfile::ExportArguments;
use cyfile::File;
use cyfile::Note;
use cyfile::Page;
use cyfile::Project;
use cyfile::Text;
use std::fs;

fn main() {
    let note_0 = Note::new()
        .with_coordinate(0.5, 0.5)
        .with_text(Text::new().with_content("content").with_comment("comment"));

    let note_1 = Note::new()
        .with_coordinate(-0.5, -0.5)
        .with_text(Text::new().with_content("content").with_comment("comment"));

    let image = fs::read("tests/images/0.png").unwrap();
    let page = Page::new(image).with_note(note_0).with_note(note_1);
    let project = Project::new().with_title("Export").with_page(page);

    let args_0_0 = ExportArguments::new("examples/outputs/v0.0.cy").with_version((0, 0));
    let args_0_1 = ExportArguments::new("examples/outputs/v0.1.cy").with_version((0, 1));
    let args_0_2 = ExportArguments::new("examples/outputs/v0.2.cy").with_version((0, 2));

    File::export(&project, args_0_0).unwrap();
    File::export(&project, args_0_1).unwrap();
    File::export(&project, args_0_2).unwrap();
}
