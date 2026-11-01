//! # Elemental Clash Game
//!
//! Game pertarungan taktis 5 elemen (Api, Es, Angin, Tanah, Petir) ronde vs AI komputer
//! dengan sistem interaksi pentagonal (Rock-Paper-Scissors-Lizard-Spock style).

use rand::seq::SliceRandom;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Api,   // Fire
    Es,    // Ice
    Angin, // Wind
    Tanah, // Earth
    Petir, // Lightning
}

impl Element {
    pub const ALL: [Element; 5] = [
        Element::Api,
        Element::Es,
        Element::Angin,
        Element::Tanah,
        Element::Petir,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Element::Api => "Api",
            Element::Es => "Es",
            Element::Angin => "Angin",
            Element::Tanah => "Tanah",
            Element::Petir => "Petir",
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Element {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Element::Api),
            "2" => Ok(Element::Es),
            "3" => Ok(Element::Angin),
            "4" => Ok(Element::Tanah),
            "5" => Ok(Element::Petir),
            _ => Err("Pilihan hanya 1-5!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClashResult {
    PlayerWin(&'static str),
    AiWin(&'static str),
    Draw,
}

pub fn evaluate_clash(player: Element, ai: Element) -> ClashResult {
    if player == ai {
        return ClashResult::Draw;
    }

    match (player, ai) {
        // Api
        (Element::Api, Element::Es) => ClashResult::PlayerWin("Api mengalahkan Es! PLAYER MENANG!"),
        (Element::Api, Element::Angin) => {
            ClashResult::PlayerWin("Api mengalahkan Angin! PLAYER MENANG!")
        }
        (Element::Api, Element::Tanah) => ClashResult::AiWin("Tanah mengalahkan Api! AI MENANG!"),
        (Element::Api, Element::Petir) => ClashResult::AiWin("Petir mengalahkan Api! AI MENANG!"),

        // Es
        (Element::Es, Element::Tanah) => ClashResult::PlayerWin("Es mengalahkan Tanah! PLAYER MENANG!"),
        (Element::Es, Element::Angin) => ClashResult::PlayerWin("Es mengalahkan Angin! PLAYER MENANG!"),
        (Element::Es, Element::Api) => ClashResult::AiWin("Api mengalahkan Es! AI MENANG!"),
        (Element::Es, Element::Petir) => ClashResult::AiWin("Petir mengalahkan Es! AI MENANG!"),

        // Angin
        (Element::Angin, Element::Tanah) => {
            ClashResult::PlayerWin("Angin mengalahkan Tanah! PLAYER MENANG!")
        }
        (Element::Angin, Element::Petir) => {
            ClashResult::PlayerWin("Angin mengalahkan Petir! PLAYER MENANG!")
        }
        (Element::Angin, Element::Api) => ClashResult::AiWin("Api mengalahkan Angin! AI MENANG!"),
        (Element::Angin, Element::Es) => ClashResult::AiWin("Es mengalahkan Angin! AI MENANG!"),

        // Tanah
        (Element::Tanah, Element::Petir) => {
            ClashResult::PlayerWin("Tanah mengalahkan Petir! PLAYER MENANG!")
        }
        (Element::Tanah, Element::Api) => {
            ClashResult::PlayerWin("Tanah mengalahkan Api! PLAYER MENANG!")
        }
        (Element::Tanah, Element::Es) => ClashResult::AiWin("Es mengalahkan Tanah! AI MENANG!"),
        (Element::Tanah, Element::Angin) => {
            ClashResult::AiWin("Angin mengalahkan Tanah! AI MENANG!")
        }

        // Petir
        (Element::Petir, Element::Api) => {
            ClashResult::PlayerWin("Petir mengalahkan Api! PLAYER MENANG!")
        }
        (Element::Petir, Element::Es) => ClashResult::PlayerWin("Petir mengalahkan Es! PLAYER MENANG!"),
        (Element::Petir, Element::Angin) => {
            ClashResult::AiWin("Angin mengalahkan Petir! AI MENANG!")
        }
        (Element::Petir, Element::Tanah) => {
            ClashResult::AiWin("Tanah mengalahkan Petir! AI MENANG!")
        }

        _ => unreachable!(),
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or_default();
    buffer.trim().to_string()
}

fn play_game() {
    let mut score_player = 0;
    let mut score_ai = 0;
    let mut rng = rand::thread_rng();

    println!("\n========================================");
    println!("          ELEMENTAL CLASH GAME          ");
    println!("========================================\n");

    for round in 1..=3 {
        println!("----------------------------------------");
        println!("                RONDE {}", round);
        println!("----------------------------------------");
        println!(
            "Skor Sementara -> Player: {} | AI: {}",
            score_player, score_ai
        );
        println!("Pilih Elemenmu:");
        println!("1. Api    (Mengalahkan: Es, Angin)");
        println!("2. Es     (Mengalahkan: Tanah, Angin)");
        println!("3. Angin  (Mengalahkan: Tanah, Petir)");
        println!("4. Tanah  (Mengalahkan: Petir, Api)");
        println!("5. Petir  (Mengalahkan: Api, Es)");

        let player_choice = loop {
            let input = read_line("Masukkan pilihan (1-5): ");
            match Element::from_str(&input) {
                Ok(elem) => break elem,
                Err(err) => println!("[Peringatan] {}", err),
            }
        };

        let ai_choice = *Element::ALL.choose(&mut rng).unwrap();

        println!("\nPlayer mengeluarkan : {}", player_choice);
        println!("AI mengeluarkan     : {}", ai_choice);

        match evaluate_clash(player_choice, ai_choice) {
            ClashResult::PlayerWin(msg) => {
                println!(">> {}", msg);
                score_player += 1;
            }
            ClashResult::AiWin(msg) => {
                println!(">> {}", msg);
                score_ai += 1;
            }
            ClashResult::Draw => {
                println!(">> Kedua elemen saling berbenturan! RONDE SERI!");
            }
        }
        println!();
    }

    println!("========================================");
    println!("            PERTANDINGAN SELESAI        ");
    println!("========================================");
    println!("Skor Akhir -> Player: {} | AI: {}", score_player, score_ai);
    if score_player > score_ai {
        println!("🏆 SELAMAT! Kamu MEMENANGKAN Elemental Clash!");
    } else if score_ai > score_player {
        println!("💀 DEFEAT! AI memenangkan pertandingan ini.");
    } else {
        println!("🤝 HASIL SERI! Kedua petarung seimbang.");
    }
    println!("========================================\n");
}

fn main() {
    loop {
        println!("=== ELEMENTAL CLASH ===");
        println!("1. Mulai Game (3 Ronde)");
        println!("0. Keluar");
        let choice = read_line(">> ");

        match choice.as_str() {
            "1" => play_game(),
            "0" => {
                println!("Terima kasih telah bermain Elemental Clash!");
                break;
            }
            _ => println!("[Error] Pilihan harus 1 atau 0!\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_matchups() {
        assert!(matches!(
            evaluate_clash(Element::Api, Element::Es),
            ClashResult::PlayerWin(_)
        ));
        assert!(matches!(
            evaluate_clash(Element::Api, Element::Angin),
            ClashResult::PlayerWin(_)
        ));
        assert!(matches!(
            evaluate_clash(Element::Api, Element::Tanah),
            ClashResult::AiWin(_)
        ));
        assert!(matches!(
            evaluate_clash(Element::Api, Element::Petir),
            ClashResult::AiWin(_)
        ));
        assert_eq!(evaluate_clash(Element::Api, Element::Api), ClashResult::Draw);
    }

    #[test]
    fn test_all_elements_have_two_wins_and_two_losses() {
        for &p in &Element::ALL {
            let mut wins = 0;
            let mut losses = 0;
            let mut draws = 0;

            for &ai in &Element::ALL {
                match evaluate_clash(p, ai) {
                    ClashResult::PlayerWin(_) => wins += 1,
                    ClashResult::AiWin(_) => losses += 1,
                    ClashResult::Draw => draws += 1,
                }
            }

            assert_eq!(wins, 2, "Setiap elemen harus menang melawan 2 elemen");
            assert_eq!(losses, 2, "Setiap elemen harus kalah melawan 2 elemen");
            assert_eq!(draws, 1, "Setiap elemen harus seri melawan dirinya sendiri");
        }
    }
}
