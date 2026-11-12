use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 1,
        number: 2,
        title: "Logika Proposisi Bagian 1",
        subtitle: "Pernyataan, Operator Logika, & Kuantor",
        icon: "02",
        chunks: vec![
            Chunk {
                id: "2.1",
                title: "Apa Itu Proposisi?",
                reading_time: "3 menit",
                diffuse_content: r#"# Analogi: Pertanyaan vs Fakta yang Bisa Diuji

Bayangkan kamu sedang mengobrol dengan seorang teman:
- Jika temanmu berkata: *"Semarang adalah ibukota Jawa Tengah."* → Kamu bisa mengeceknya di Google Maps dan langsung tahu kalimat ini bernilai **BENAR (True)**.
- Jika temanmu berkata: *"Tolong ambilkan minum!"* → Apakah kalimat ini benar atau salah? Tidak masuk akal menilainya! Ini kalimat perintah, bukan fakta.
- Jika temanmu berkata: *"Kopi ini enak sekali."* → Ini adalah opini subjektif, bukan fakta matematika yang bernilai pasti benar atau salah.

Dalam logika matematika, kita hanya peduli pada kalimat deklaratif yang memiliki nilai kepastian mutlak: **HANYA BISA BENAR ATAU SALAH**, tidak keduanya dan bukan opini."#,
                focused_content: r#"# Definisi Formal: Proposisi & Nilai Kebenaran

**Proposisi (*Proposition*)** adalah kalimat deklaratif (*declarative sentence*) yang bernilai **Benar (*True / T / B*)** atau **Salah (*False / F / S*)**, tetapi tidak dapat bernilai keduanya sekaligus.

Nilai kebenaran suatu proposisi dilambangkan dengan T (True/1) atau F (False/0).

### Contoh Proposisi:
1. p : 2 + 5 = 7 (Proposisi, bernilai **Benar**)
2. q : 4  adalah bilangan ganjil (Proposisi, bernilai **Salah**)
3. r : Soekarno adalah presiden pertama RI (Proposisi, bernilai **Benar**)

### Bukan Proposisi:
1. *"Jam berapa sekarang?"* (Kalimat tanya)
2. *"Serahkan tugasmu sekarang juga!"* (Kalimat perintah)
3. *x + 3 = 8* (Bukan proposisi karena nilai kebenarannya belum pasti, bergantung pada nilai variabel x. Ini disebut **Kalimat Terbuka**).

### Negasi / Ingkaran (¬ p atau ¬ p):
Jika p adalah proposisi, maka negasi dari p adalah pernyataan *"Tidak benar bahwa p"*.
- Jika p bernilai **Benar**, maka ¬ p bernilai **Salah**.
- Jika p bernilai **Salah**, maka ¬ p bernilai **Benar**."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Manakah di antara kalimat berikut yang merupakan PROPOSISI?",
                        options: [
                            "Tolong tutup pintu itu sekarang!",
                            "Berapa hasil dari 5 dikalikan 10?",
                            "Angka 17 adalah bilangan prima.",
                            "Semoga hari ini tidak hujan."
                        ],
                        correct_idx: 2,
                        explanation: "'Angka 17 adalah bilangan prima' adalah kalimat deklaratif yang jelas bernilai Benar (True), sehingga memenuhi syarat proposisi. Pilihan lain adalah perintah, pertanyaan, dan harapan."
                    },
                    QuizQuestion {
                        question: "Jika kalimat p adalah 'Hari ini hujan', manakah negasi yang benar dari p?",
                        options: [
                            "Hari ini panas terik dan kemarau",
                            "Tidak benar bahwa hari ini hujan (Hari ini tidak hujan)",
                            "Kemarin hari hujan",
                            "Besok mungkin hujan"
                        ],
                        correct_idx: 1,
                        explanation: "Negasi dari p adalah ingkaran langsungnya: 'Tidak benar bahwa hari ini hujan' atau 'Hari ini tidak hujan'."
                    }
                ],
                key_concepts: &["Proposisi", "Kalimat Deklaratif", "Kalimat Terbuka", "Nilai Kebenaran T/F", "Negasi"],
            },
            Chunk {
                id: "2.2",
                title: "Operator Majemuk & Analogi Janji",
                reading_time: "4 menit",
                diffuse_content: r#"# Memahami Implikasi (p → q) Lewat Analogi Janji Orang Tua

Seringkali operator logika yang paling membingungkan adalah **Implikasi (p → q)**: *"Jika p, maka q"*.

Untuk memahaminya secara alami, bayangkan seorang ayah berjanji kepada anaknya:
> *"Jika kamu ranking 1 (p), Ayah belikan sepeda baru (q)."*

Kapan ayah dianggap **PEMBOHONG (False)**?
1. Kamu ranking 1 (T), Ayah belikan sepeda (T) → **Jujur (T)**
2. Kamu ranking 1 (T), Ayah **INGKAR JANJI** tidak belikan sepeda (F) → **PEMBOHONG (F)**!
3. Kamu **TIDAK** ranking 1 (F), tapi Ayah tetap belikan sepeda karena ada rezeki (T) → Ayah **TIDAK BOHONG (T)**, karena janji tadi hanya berlaku jika kamu ranking 1!
4. Kamu tidak ranking 1 (F), Ayah tidak belikan (F) → Tetap **JUJUR (T)**!

> **Kunci Emas Implikasi**: p → q **HANYA SALAH** jika anteseden p Benar tapi konsekuen q Salah (T → F = F)!"#,
                focused_content: r#"# Operator Logika Majemuk

1. **Konjungsi (p ∧ q / 'DAN')**:
   - Bernilai **Benar (T)** hanya jika kedua proposisi p dan q sama-sama Benar.
2. **Disjungsi (p ∨ q / 'ATAU')**:
   - Bernilai **Salah (F)** hanya jika kedua proposisi p dan q sama-sama Salah.
3. **Implikasi (p → q / 'JIKA-MAKA')**:
   - p disebut Anteseden / Hipotesis; q disebut Konsekuen / Kesimpulan.
   - Bernilai **Salah (F)** hanya ketika p = T dan q = F.
4. **Biimplikasi (p ↔ q / 'JIKA DAN HANYA JIKA')**:
   - Bernilai **Benar (T)** jika p dan q memiliki nilai kebenaran yang **SAMA** (T ↔ T = T, F ↔ F = T).

### Tabel Kebenaran Gabungan:
| p | q | p ∧ q | p ∨ q | p → q | p ↔ q |
| :-: | :-: | :---------: | :--------: | :-------: | :-------------------: |
| T | T | **T** | **T** | **T** | **T** |
| T | F | **F** | **T** | **F** | **F** |
| F | T | **F** | **T** | **T** | **F** |
| F | F | **F** | **F** | **T** | **T** |"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Diketahui p bernilai SALAH dan q bernilai BENAR. Apakah nilai dari implikasi p -> q?",
                        options: [
                            "SALAH",
                            "BENAR",
                            "Tidak terdefinisi",
                            "Bisa benar bisa salah"
                        ],
                        correct_idx: 1,
                        explanation: "Jika anteseden bernilai SALAH (F -> T), maka implikasi secara otomatis bernilai BENAR (T). Ingat analogi janji: jika syarat tidak terpenuhi, pemberi janji tidak bisa dituduh berbohong."
                    },
                    QuizQuestion {
                        question: "Kapan proposisi biimplikasi p <-> q bernilai BENAR?",
                        options: [
                            "Hanya saat p dan q sama-sama bernilai Salah",
                            "Hanya saat p bernilai Benar dan q bernilai Salah",
                            "Saat p dan q memiliki nilai kebenaran yang sama (keduanya Benar atau keduanya Salah)",
                            "Hanya saat p bernilai Salah dan q bernilai Benar"
                        ],
                        correct_idx: 2,
                        explanation: "Biimplikasi bernilai Benar jika kedua pernyataan ekuivalen / memiliki nilai kebenaran yang sama (T <-> T = T, F <-> F = T)."
                    }
                ],
                key_concepts: &["Konjungsi (AND)", "Disjungsi (OR)", "Implikasi (IF-THEN)", "Biimplikasi (IFF)", "Tabel Kebenaran"],
            },
            Chunk {
                id: "2.3",
                title: "Kuantor & Tautologi vs Kontradiksi",
                reading_time: "4 menit",
                diffuse_content: r#"# Logika Sehari-hari: 'Semua' vs 'Ada'

Perhatikan dua kalimat ini:
1. *"Semua mahasiswa di kelas ini memakai kacamata."* (Kuantor Universal ∀)
2. *"Ada mahasiswa di kelas ini yang memakai kacamata."* (Kuantor Eksistensial ∃)

### Bagaimana Cara Membuktikan Pernyataan 'Semua'?
Untuk membuktikan *"Semua mahasiswa memakai kacamata"* itu SALAH, apakah kamu harus memeriksa seluruh mahasiswa?
**Cukup temukan 1 orang saja yang tidak memakai kacamata!** Satu contoh penyangkal (*counterexample*) langsung menggugurkan klaim 'semua'.

Itulah mengapa:

> **Negasi dari "SEMUA  P(x)" adalah "ADA  x  yang TIDAK  P(x)"**

### Tautologi vs Kontradiksi
- **Tautologi**: Kalimat yang selalu benar dalam situasi apapun (contoh: *"Besok hujan atau besok tidak hujan"* → p ∨ ¬ p).
- **Kontradiksi**: Kalimat yang mustahil benar (contoh: *"Hari ini hujan dan hari ini tidak hujan"* → p ∧ ¬ p)."#,
                focused_content: r#"# Kuantor & Ekuivalensi Logika

### 1. Kuantor Universal (∀)
- Simbol ∀ dibaca *"untuk semua"* atau *"untuk setiap"*.
- ∀ x, P(x) bernilai Benar jika dan hanya jika P(x) bernilai Benar untuk **seluruh** nilai x dalam semesta pembicaraan.

### 2. Kuantor Eksistensial (∃)
- Simbol ∃ dibaca *"ada"*, *"terdapat"*, atau *"sekurang-kurangnya satu"*.
- ∃ x, P(x) bernilai Benar jika terdapat **minimal satu** x sehingga P(x) Benar.

### 3. Hukum Negasi Kuantor:

> **¬(∀ x, P(x)) ≡ ∃ x, ¬ P(x)**

> **¬(∃ x, P(x)) ≡ ∀ x, ¬ P(x)**

### 4. Tautologi & Kontradiksi:
- **Tautologi**: Suatu proposisi majemuk yang selalu bernilai T untuk seluruh kombinasi nilai kebenaran komponennya (contoh: p ∨ ¬ p).
- **Kontradiksi**: Suatu proposisi majemuk yang selalu bernilai F untuk seluruh kombinasi nilai kebenaran komponennya (contoh: p ∧ ¬ p).
- **Kontinjensi**: Proposisi yang bukan tautologi dan bukan kontradiksi."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Manakah negasi dari kalimat: 'Semua mahasiswa rajin belajar'?",
                        options: [
                            "Semua mahasiswa malas belajar",
                            "Tidak ada satupun mahasiswa yang rajin",
                            "Ada beberapa mahasiswa yang tidak rajin belajar",
                            "Hanya satu mahasiswa yang rajin belajar"
                        ],
                        correct_idx: 2,
                        explanation: "Negasi dari kuantor universal (∀x P(x)) adalah kuantor eksistensial dengan ingkaran predikat: ∃x ¬P(x), yaitu 'Ada mahasiswa yang tidak rajin belajar'."
                    },
                    QuizQuestion {
                        question: "Bentuk proposisi majemuk (p OR ~p) disebut sebagai...",
                        options: [
                            "Kontradiksi",
                            "Tautologi",
                            "Implikasi",
                            "Kuantor eksistensial"
                        ],
                        correct_idx: 1,
                        explanation: "p OR ~p selalu bernilai Benar untuk nilai p apapun (jika p=T maka T OR F = T; jika p=F maka F OR T = T). Pernyataan yang selalu bernilai Benar disebut Tautologi."
                    }
                ],
                key_concepts: &["Kuantor Universal (∀)", "Kuantor Eksistensial (∃)", "Negasi Kuantor", "Tautologi", "Kontradiksi"],
            }
        ],
    }
}
