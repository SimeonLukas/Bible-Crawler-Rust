use regex::Regex;
use serde_json;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
// use std::thread;
// use std::time::Duration;

fn main() {
    start();
    let mut entscheidung = String::new();
    println!("Nochmal? J/N");
    io::stdin()
        .read_line(&mut entscheidung)
        .expect("Diese Version kenne ich nicht!");

    entscheidung = entscheidung.trim().to_string();

    if entscheidung == "N" || entscheidung == "n" {
        exit(1);
    } else {
        main();
    }
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // Include all bible .json files
    let eu_json = include_str!("EU.json").to_string();
    let lut_json = include_str!("LUT.json").to_string();
    let elb_json = include_str!("ELB.json").to_string();
    let hfa_json = include_str!("HFA.json").to_string();
    let vlx_json = include_str!("VLX.json").to_string();

    println!("Welche Bibel soll es sein?");
    // Print all available bibles in a table
    println!("Deutsche Übersetzungen");
    println!(
        "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
        "1. Einheitsübersetzung (EU)",
        "2. Luther Bibel (LUT)",
        "3. Elberfelder Bibel (ELB)",
        "4. Hoffnung für alle (HFA)"
    );
    println!("{0: <30}", "5. Die Volxbibel (VLX)");

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
    } else if version == "5" || version == "VLX" {
        println!("Die Elberfelder Bibel wurde gewählt!");
        version = "VLX".to_string();
        json = vlx_json;
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
    for n in 1..&books + 1 {
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
    println!("{}", "Nummer 0 für alle Bücher");
    println!("Welches Buch möchtest du lesen? Bitte Nummer angeben.");
    // user selects book to download
    io::stdin()
        .read_line(&mut book)
        .expect("Dieses Buch kenne ich nicht!");
    let mut v = Vec::new();
    let book = book.trim();
    let mut book: u8 = book.parse().expect("Es wurde keine Nummer angegeben.");
    // fallback on wrong input
    if book > books{
        println!("Dieses Buch gibt es nicht. Es wird das Buch Genesis gewählt.");
        book = 1;
        v.push(book);
        
    }
    else if book == 0 {
        //    printljn!("{}", &z);
           v = (1..books+1).collect();
        
    }
    else{
        v.push(book);
    }
    for i in v {
    let book = i.to_string();
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
        let mut url = String::new();
        let mut urlfin = String::new();
        let mut text = String::new();
        let replace_biblename = Regex::new(r#"<h1(.*)"#).unwrap();
        let replace_tags = Regex::new(r#"<.*?>"#).unwrap();
        let replace_linebreaks = Regex::new(r#"\n"#).unwrap();
        let replace_footnotes = Regex::new(r#".[0-9]]"#).unwrap();

        if version == "VLX" {
            url = "https://lesen.volxbibel.de/book/".to_string();
            urlfin = format!("{url}{book}/chapter/{n}");
            let number = n.to_string();
            let book = book.to_string();
            text.push_str("# ");
            text.push_str(&book);
            text.push_str(" ");
            text.push_str(&number);
            text.push_str(" Die Volxbibel\n");
            let ergebnis = reqwest::get(urlfin).await?.text().await?.to_string();
            let ergebnis: Vec<&str> = ergebnis.split("&quot;verses&quot;:").collect();
            let ergebnis: Vec<&str> = ergebnis[1].split(",&quot;guid&quot;").collect();
            let ergebnis = ergebnis[0].to_string();
            let ergebnis = ergebnis.replace("&quot;", "\"");
            let chapter_json: Vec<Value> =
                serde_json::from_str(&ergebnis).expect("file should be proper JSON");
            for vers in &chapter_json {
                let vers = vers.to_string();
                let vers_json: serde_json::Value =
                    serde_json::from_str(&vers).expect("file should be proper JSON");
                let label = vers_json.get("label").expect("file should have key");
                let vers_text = vers_json.get("text").expect("file should have key");

                let mut vers = format!("{} {}", &label, &vers_text);
                vers = vers.replace("\\n", "");
                vers = vers.replace("\"", "");
                vers = vers.replace("&#039;", "'");
                vers = vers.replace("headline", "##");
                vers = vers.trim().to_string();
                if vers.len() > 3 {
                    text.push_str(&vers);
                    text.push_str("\n");
                }

            }
        } else {
            url = "https://www.bibleserver.com/".to_string();
            urlfin = format!("{url}{version}/{book}{n}");
            let ergebnis: String = reqwest::get(urlfin).await?.text().await?.to_string();
            let ergebnis: Vec<&str> = ergebnis
                .split("<header style=\"grid-row-start: 1;grid-row-end: 2\">")
                .collect();
            let ergebnis: Vec<&str> = ergebnis[1].split("<footer ").collect();
            text = ergebnis[0].to_string();
            text = replace_biblename.replace_all(&text, "#").to_string();
            text = replace_tags.replace_all(&text, "").to_string();
            text = replace_linebreaks.replace_all(&text, "").to_string();
            text = replace_footnotes.replace_all(&text, "").to_string();
            text = text.trim().to_string();
            text = text.replace("\u{2}", "\n");
            text = text.replace("\u{3}", "");
            text = text.replace(" &#x1;", "\n## ");
            text = text.replace("&#x1;", "\n## ");
            text = text.trim().to_string();
        }
        println!("Kapitel {} gecrawled.", &n);





        let replace_verse_start = Regex::new(r"\n(?P<v>\d+)").unwrap();
        let chapter_text = replace_verse_start.replace_all(&text, "\n ### $v \n").to_string();
        let chapter_file = format!("{version}/{book}/{book} {n}.md");
        let dirchapter = format!("{version}/{book}");
        if Path::new(&dirchapter).exists() != true {
            fs::create_dir(&dirchapter)?;
        }
        let mut chapterfile = File::create(&chapter_file).expect("Datei konnte leider nicht erstellt werden.");
        chapterfile.write_all(&chapter_text.as_bytes())
        .expect("Inhalt konnte leider nicht geschrieben werden!");


        ausgabe.push_str(&text);
        ausgabe.push_str("\n");
    }
    file.write_all(ausgabe.as_bytes())
        .expect("Inhalt konnte leider nicht geschrieben werden!");
    println!(
        "Das Dokument {}.md wurde im Ordner {} erstellt.",
        &book, &version
    );
}
    Ok(())
}



