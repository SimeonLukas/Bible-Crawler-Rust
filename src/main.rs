use regex::Regex;
use serde_json;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Include all bible .json files
    let eu_json = include_str!("EU.json").to_string();
    let lut_json = include_str!("LUT.json").to_string();
    let elb_json = include_str!("ELB.json").to_string();
    let hfa_json = include_str!("HFA.json").to_string();

    println!("Welche Bibel soll es sein?");
    // Print all available bibles in a table
    println!(
        "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
        "1. Einheitsübersetzung (EU)",
        "2. Luther Bibel (LUT)",
        "3. Elberfelder Bibel (ELB)",
        "4. Hoffnung für alle (HFA)"
    );

    // generate mutable variables
    let mut json = String::new();
    let mut version = String::new();
    let mut book = String::new();
    // user selects bible version
    io::stdin()
        .read_line(&mut version)
        .expect("Diese Version kenne ich nicht!");
    // clean up input
    version = version.trim().to_string();

    if version == "1" || version == "EU" {
        println!("Die Einheitsübersetzung wurde gewählt.");
        version = "EU".to_string();
        json = eu_json;
    } else if version == "2" || version == "LUT" {
        println!("Die Luther Bibel wurde gewählt!");
        version = "LUT".to_string();
        json = lut_json;
    } else if version == "3" || version == "ELB" {
        println!("Die Elberfelder Bibel wurde gewählt!");
        version = "ELB".to_string();
        json = elb_json;
    } else if version == "4" || version == "HFA" {
        println!("Die Elberfelder Bibel wurde gewählt!");
        version = "HFA".to_string();
        json = hfa_json;
    } else {
        println!(
            "Da keine vorhandene Bibel gewählt wurde, wurde die Einheitsübersetzung ausgewählt."
        );
        version = "EU".to_string();
        json = eu_json;
    }

    let mut booklist = "".to_string();
    let json: serde_json::Value = serde_json::from_str(&json).expect("file should be proper JSON");
    let books = json.get("Books").expect("file should have books key");
    let books = books.to_string();
    let books: u8 = books.parse().unwrap();
    for n in 1..books + 1 {
        if n % 5 == 0 {
            booklist.push_str("\n");
        }
        let number = n.to_string();
        let book = json.get(&number).expect("file should have books key");
        let book_name = book.get("name").expect("file should have key");
        let book_name = book_name.to_string();
        let book_name = book_name.replace("\"", "");
        // Generate readable List of Books
        booklist.push_str(&number);
        booklist.push_str(". ");
        booklist.push_str(&book_name);
        booklist.push_str(" | ")
    }
    // Show List of Books
    println!("{}", &booklist);
    println!("Welches Buch möchtest du lesen? Bitte Nummer angeben.");
    // user selects book to download
    io::stdin()
        .read_line(&mut book)
        .expect("Dieses Buch kenne ich nicht!");

    let book = book.trim();
    let mut book: u8 = book.parse().expect("Es wurde keine Nummer angegeben.");
    // fallback on wrong input
    if book > books || book == 0 {
        println!("Dieses Buch gibt es nicht. Es wird das Buch Genesis gewählt.");
        book = 1;
    }
    let book = book.to_string();
    let book = json.get(book).expect("file should have key");
    let chapters = book.get("chapters").expect("file should have key");
    let chapters = chapters.to_string();
    let chapters: u8 = chapters.parse().unwrap();
    let book = book.get("name").expect("file should have key");
    let book = book.to_string();
    let book = book.replace("\"", "");
    println!("{}", &book);

    println!("{}", "Dokument wird erstellt!");
    let dateiname = format!("{version}/{book}.md");
    let dir = format!("{version}");
    if Path::new(&dir).exists() != true {
        fs::create_dir(&dir)?;
    }
    let mut file = File::create(&dateiname).expect("Datei konnte leider nicht erstellt werden.");
    let mut ausgabe = " ".to_string();
    for n in 1..chapters + 1 {
        let url = "https://www.bibleserver.com/";
        let urlfin = format!("{url}{version}/{book}{n}");
        // Check for the right url
        // println!("{}", urlfin);
        let ergebnis: String = reqwest::get(urlfin).await?.text().await?.to_string();
        let split: Vec<&str> = ergebnis
            .split("<header style=\"grid-row-start: 1;grid-row-end: 2\">")
            .collect();
        let split_2: Vec<&str> = split[1].split("<footer ").collect();

        let replace_biblename = Regex::new(r#"<h1(.*)"#).unwrap();
        let replace_tags = Regex::new(r#"<.*?>"#).unwrap();
        let replace_linebreaks = Regex::new(r#"\n"#).unwrap();
        let replace_footnotes = Regex::new(r#".[0-9]]"#).unwrap();
        let text = split_2[0];
        let text = replace_biblename.replace_all(&text, "#");
        let text = replace_tags.replace_all(&text, "");
        let text = replace_linebreaks.replace_all(&text, "");
        let text = replace_footnotes.replace_all(&text, "");
        let text = text.trim();
        let text = text.replace("\u{2}", "\n");
        let text = text.replace("\u{3}", "");
        let text = text.replace(" &#x1;", "\n## ");
        let text = text.replace("&#x1;", "\n## ");
        let text = text.trim();
        println!("Kapitel {} gecrawled.", &n);
        ausgabe.push_str(&text);
        ausgabe.push_str("\n");
    }
    file.write_all(ausgabe.as_bytes())
        .expect("Inhalt konnte leider nicht geschrieben werden!");
    println!(
        "Das Dokument {}.md wurde im Ordner {} erstellt.",
        &book, &version
    );
    Ok(())
}
