use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Dokument wird erstellt!");
    let mut file = File::create("Bibel.md").expect("Datei konnte leider nicht erstellt werden.");
    let mut ausgabe = " ".to_string();
    for n in 1..3 {
        let url = "https://www.bibleserver.com/EU/Psalmen";
        let urlfin = format!("{url}{n}");
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
