use cyfile::Credit;
use cyfile::File;
use cyfile::Note;
use cyfile::Page;
use cyfile::Project;
use cyfile::Text;
use std::fs;

fn main() {
    let cover = fs::read("tests/images/0.png").unwrap();

    let note_0 = Note::new()
        .with_coordinate(0.5, 0.5)
        .with_text(Text::new().with_content("content").with_comment("comment"));

    let note_1 = Note::new()
        .with_coordinate(-0.5, -0.5)
        .with_text(Text::new().with_content("content").with_comment("comment"));

    let image = fs::read("tests/images/0.png").unwrap();
    let page = Page::new(image).with_note(note_0).with_note(note_1);

    let project = Project::new()
        .with_cover(cover)
        .with_category("Main Story")
        .with_title("Create")
        .with_credit(Credit::Artists, "Ayaka")
        .with_credit(Credit::Translators, "Chongbenbao")
        .with_credit(Credit::Proofreaders, "Jiangguo")
        .with_credit(Credit::Retouchers, "Baishi")
        .with_credit(Credit::Retouchers, "Zheerhui")
        .with_credit(Credit::Typesetters, "Zheerhui")
        .with_credit(Credit::Typesetters, "Goolnn")
        .with_credit(Credit::Supervisors, "Chaibai")
        .with_page(page);

    let file = File::create(project);

    println!("{:#?}", file.project());
}
