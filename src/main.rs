use pbr::{ProgressBar, Units};
use rayon::prelude::*;
use regex::Regex;
use serde_json;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::stderr;
use std::path::Path;
use std::process::exit;
use std::thread;

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();
    crawler();
}

fn crawler() {
    start();
    let mut entscheidung = String::new();
    println!("\nNochmal? J/N");
    io::stdin()
        .read_line(&mut entscheidung)
        .expect("Diese Version kenne ich nicht!");

    entscheidung = entscheidung.trim().to_string();

    if entscheidung == "N" || entscheidung == "n" {
        exit(1);
    } else {
        crawler();
    }
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // Füge .json für Namenskonventionen ein
    let names_json = include_str!("names.json").to_string();
    // Füge .json der Bibelübersetungen hinzu
    let eu_json = include_str!("EU.json").to_string();
    let lut_json = include_str!("LUT.json").to_string();
    let elb_json = include_str!("ELB.json").to_string();
    let hfa_json = include_str!("HFA.json").to_string();
    let btx_json = include_str!("BTX.json").to_string();
    let slt_json = include_str!("SLT.json").to_string();
    let gnb_json = include_str!("GNB.json").to_string();
    let neue_json = include_str!("NEUE.json").to_string();
    let bb_json = include_str!("BB.json").to_string();
    let na28_json = include_str!("NA28.json").to_string();
    let bhs_json = include_str!("BHS.json").to_string();

    println!("Welche Bibel soll es sein?");
    println!("Übersetzungen von bibleserver.com:");
    println!(
        "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
        "1. Einheitsübersetzung (EU)",
        "2. Luther Bibel (LUT)",
        "3. Elberfelder Bibel (ELB)",
        "4. Hoffnung für alle (HFA)"
    );
    println!(
        "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
        "5. Schlachter Bibel (SLT)",
        "6. La Biblia Textual (BTX)",
        "7. Gute Nachricht Bibel 2018 (GNB)",
        "8. Neue Evangelistische Übersetzung (NEUE / NeÜ)"
    );
    println!("Übersetzungen von die-bibel.de:");
    println!(
        "{0: <30} | {1: <30} | {2: <30}",
        "9. Basis Bibel (BB)",
        "10. Nestle-Aland 28 (NA28)",
        "11. Biblia Hebraica Stuttgartensia (BHS)"
    );
    let mut json = String::new();
    let mut version = String::new();
    let mut book = String::new();
    io::stdin()
        .read_line(&mut version)
        .expect("Diese Version kenne ich nicht!");
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
        println!("Die Hoffnung für alle Bibel wurde gewählt!");
        version = "HFA".to_string();
        json = hfa_json;
    } else if version == "5" || version == "SLT" {
        println!("Die Schlachter Bibel wurde gewählt!");
        version = "SLT".to_string();
        json = slt_json;
    } else if version == "6" || version == "BTX" {
        println!("La Biblia Textual wurde gewählt!");
        version = "BTX".to_string();
        json = btx_json;
    } else if version == "7" || version == "GNB" {
        println!("Gute Nachricht Bibel 2018 wurde gewählt!");
        version = "GNB".to_string();
        json = gnb_json;
    } else if version == "8" || version == "NEÜ" {
        println!("Neue Evangelistische Übersetzung wurde gewählt!");
        version = "NEÜ".to_string();
        json = neue_json;
    } else if version == "9" || version == "BB" {
        println!("Basis Bibel wurde gewählt!");
        version = "BB".to_string();
        json = bb_json;
    } else if version == "10" || version == "NA28" {
        println!("Nestle-Aland 28 wurde gewählt!");
        version = "NA28".to_string();
        json = na28_json;
    } else if version == "11" || version == "BHS" {
        println!("Biblia Hebraica Stuttgartensia wurde gewählt!");
        version = "BHS".to_string();
        json = bhs_json;
    } else {
        println!(
            "Da keine vorhandene Bibel gewählt wurde, wurde die Einheitsübersetzung ausgewählt."
        );
        version = "EU".to_string();
        json = eu_json;
    }

    println!("\nWelche Namenskonvention soll verwendet werden?");
    println!("1. Abkürzungen Großbuchstaben (GEN, EXO, LEV, NUM, DEU, etc.)");
    println!("2. Abkürzungen Groß und Kleinbuchstaben  (Gen, Exo, Lev, Num, Deu, etc.)");
    println!("3. Deutsch typisch evangelisch (1. Mose, 2. Mose, 3. Mose, 4. Mose, 5. Mose, etc.)");
    println!(
        "4. Deutsch typisch katholisch (Genesis, Exodus, Levitikus, Numeri, Deuteronomium, etc.)"
    );
    println!("5. Englisch (Genesis, Exodus, Leviticus, Numbers, Deuteronomy, etc.)");
    println!("6. Latein (Genesis, Exodus, Leviticus, Numeri, Deuteronomium, etc.)");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Diese Namenskonvention kenne ich nicht!");
    // clean up input
    name = name.trim().to_string();
    if name == "1" {
        name = "Abkürzungen1".to_string();
        println!("Abkürzungen wurden gewählt.");
    } else if name == "2" {
        name = "Abkürzungen2".to_string();
        println!("Abkürzungen wurden gewählt.");
    } else if name == "3" {
        name = "Deutsch1".to_string();
        println!("Deutsch evang. wurde gewählt.");
    } else if name == "4" {
        name = "Deutsch2".to_string();
        println!("Deutsch kath. wurde gewählt.");
    } else if name == "5" {
        name = "Englisch".to_string();
        println!("Englisch wurde gewählt.");
    } else if name == "6" {
        name = "Latein".to_string();
        println!("Latein wurde gewählt.");
    } else {
        name = "Abkürzungen".to_string();
        println!("Da keine Namenskonvention gewählt wurde, wurden Abkürzungen ausgewählt.");
    }
    let nameconvention = name.clone();
    let mut booklist = "".to_string();
    let json: serde_json::Value = serde_json::from_str(&json).expect("file should be proper JSON");
    let namesjson: serde_json::Value =
        serde_json::from_str(&names_json).expect("file should be proper JSON");
    let books = json.get("Books").expect("file should have books key");
    let books = books.to_string();
    let books: u8 = books.parse().unwrap();
    let v: Vec<u8> = (1..books + 1).collect();
    let mut total_chapters: i32 = 0;
    for book in &v {
        let chapters = json
            .get(&book.to_string())
            .expect("file should have key")
            .get("chapters")
            .expect("file should have key");
        let chapters = chapters.to_string();
        let chapters: i32 = chapters.parse().unwrap();
        total_chapters = total_chapters
            .checked_add(chapters)
            .expect("Overflow occurred while adding chapters");
    }
    let pb = std::sync::Arc::new(std::sync::Mutex::new(ProgressBar::new(
        total_chapters as u64,
    )));
    v.par_iter().for_each(|&i| {
        let pb = pb.clone();
        let mut pb = pb.lock().unwrap();
        let json = json.clone();
        let namesjson = namesjson.clone();
        let nameconvention = nameconvention.clone();
        let version = version.clone();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let book = i.to_string();
                let book = json.get(&book).expect("file should have key");
                let chapters = book.get("chapters").expect("file should have key");
                let chapters = chapters.to_string();
                let chapters: u8 = chapters.parse().unwrap();
                let book = book.get("name").expect("file should have key");
                let book = book.to_string();
                let mut book = book.replace("\"", "");
                let mut bookname = book.clone();
                for book_info in namesjson.as_array().unwrap() {
                    if book_info["id"].as_str().unwrap() == &bookname
                        || book_info["name"].as_str().unwrap() == &bookname
                    {
                        if nameconvention == "Abkürzungen1" {
                            bookname = book_info["id"].as_str().unwrap().to_string();
                        } else if nameconvention == "Abkürzungen2" {
                            bookname = book_info["id2"].as_str().unwrap().to_string();
                        } else if nameconvention == "Deutsch1" {
                            bookname = book_info["german_evang"].as_str().unwrap().to_string();
                        } else if nameconvention == "Deutsch2" {
                            bookname = book_info["german_kath"].as_str().unwrap().to_string();
                        } else if nameconvention == "Englisch" {
                            bookname = book_info["english"].as_str().unwrap().to_string();
                        } else if nameconvention == "Latein" {
                            bookname = book_info["latin"].as_str().unwrap().to_string();
                        }
                    }
                }
                let dateiname = format!("{version}/{bookname}.md");
                let dir = format!("{version}");
                if !Path::new(&dir).exists() {
                    fs::create_dir(&dir).unwrap();
                }
                let mut file =
                    File::create(&dateiname).expect("Datei konnte leider nicht erstellt werden.");
                let mut ausgabe = " ".to_string();
                for n in 1..chapters + 1 {
                    let mut url = String::new();
                    let mut urlfin = String::new();
                    let mut text = String::new();
                    let replace_specialtags =
                        Regex::new(r#"<span class="d-sr-only"></span>"#).unwrap();
                    let replace_tags = Regex::new(r#"<.*?>"#).unwrap();
                    let replace_linebreaks = Regex::new(r#"\n"#).unwrap();
                    let replace_footnotes = Regex::new(r#".[0-9]]"#).unwrap();
                    if version == "BB" || version == "NA28" || version == "BHS" {
                        url = "https://www.die-bibel.de/bibel/".to_string();
                        urlfin = format!("{url}{version}/{book}.{n}");
                        let ergebnis: String = reqwest::get(&urlfin)
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap()
                            .to_string();
                        let ergebnis: Vec<&str> = ergebnis
                            .split("<ibep-bible-chapter _nghost-ibep-main-c4145848528>")
                            .collect();
                        let ergebnis: Vec<&str> =
                            ergebnis[1].split("</ibep-bible-chapter>").collect();
                        text = ergebnis[0].to_string();
                        text = text.replace("<br>", "\n");
                        text = text.replace("<bible-v", "\n<bible-v");
                        text = text.replace("</bible-v>", " </bible-v>");
                        text = text.replace("</ibep-bible-passage>", " </ibep-bible-passage>");
                        text = replace_specialtags.replace_all(&text, "").to_string();
                        text = replace_tags.replace_all(&text, "").to_string();
                        text = replace_linebreaks.replace_all(&text, "\n").to_string();
                        text = replace_footnotes.replace_all(&text, "").to_string();
                        text = text.trim().to_string();
                    } else {
                        url = "https://www.bibleserver.com/".to_string();
                        urlfin = format!("{url}{version}/{book}{n}");
                        let ergebnis: String = reqwest::get(&urlfin)
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap()
                            .to_string();
                        let ergebnis: Vec<&str> = ergebnis
                            .split("<header style=\"grid-row:1 / 2;\">")
                            .collect();
                        let ergebnis: Vec<&str> = ergebnis[1].split("<footer ").collect();
                        text = ergebnis[0].to_string();
                        text = replace_specialtags.replace_all(&text, "").to_string();
                        text = text.replace("</h1>", "</h1>++break++");
                        text = text.replace("</h2>", "</h2>++break++");
                        text = text.replace("</h3>", "<h3>++break++");
                        text = replace_tags.replace_all(&text, "").to_string();
                        text = replace_linebreaks.replace_all(&text, "").to_string();
                        text = replace_footnotes.replace_all(&text, "").to_string();
                        text = text.trim().to_string();
                        text = text.replace("\u{2}", "\n");
                        text = text.replace("\u{3}", "");
                        text = text.replace(" &#x1;", "\n## ");
                        text = text.replace("&#x1;", "\n## ");
                        text = text.replace("++break++", "\n");
                        text = text.trim().to_string();
                    }

                    let replace_verse_start = Regex::new(r"\n(?P<v>\d+)").unwrap();
                    let chapter_text = replace_verse_start
                        .replace_all(&text, "\n\n ###### $v \n\n")
                        .to_string();
                    let chapter_file = format!("{version}/{bookname}/{bookname} {n}.md");
                    pb.set_width(Some(66));
                    pb.message(format!("{} ", bookname).as_str());
                    pb.inc();
                    let dirchapter = format!("{version}/{bookname}");
                    if !Path::new(&dirchapter).exists() {
                        fs::create_dir(&dirchapter).unwrap();
                    }
                    let mut chapterfile = File::create(&chapter_file)
                        .expect("Datei konnte leider nicht erstellt werden.");
                    chapterfile
                        .write_all(&chapter_text.as_bytes())
                        .expect("Inhalt konnte leider nicht geschrieben werden!");

                    ausgabe.push_str(&text);
                    ausgabe.push_str("\n");
                }
                file.write_all(ausgabe.as_bytes())
                    .expect("Inhalt konnte leider nicht geschrieben werden!");
            });
    });
    let mut pb = pb.lock().unwrap();
    pb.finish();
    Ok(())
}
