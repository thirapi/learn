use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 2,
        number: 3,
        title: "Logika Proposisi Bagian 2",
        subtitle: "Hukum De Morgan & Aturan Inferensi",
        icon: "03",
        chunks: vec![
            Chunk {
                id: "3.1",
                title: "Hukum De Morgan & Ekuivalensi",
                reading_time: "4 menit",
                diffuse_content: r#"# Analogi Hukum De Morgan: Aturan Syarat Beasiswa

Bayangkan sebuah syarat beasiswa:
> *"Untuk mendapatkan beasiswa, kamu harus berprestasi (p) DAN kurang mampu (q)."*

Kapan seseorang **TIDAK** memenuhi syarat tersebut?
Apakah dia harus gagal di kedua syarat? **Tidak!**
Dia ditolak jika dia **TIDAK berprestasi (¬ p)** ATAU **TIDAK kurang mampu (¬ q)**.

Perhatikan perubahan kata:
- Negasi dari *(DAN)* berubah menjadi *(ATAU)*!
- ¬(p ∧ q) ≡ ¬ p ∨ ¬ q

Inilah yang ditemukan oleh matematikawan Augustus De Morgan:
Menolak sebuah gabungan *(DAN)* sama dengan menolak salah satunya *(ATAU)*, dan sebaliknya!"#,
                focused_content: r#"# Hukum-Hukum Aljabar Proposisi

Dua proposisi majemuk P dan Q dikatakan **Ekuivalen secara Logika (P ≡ Q)** jika keduanya memiliki nilai kebenaran yang identik di setiap baris tabel kebenarannya.

### 1. Hukum De Morgan (Sangat Penting):

> **¬(p ∧ q) ≡ ¬ p ∨ ¬ q**

> **¬(p ∨ q) ≡ ¬ p ∧ ¬ q**

### 2. Hukum Implikasi:

> **p → q ≡ ¬ p ∨ q**

*(Artinya: "Jika p maka q" sama persis artinya dengan "Tidak p, ATAU q")*

### 3. Hukum Distributif:

> **p ∧ (q ∨ r) ≡ (p ∧ q) ∨ (p ∧ r)**

> **p ∨ (q ∧ r) ≡ (p ∨ q) ∧ (p ∨ r)**

### 4. Hukum Negasi Ganda:

> **¬(¬ p) ≡ p**

"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Berdasarkan Hukum De Morgan, bentuk ekuivalen dari ~(p AND q) adalah...",
                        options: [
                            "~p AND ~q",
                            "~p OR ~q",
                            "p OR q",
                            "~p -> ~q"
                        ],
                        correct_idx: 1,
                        explanation: "Hukum De Morgan menyatakan bahwa negasi dari konjungsi (AND) menjadi disjungsi (OR) dari masing-masing negasinya: ~(p AND q) ekuivalen dengan ~p OR ~q."
                    },
                    QuizQuestion {
                        question: "Bentuk ekuivalensi dasar dari implikasi p -> q dalam bentuk disjungsi adalah...",
                        options: [
                            "p OR q",
                            "~p AND q",
                            "~p OR q",
                            "p AND ~q"
                        ],
                        correct_idx: 2,
                        explanation: "p -> q secara logika ekuivalen dengan ~p OR q. Kamu bisa membuktikannya lewat tabel kebenaran: keduanya hanya bernilai False ketika p=T dan q=F."
                    }
                ],
                key_concepts: &["Ekuivalensi Logika (≡)", "Hukum De Morgan", "Bentuk Implikasi (~p ∨ q)", "Distributif Logika"],
            },
            Chunk {
                id: "3.2",
                title: "Aturan Inferensi: Ponens, Tollens, Silogisme",
                reading_time: "5 menit",
                diffuse_content: r#"# Seni Menarik Kesimpulan yang Tak Terbantahkan

Inferensi adalah proses menurunkan kesimpulan (*conclusion*) yang sah dari sekumpulan fakta atau premis yang diketahui.

### 1. Modus Ponens (Aturan Menegaskan)
- Premis 1: *"Jika hujan (p), jalanan basah (q)."*
- Premis 2: *"Sekarang sedang hujan (p)."*
- **Kesimpulan Pasti**: *"Maka jalanan basah (q)!"*

### 2. Modus Tollens (Aturan Menyangkal)
- Premis 1: *"Jika kamu makan racun (p), kamu keracunan (q)."*
- Premis 2: *"Kamu sehat bugar, tidak keracunan (¬ q)."*
- **Kesimpulan Pasti**: *"Pasti kamu tidak makan racun (¬ p)!"*

### 3. Silogisme Hipotesis (Aturan Rantai Implikasi)
- Premis 1: *"Jika kamu rajin belajar (p), kamu lulus ujian (q)."*
- Premis 2: *"Jika kamu lulus ujian (q), kamu dapat kerja (r)."*
- **Kesimpulan Rantai**: *"Jika kamu rajin belajar (p), kamu dapat kerja (r)!"*"#,
                focused_content: r#"# Argumen & Aturan Inferensi Valid

Sebuah argumen dikatakan **Valid** jika kebenaran semua premisnya menjamin secara mutlak kebenaran kesimpulannya.

> **\begin{array}{cll}
**Aturan** & **Struktur Premis & Konklusi** & **Bentuk Tautologi** \\
\hline
Modus Ponens & p → q,\; p ⇒ ∴ q & ((p → q) ∧ p) → q \\
Modus Tollens & p → q,\; ¬ q ⇒ ∴ ¬ p & ((p → q) ∧ ¬ q) → ¬ p \\
Silogisme Hipotesis & p → q,\; q → r ⇒ ∴ p → r & ((p → q) ∧ (q → r)) → (p → r) \\
Silogisme Disjungtif & p ∨ q,\; ¬ p ⇒ ∴ q & ((p ∨ q) ∧ ¬ p) → q \\
Simplifikasi & p ∧ q ⇒ ∴ p & (p ∧ q) → p \\
Konjungsi & p,\; q ⇒ ∴ p ∧ q & (p) ∧ (q) → (p ∧ q)
\end{array}**

"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Diberikan premis: (1) Jika Andi belajar, Andi lulus ujian. (2) Andi tidak lulus ujian. Kesimpulan yang sah menurut Modus Tollens adalah...",
                        options: [
                            "Andi belajar dengan giat",
                            "Andi tidak belajar",
                            "Ujian terlalu sulit",
                            "Andi pindah jurusan"
                        ],
                        correct_idx: 1,
                        explanation: "Berdasarkan Modus Tollens: p -> q dan ~q menghasilkan kesimpulan ~p ('Andi tidak belajar')."
                    },
                    QuizQuestion {
                        question: "Argumen: 'p OR q' dan '~p', menghasilkan kesimpulan 'q'. Aturan inferensi ini disebut...",
                        options: [
                            "Modus Ponens",
                            "Modus Tollens",
                            "Silogisme Disjungtif",
                            "Silogisme Hipotesis"
                        ],
                        correct_idx: 2,
                        explanation: "Silogisme Disjungtif menyatakan: jika salah satu dari dua pilihan bernilai benar (p OR q) dan opsi pertama salah (~p), maka opsi kedua pasti benar (q)."
                    }
                ],
                key_concepts: &["Modus Ponens", "Modus Tollens", "Silogisme Hipotesis", "Silogisme Disjungtif", "Argumen Valid"],
            },
            Chunk {
                id: "3.3",
                title: "Konvers, Invers, Kontraposisi & Fallacy",
                reading_time: "4 menit",
                diffuse_content: r#"# Jebakan Logika: Jangan Tertipu Arah Panah!

Jika ada pernyataan: *"Jika seseorang adalah Warga Semarang, maka dia adalah Warga Jawa Tengah."*
Apakah otomatis berlaku sebaliknya: *"Jika dia Warga Jawa Tengah, pasti dia Warga Semarang"*?
**TENTU TIDAK!** Bisa saja dia warga Solo, Kudus, atau Banyumas.

Inilah mengapa arah implikasi tidak boleh sembarangan dibalik:
- **Pernyataan Awal**: p → q
- **Konvers (Dibalik)**: q → p *(TIDAK ekuivalen!)*
- **Invers (Dinegasikan)**: ¬ p → ¬ q *(TIDAK ekuivalen!)*
- **Kontraposisi (Dibalik & Dinegasikan)**: ¬ q → ¬ p *(SELALU EKUIVALEN!)*

> **Pola Emas**: Kontraposisi dari *"Jika warga Semarang maka warga Jateng"* adalah *"Jika BUKAN warga Jateng, maka PASTI BUKAN warga Semarang"*. Ini 100% benar!"#,
                focused_content: r#"# Hubungan Implikasi, Konvers, Invers, dan Kontraposisi

Dari implikasi awal p → q, dapat dibentuk tiga proposisi turunan:

1. **Konvers (*Converse*)**: q → p
2. **Invers (*Inverse*)**: ¬ p → ¬ q
3. **Kontraposisi (*Contrapositive*)**: ¬ q → ¬ p

### Tabel Kebenaran Hubungan:
| p | q | Implikasi (p → q) | Konvers (q → p) | Invers (¬ p → ¬ q) | Kontraposisi (¬ q → ¬ p) |
| :-: | :-: | :------------------: | :-----------------: | :--------------------------: | :--------------------------------: |
| T | T | **T** | T | T | **T** |
| T | F | **F** | T | T | **F** |
| F | T | **T** | F | F | **T** |
| F | F | **T** | T | T | **T** |

> ****Kesimpulan Formal: ** p → q ≡ ¬ q → ¬ p   dan   q → p ≡ ¬ p → ¬ q**

### Dua Sesat Pikir Populer (*Logical Fallacies*):
- **Affirming the Consequent**: Menganggap p → q dan q menghasilkan p (Salah!).
- **Denying the Antecedent**: Menganggap p → q dan ¬ p menghasilkan ¬ q (Salah!)."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Manakah bentuk turunan dari implikasi p -> q yang SELALU ekuivalen dengan pernyataan aslinya?",
                        options: [
                            "Konvers (q -> p)",
                            "Invers (~p -> ~q)",
                            "Kontraposisi (~q -> ~p)",
                            "Semuanya ekuivalen"
                        ],
                        correct_idx: 2,
                        explanation: "Hanya Kontraposisi (~q -> ~p) yang memiliki nilai kebenaran identik dengan implikasi awal p -> q di semua kondisi."
                    }
                ],
                key_concepts: &["Konvers (q → p)", "Invers (¬p → ¬q)", "Kontraposisi (¬q → ¬p)", "Affirming the Consequent", "Denying the Antecedent"],
            }
        ],
    }
}
