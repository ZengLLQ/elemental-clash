# 🌪️ Elemental Clash Game (`elemental-clash`)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Game pertarungan taktis 5 elemen (Api, Es, Angin, Tanah, Petir) ronde bertahap melawan AI komputer dalam **Rust**.

Dikonversi dan di-*overhaul* dari tugas praktikum C++ menjadi arsitektur evaluasi *pattern matching* Rust yang elegan dan teruji secara matematis (*rock-paper-scissors-lizard-spock variant*).

---

## 🌀 Matriks Keunggulan Elemen

Setiap elemen memiliki tepat 2 keunggulan dan 2 kelemahan:

| Elemen | Menang Melawan | Kalah Melawan |
| :---: | :--- | :--- |
| **Api** | Es, Angin | Tanah, Petir |
| **Es** | Tanah, Angin | Api, Petir |
| **Angin** | Tanah, Petir | Api, Es |
| **Tanah** | Petir, Api | Es, Angin |
| **Petir** | Api, Es | Angin, Tanah |

---

## 🎮 Format Permainan

- Pertandingan berjalan dalam **3 ronde**.
- Setiap ronde, skor akumulatif ditampilkan.
- Di akhir ronde ke-3, pemenang utama ditentukan berdasarkan total poin kemenangan.

---

## 🛠️ Cara Menjalankan

```bash
git clone https://github.com/ZengLLQ/elemental-clash.git
cd elemental-clash

# Jalankan unit tests
cargo test

# Mainkan game
cargo run
```

---

## 📜 Lisensi

Proyek ini dilisensikan di bawah lisensi [MIT](LICENSE).  
Dibuat ulang oleh **Jonathan Cahyadi** (*Overhauled College Project*).
