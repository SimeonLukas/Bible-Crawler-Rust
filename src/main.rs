use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use regex::Regex;
use serde_json;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

fn main() {
    crawler();
}

fn crawler() {
    let _ = start();
    let mut entscheidung = String::new();
    println!("\nNochmal? J/N");
    io::stdin()
        .read_line(&mut entscheidung)
        .expect("Diese Version kenne ich nicht!");

    entscheidung = entscheidung.trim().to_string();

    if entscheidung == "N" || entscheidung == "n" {
        // No need to shutdown the global thread pool
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
    let json;
    let mut version = String::new();
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
    println!("7. Abkürzungen Einheitsübersetzung (1Sam, 1Makk, Weish, etc.)");

    let mut name = String::new();
    let mut replace_link = false;
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
    } else if name == "7" {
        name = "AbkürzungenEU".to_string();
        println!("Abkürzungen Einheitsübersetzung wurde gewählt.");

        let mut replace_link_str = String::new();
        println!("\nSollen die Verweise durch Wikilinks ersetzt werden? J/N");
        io::stdin()
            .read_line(&mut replace_link_str)
            .expect("Diese Version kenne ich nicht!");
        replace_link_str = replace_link_str.trim().to_string();

        if replace_link_str == "J"
            || replace_link_str == "j"
            || replace_link_str == "Y"
            || replace_link_str == "y"
        {
            replace_link = true;
        }
    } else {
        name = "Abkürzungen".to_string();
        println!("Da keine Namenskonvention gewählt wurde, wurden Abkürzungen ausgewählt.");
    }
    let nameconvention = name.clone();

    println!("\nWelche Dateien sollen erzeugt werden?");
    println!("1 Nur die Bücher (Gen, Exo, Lev, Num, Deu, etc.)");
    println!("2 Nur die Kapitel (Gen 1, Gen 2, Gen 3, etc.)");
    println!("3 Die Kapitel und die Bücher (Gen, Gen1, Gen2, etc.)");
    println!("4 Nur die Verse (Gen 1,1; Gen 1,2; Gen 1,3; etc.)");
    println!("5 Die Verse und die Bücher (Gen; Gen 1,1; Gen 1,2; Gen 1,3; etc.)");
    println!("6 Die Verse und die Kapitel (Gen 1; Gen 1,1; Gen 1,2; Gen 1,3; etc.)");
    println!("7 Die Verse, die Kapitel und die Bücher (Gen; Gen 1; Gen 1,1; Gen 1,2; etc.)");

    let mut create_files = String::new();
    let mut create_book = false;
    let mut create_chapter = false;
    let mut create_vers = false;
    io::stdin()
        .read_line(&mut create_files)
        .expect("Diese Variante kenne ich nicht!");
    // clean up input
    create_files = create_files.trim().to_string();

    if create_files == "1" {
        create_book = true;
        println!("Nur die Bücher wurden gewählt.");
    } else if create_files == "2" {
        create_chapter = true;
        println!("Nur die Kapitel wurden gewählt.");
    } else if create_files == "3" {
        create_book = true;
        create_chapter = true;
        println!("Kapitel und Bücher wurden gewählt.");
    } else if create_files == "4" {
        create_vers = true;
        println!("Nur die Verse wurden gewählt.");
    } else if create_files == "5" {
        create_vers = true;
        create_book = true;
        println!("Verse und Bücher wurden gewählt.");
    } else if create_files == "6" {
        create_vers = true;
        create_chapter = true;
        println!("Verse und Kapitel wurden gewählt.");
    } else if create_files == "7" {
        create_vers = true;
        create_chapter = true;
        create_book = true;
        println!("Verse, Kapitel und Bücher wurden gewählt.");
    } else {
        println!("Da keine Variante gewählt wurde, wird Kapitel und Bücher ausgewählt");
        create_book = true;
        create_chapter = true;
    }

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
    let pb = Arc::new(Mutex::new(ProgressBar::new(total_chapters as u64)));
    pb.lock().unwrap().set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {pos}/{len} ({eta}) {elapsed_precise} {msg}")
            .unwrap(),
    );
    v.par_iter().for_each(|&i| {
        let pb = pb.clone();
        let json = json.clone();
        let namesjson = namesjson.clone();
        let nameconvention = nameconvention.clone();
        let version = version.clone();
        Runtime::new().unwrap().block_on(async move {
            let book = i.to_string();
            let book = json.get(&book).expect("file should have key");
            let chapters = book.get("chapters").expect("file should have key");
            let chapters = chapters.to_string();
            let chapters: u8 = chapters.parse().unwrap();
            let book = book.get("name").expect("file should have key");
            let book = book.to_string();
            let book = book.replace("\"", "");
            let mut bookname = book.clone();
            for book_info in namesjson.as_array().unwrap() {
                if book_info["id"].as_str().unwrap() == &bookname
                    || book_info["name"].as_str().unwrap() == &bookname
                {
                    if nameconvention == "Abkürzungen1" {
                        bookname = book_info["id"].as_str().unwrap().to_string();
                    } else if nameconvention == "Abkürzungen2" {
                        bookname = book_info["id2"].as_str().unwrap().to_string();
                    } else if nameconvention == "AbkürzungenEU" {
                        bookname = book_info["id_eu"].as_str().unwrap().to_string();
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
            let dir = format!("{version}");
            if !Path::new(&dir).exists() {
                let _ = std::fs::create_dir(&dir).ok();
            }

            let mut ausgabe = " ".to_string();
            for n in 1..chapters + 1 {
                let url;
                let urlfin;
                let mut text;
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
                    let ergebnis: Vec<&str> = ergebnis[1].split("</ibep-bible-chapter>").collect();
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
                    let mut ergebnis: Vec<&str> = ergebnis.split("<header style=").collect();
                    ergebnis = ergebnis[1].split("<footer").collect();
                    text = ergebnis[0].to_string();
                    text = "<".to_string() + &text;
                    text = replace_specialtags.replace_all(&text, "").to_string();
                    text = text.replace("</h1>", "</h1>++break++");
                    text = text.replace("</h2>", "</h2>++break++");
                    text = text.replace("</h3>", "<h3>++break++");
                    text = replace_tags.replace_all(&text, "").to_string();
                    text = replace_linebreaks.replace_all(&text, "").to_string();
                    text = replace_footnotes.replace_all(&text, "").to_string();
                    if version == "EU" {
                        text = text.replace("Einheitsübersetzung 2016", "");
                        text = text.replace("von Katholisches Bibelwerk", "");
                    }
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

                // Test zusätzliche Ausgabe in Versen
                if create_vers {
                    let dirchapter = format!("{version}/{bookname}");
                    if !Path::new(&dirchapter).exists() {
                        let _ = std::fs::create_dir(&dirchapter).unwrap();
                    }

                    if create_chapter {
                        let dirverse = format!("{version}/{bookname}/{bookname} {n}");
                        if !Path::new(&dirverse).exists() {
                            let _ = std::fs::create_dir(&dirverse).unwrap();
                        }
                    }

                    let mut v_num = String::new();
                    for line in chapter_text.lines() {
                        if line.is_empty() {
                            continue;
                        } else if line.starts_with(" ###### ") {
                            v_num = line.to_string();
                            v_num = v_num.replace(" ###### ", "");
                            v_num = v_num.trim().to_string();
                        } else if !v_num.is_empty() {
                            let mut verse_text = line.to_string();
                            verse_text = verse_text.trim().to_string();

                            if replace_link {
                                let mut wikilinks_parts = Vec::new();

                                // Alle "(...)"-Blöcke am Ende einsammeln
                                while verse_text.ends_with(')') {
                                    if let Some(start_idx) = verse_text.rfind('(') {
                                        let part = verse_text[start_idx..].to_string();

                                        if part.starts_with('(') && part.ends_with(')') {
                                            wikilinks_parts.push(part);
                                            verse_text.truncate(start_idx);
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }

                                wikilinks_parts.reverse();
                                let mut seen = HashSet::new();
                                let mut ordered_links = Vec::new();

                                for wikilinks in wikilinks_parts {
                                    if wikilinks.contains(';') {
                                        let inner =
                                            wikilinks.trim_start_matches('(').trim_end_matches(')');

                                        for part in inner.split(';') {
                                            let part = part.trim();
                                            if !part.is_empty() && seen.insert(part.to_string()) {
                                                ordered_links.push(part.to_string());
                                            }
                                        }
                                    } else {
                                        let inner =
                                            wikilinks.trim_start_matches('(').trim_end_matches(')');

                                        if !inner.is_empty() && seen.insert(inner.to_string()) {
                                            ordered_links.push(inner.to_string());
                                        }
                                    }
                                }

                                // Links wieder anhängen (dedupliziert)
                                verse_text = verse_text.trim().to_string();
                                for link in ordered_links {
                                    verse_text.push_str(&format!(" [[{}]]", link));
                                }
                            }

                            let verse_file;
                            if create_chapter {
                                verse_file = format!(
                                    "{version}/{bookname}/{bookname} {n}/{bookname} {n},{v_num}.md"
                                );
                            } else {
                                verse_file =
                                    format!("{version}/{bookname}/{bookname} {n},{v_num}.md");
                            }
                            let mut versefile = File::create(&verse_file)
                                .expect("Datei konnte leider nicht erstellt werden.");
                            versefile
                                .write_all(&verse_text.as_bytes())
                                .expect("Inhalt konnte leider nicht geschrieben werden!");
                        }
                    }
                }
                // Ausgabe in Versen Ende

                if create_chapter {
                    let chapter_file = format!("{version}/{bookname}/{bookname} {n}.md");
                    let dirchapter = format!("{version}/{bookname}");
                    if !Path::new(&dirchapter).exists() {
                        let _ = std::fs::create_dir(&dirchapter).unwrap();
                    }

                    let mut chapterfile = File::create(&chapter_file)
                        .expect("Datei konnte leider nicht erstellt werden.");
                    chapterfile
                        .write_all(&chapter_text.as_bytes())
                        .expect("Inhalt konnte leider nicht geschrieben werden!");
                }

                ausgabe.push_str(&text);
                ausgabe.push_str("\n");

                let pb = pb.lock().unwrap();
                pb.set_message(format!("{} Kapitel {}", bookname, n));
                pb.inc(1);
            }

            if create_book {
                let dateiname = format!("{version}/{bookname}.md");
                let mut bookfile =
                    File::create(&dateiname).expect("Datei konnte leider nicht erstellt werden.");
                bookfile
                    .write_all(ausgabe.as_bytes())
                    .expect("Inhalt konnte leider nicht geschrieben werden!");
            }
        });
    });
    let pb = pb.lock().unwrap();
    pb.finish_with_message("Fertig!");
    Ok(())
}
