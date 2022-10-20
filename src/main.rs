use regex::Regex;
use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;
use serde_json::{Number, Value};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let EU_JSON = include_str!("EU.json").to_string();
    let LUT_JSON = include_str!("LUT.json").to_string();
    println!("Welche Bibel soll es sein?");
    println!(
        "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
        "1. Einheitsübersetzung (EU)", "2. Lutherbibel (LUT)", "DBU", "ELB"
    );
    let mut JSON = String::new();
    let mut version = String::new();
    let mut book = String::new();

    io::stdin()
        .read_line(&mut version)
        .expect("Diese Version kenne ich nicht!");

    version = version.trim().to_string();

    if version == "1" || version == "EU" {
        println!("Die Einheitsüberstzung wurde gewählt.");
        version = "EU".to_string();
        JSON = EU_JSON;
    }

    if version == "2" || version == "LUT" {
        println!("Die Lutherbibel wurde gewählt!");
        version = "LUT".to_string();
        JSON = LUT_JSON;
    }





        // let json = format!("{version}.json");
        // let file = File::open(json)
        // .expect("file should open read only");
        let json: serde_json::Value = serde_json::from_str(&JSON)
        .expect("file should be proper JSON");
        let Books = json.get("Books")
        .expect("file should have Books key");
        let Books = Books.to_string();
        let Books: u8 = Books.parse().unwrap();
        for n in 1..Books+1{
            let number = n.to_string();
            let Book = json.get(&number)
            .expect("file should have Books key");
            let Book_name = Book.get("name")
            .expect("file should have key");
            let Book_name = Book_name.to_string();
            let Book_name = Book_name.replace("\"", "");
            println!("{}. {}", &number ,&Book_name);
        }
        println!("Welches Buch möchtest du lesen? Bitte Nummer angeben.");
        io::stdin()
            .read_line(&mut book)
            .expect("Dieses Buch kenne ich nicht!");
        let book = book.trim();
        let book = json.get(book)
        .expect("file should have key");
        let chapters = book.get("chapters")
        .expect("file should have key");
        let chapters = chapters.to_string();
        let chapters: u8 = chapters.parse().unwrap();
        let book = book.get("name")
        .expect("file should have key");
        let book = book.to_string();
        let book = book.replace("\"", "");
        println!("{}", &book);

    println!("{}", "Dokument wird erstellt!");
    let dateiname = format!("{version}/{book}.md");
    let dir = format!("{version}");
    if Path::new(&dir).exists() != true{
        fs::create_dir(&dir)?;
    }
    let mut file = File::create(&dateiname).expect("Datei konnte leider nicht erstellt werden.");
    let mut ausgabe = " ".to_string();
    for n in 1..chapters+1 {
        let url = "https://www.bibleserver.com/";
        let urlfin = format!("{url}{version}/{book}{n}");
        println!("{}", urlfin);
        let ergebnis: String = reqwest::get(urlfin).await?.text().await?.to_string();
        let split: Vec<&str> = ergebnis
            .split("<header style=\"grid-row-start: 1;grid-row-end: 2\">")
            .collect();
        let split_2: Vec<&str> = split[1].split("<footer ").collect();

        let replace_biblename = Regex::new(r#"<h1(.*)"#).unwrap();
        let replace_tags = Regex::new(r#"<.*?>"#).unwrap();
        let replace_linebreaks = Regex::new(r#"\n"#).unwrap();
        let replace_footnotes = Regex::new(r#".[0-9]]"#).unwrap();
        let m = split_2[0];
        // println!("{:#?}", &m);
        let ma = replace_biblename.replace_all(&m, "#");
        let mat = replace_tags.replace_all(&ma, "");
        let matc = replace_linebreaks.replace_all(&mat, "");
        let matc = replace_footnotes.replace_all(&matc, "");
        println!("{:#?}", &matc);
        let matc = matc.trim();
        let matc = matc.replace("\u{2}", "\n");
        let matc = matc.replace("\u{3}", "");
        let matc = matc.replace(" &#x1;", "\n## ");
        let matc = matc.replace("&#x1;", "\n## ");
        let matc = matc.trim();
        ausgabe.push_str(&matc);
        ausgabe.push_str("\n");
    }
    file.write_all(ausgabe.as_bytes())
        .expect("Inhalt konnte leider nicht geschrieben werden!");
    println!("{}", "Dokument wurde erstellt!");
    Ok(())
}
