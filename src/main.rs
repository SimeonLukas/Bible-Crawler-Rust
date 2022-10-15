use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welche Bibel soll es sein?");
    println!(
        "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
        "1. Einheitsübersetzung (EU)", "2. Lutherbibel (LUT)", "DBU", "ELB"
    );
    let mut version = String::new();
    let mut buch = String::new();

    io::stdin()
        .read_line(&mut version)
        .expect("Diese Version kenne ich nicht!");

    version = version.trim().to_string();

    if version == "1" || version == "EU" {
        println!("Die Einheitsüberstzung wurde gewählt!");
        version = "EU".to_string();
        io::stdin()
            .read_line(&mut version)
            .expect("Welches Buch möchtest du lesen?");
    }

    if version == "2" || version == "LUT" {
        println!("Die Lutherbibel wurde gewählt!");
        version = "LUT".to_string();
    }

    println!("{}", "Dokument wird erstellt!");
    let dateiname = format!("{version}-Bibel.md");
    let mut file = File::create(&dateiname).expect("Datei konnte leider nicht erstellt werden.");
    let mut ausgabe = " ".to_string();
    for n in 1..2 {
        let url = "https://www.bibleserver.com/";
        let urlfin = format!("{url}{version}/{n}");
        let ergebnis: String = reqwest::get(urlfin).await?.text().await?.to_string();
        let split: Vec<&str> = ergebnis
            .split("<header style=\"grid-row-start: 1;grid-row-end: 2\">")
            .collect();
        let split_2: Vec<&str> = split[1].split("<footer").collect();

        let replace_biblename = Regex::new(r#"<h1(.*)"#).unwrap();
        let replace_tags = Regex::new(r#"<.*?>"#).unwrap();
        let replace_linebreaks = Regex::new(r#"\n"#).unwrap();
        let m = split_2[0].trim();
        let ma = replace_biblename.replace_all(&m, "#");
        let mat = replace_tags.replace_all(&ma, "");
        let matc = replace_linebreaks.replace_all(&mat, "");
        println!("{:#?}", &matc);
        let matc = matc.trim();
        let matc = matc.replace("\u{2}", "\n");
        let matc = matc.replace("\u{3}", "");
        let matc = matc.replace(" &#x1;", "\n## ");
        let matc = matc.trim();
        ausgabe.push_str(&matc);
        ausgabe.push_str("\n");
    }
    file.write_all(ausgabe.as_bytes())
        .expect("Inhalt konnte leider nicht geschrieben werden!");
    println!("{}", "Dokument wurde erstellt!");
    Ok(())
}
