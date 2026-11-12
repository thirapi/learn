use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 6,
        number: 7,
        title: "Teori Fungsi",
        subtitle: "Injektif, Surjektif, Bijektif, & Komposisi",
        icon: "07",
        chunks: vec![
            Chunk {
                id: "7.1",
                title: "Definisi Fungsi & Mesin Vending",
                reading_time: "4 menit",
                diffuse_content: r#"# Analogi: Mesin Penjual Minuman Otomatis (Vending Machine)

Bayangkan sebuah vending machine:
- Tombol-tombol pilihan di panel adalah **Domain (Daerah Asal)**.
- Seluruh minuman yang ada di dalam lemari mesin adalah **Kodomain (Daerah Kawan)**.
- Minuman yang benar-benar bisa keluar saat tombol ditekan adalah **Range (Daerah Hasil)**.

### Apa Syarat Mutlak Disebut Fungsi?
1. **Setiap tombol harus memiliki fungsi**: Kamu tidak boleh menekan tombol yang tidak terhubung ke apapun (semua elemen domain harus punya pasangan).
2. **Satu tombol tidak boleh mengeluarkan dua minuman berbeda sekaligus secara acak**: Jika kamu tekan tombol 'A1' (Kopi), mesin harus konsisten mengeluarkan Kopi, bukan tiba-tiba Kopi dan Jus bersamaan!

> **Kunci Fungsi**: **Setiap anggota di Domain WAJIB punya pasangan, dan pasangannya HARUS TEPAT SATU di Kodomain!**"#,
                focused_content: r#"# Definisi Formal: Fungsi / Pemetaan

Misalkan A dan B adalah himpunan. **Fungsi (*Function / Mapping*)** dari A ke B, dinotasikan f: A → B, adalah relasi khusus yang memasangkan **setiap** elemen a ∈ A dengan **tepat satu** elemen b ∈ B.

Jika f(a) = b, maka:
- b disebut **bayangan (*image*)** dari a oleh f.
- a disebut **prapeta (*pre-image*)** dari b.

### Terminologi:
1. **Domain (Daerah Asal)**: Himpunan A.
2. **Kodomain (Daerah Kawan)**: Himpunan B.
3. **Range (Daerah Hasil / R_f)**: Himpunan seluruh bayangan yang dihasilkan:
   

> **R_f = {f(a) | a ∈ A} ⊆ B**

### Bukan Fungsi Jika:
- Ada elemen di A yang tidak memiliki pasangan sama sekali (jomblo).
- Ada elemen di A yang memiliki lebih dari satu pasangan di B (bercabang / mendua)."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Manakah relasi dari A = {1, 2, 3} ke B = {a, b} yang MEMENUHI syarat sebagai FUNGSI?",
                        options: [
                            "R = {(1, a), (2, b)}",
                            "R = {(1, a), (1, b), (2, a), (3, b)}",
                            "R = {(1, a), (2, b), (3, a)}",
                            "R = {(2, a), (3, b)}"
                        ],
                        correct_idx: 2,
                        explanation: "Pilihan ketiga memasangkan SETIAP elemen A (1, 2, 3) tepat satu kali ke B. Pilihan 1 & 4 tidak lengkap (elemen 3/1 tidak punya pasangan), pilihan 2 mendua (elemen 1 dipasangkan dua kali ke a dan b)."
                    }
                ],
                key_concepts: &["Fungsi (Mapping)", "Domain (Daerah Asal)", "Kodomain (Daerah Kawan)", "Range (Daerah Hasil)", "Syarat Tepat Satu"],
            },
            Chunk {
                id: "7.2",
                title: "Injektif, Surjektif, & Bijektif",
                reading_time: "5 menit",
                diffuse_content: r#"# Analogi Kursi Bioskop & Tiket

Bayangkan penonton (Domain) dan kursi bioskop (Kodomain):

### 1. Injektif (Satu-ke-Satu / One-to-One):
- *Aturan*: Tidak boleh ada dua penonton duduk di satu kursi yang sama!
- Setiap orang punya kursinya masing-masing secara unik. (Mungkin ada kursi kosong yang tidak diduduki siapapun, tapi yang penting tidak ada kursi yang diperebutkan).

### 2. Surjektif (Pada / Onto):
- *Aturan*: Bioskop harus **FULL BOOKED**! Semua kursi terisi penuh oleh penonton, tidak ada kursi yang kosong. (Boleh saja ada anak kecil yang dipangku orang tuanya di satu kursi, yang penting tidak ada kursi nganggur).

### 3. Bijektif (Korespondensi Satu-ke-Satu):
- *Kombinasi Sempurna*: Setiap orang dapat 1 kursi sendiri (Injektif) **DAN** seluruh kursi terisi penuh pas tanpa sisa (Surjektif)!
- Jumlah penonton sama persis dengan jumlah kursi (|A| = |B|)."#,
                focused_content: r#"# 3 Sifat Khusus Fungsi

### 1. Fungsi Satu-ke-Satu (*Injektif / One-to-One*)
Fungsi f: A → B disebut injektif jika elemen-elemen yang berbeda di domain dipetakan ke elemen-elemen yang berbeda di kodomain:

> **∀ a₁, a₂ ∈ A, \; a₁ ≠ a₂ ⇒ f(a₁) ≠ f(a₂)**

Atau secara kontraposisi:

> **f(a₁) = f(a₂) ⇒ a₁ = a₂**

### 2. Fungsi Pada (*Surjektif / Onto*)
Fungsi f: A → B disebut surjektif jika setiap elemen di kodomain B memiliki prapeta di domain A:

> **∀ b ∈ B, \; ∃ a ∈ A  sehingga  f(a) = b**

Dengan kata lain: **Range = Kodomain** (R_f = B).

### 3. Fungsi Korespondensi Satu-ke-Satu (*Bijektif / Bijective*)
Fungsi f: A → B disebut bijektif jika dan hanya jika f adalah **Injektif SEKALIGUS Surjektif**.

> **Syarat Mutlak Fungsi Invers**:
> Fungsi invers f⁻¹: B → A **HANYA ADA** jika fungsi f bersifat **BIJEKTIF**!"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Jika sebuah fungsi memiliki sifat Range = Kodomain (seluruh elemen kodomain memiliki pasangan), fungsi tersebut bersifat...",
                        options: [
                            "Injektif (One-to-One)",
                            "Surjektif (Onto)",
                            "Refleksif",
                            "Kontradiksi"
                        ],
                        correct_idx: 1,
                        explanation: "Fungsi Surjektif (pada/onto) adalah fungsi di mana seluruh elemen di daerah kawan (kodomain) terpetakan atau menjadi range (Range = Kodomain)."
                    },
                    QuizQuestion {
                        question: "Kapan sebuah fungsi f: A -> B dijamin memiliki fungsi invers (f^-1)?",
                        options: [
                            "Hanya saat f bersifat Injektif saja",
                            "Hanya saat f bersifat Surjektif saja",
                            "Saat f bersifat Bijektif (Injektif dan Surjektif sekaligus)",
                            "Semua fungsi selalu memiliki invers"
                        ],
                        correct_idx: 2,
                        explanation: "Fungsi invers hanya terdefinisi dengan sah jika fungsinya Bijektif, karena jika dibalik arahnya dari B ke A, ia harus tetap memenuhi syarat fungsi (semua elemen B punya pasangan tepat satu di A)."
                    }
                ],
                key_concepts: &["Injektif (One-to-One)", "Surjektif (Onto)", "Bijektif (Korespondensi Satu-Satu)", "Syarat Keberadaan Invers"],
            },
            Chunk {
                id: "7.3",
                title: "Fungsi Invers & Komposisi Fungsi",
                reading_time: "4 menit",
                diffuse_content: r#"# Analogi: Pipa Pengerjaan Pabrik (Pipelining)

Bayangkan dua mesin di pabrik:
- Mesin g: Mengupas apel (Input apel utuh → apel kupas).
- Mesin f: Memotong apel (Input apel kupas → potongan apel).

Jika kedua mesin digabung menjadi satu saluran pabrik (*pipeline*):
Kamu masukkan apel utuh, apel dikupas oleh g, lalu langsung dipotong oleh f.
Secara matematika, ini ditulis sebagai:

> **(f ∘ g)(x) = f(g(x))**

*(Perhatikan urutannya: g dijalankan terlebih dahulu, baru kemudian hasilnya dimasukkan ke f!)*

Di dunia pemrograman, ini sama persis dengan konsep **Pipelining / Function Chaining** pada JavaScript atau Rust:
`apel.pipe(kupas).pipe(potong)`!"#,
                focused_content: r#"# Komposisi & Invers Fungsi

### 1. Komposisi Fungsi (f ∘ g)
Misalkan g: A → B dan f: B → C. Komposisi dari fungsi f dan g, dinotasikan (f ∘ g): A → C, didefinisikan oleh:

> **(f ∘ g)(a) = f(g(a))**

- **Catatan Urutan**: (f ∘ g)(x) berarti fungsi g diterapkan terlebih dahulu ke x, kemudian hasilnya diproses oleh fungsi f.
- **Sifat Komposisi**:
  - Umumnya **TIDAK Komutatif**: f ∘ g ≠ g ∘ f.
  - Bersifat **Asosiatif**: (f ∘ (g ∘ h)) = ((f ∘ g) ∘ h).

### 2. Fungsi Invers (f⁻¹)
Jika f: A → B adalah fungsi bijektif, maka invers dari f adalah fungsi f⁻¹: B → A sedemikian sehingga:

> **f⁻¹(b) = a ⇔ f(a) = b**

Hubungan dengan Fungsi Identitas (I):

> **(f⁻¹ ∘ f)(a) = a   dan   (f ∘ f⁻¹)(b) = b**

"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Jika g(x) = 2x + 1 dan f(x) = x - 3, berapakah hasil dari komposisi (f ∘ g)(2)?",
                        options: [
                            "2",
                            "5",
                            "1",
                            "7"
                        ],
                        correct_idx: 0,
                        explanation: "Hitung terlebih dahulu g(2): g(2) = 2(2) + 1 = 5. Kemudian masukkan ke f: f(5) = 5 - 3 = 2."
                    }
                ],
                key_concepts: &["Komposisi Fungsi (f ∘ g)", "Pipelining Komputasi", "Fungsi Invers (f^-1)", "Fungsi Identitas (I)"],
            }
        ],
    }
}
