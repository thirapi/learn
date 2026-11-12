use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 4,
        number: 5,
        title: "Teori Himpunan Bagian 2",
        subtitle: "Operasi Himpunan & Prinsip Inklusi-Eksklusi",
        icon: "05",
        chunks: vec![
            Chunk {
                id: "5.1",
                title: "5 Operasi Himpunan Dasar",
                reading_time: "5 menit",
                diffuse_content: r#"# Analogi Teman Bersama di Media Sosial

Bayangkan lingkaran pertemanan dua orang, **Andi** dan **Budi**:
- Teman Andi: {Citra, Doni, Eka}
- Teman Budi: {Doni, Eka, Fani, Gilang}

1. **Irisan (∩ / Mutual Friends)**: Siapa teman bersama mereka berdua?
   → `{Doni, Eka}` (ada di lingkaran Andi **DAN** ada di lingkaran Budi).
2. **Gabungan (∪ / Semua Teman)**: Gabungkan seluruh teman mereka tanpa duplikasi.
   → `{Citra, Doni, Eka, Fani, Gilang}`.
3. **Selisih (A - B / Teman Khusus Andi Saja)**: Teman Andi yang BUKAN temannya Budi.
   → `{Citra}` (Doni dan Eka dicoret karena berteman dengan Budi).
4. **Beda Setangkup (⊕ / Eksklusif)**: Teman yang HANYA berteman dengan salah satu dari mereka (tidak kenal keduanya).
   → `{Citra, Fani, Gilang}`."#,
                focused_content: r#"# Definisi Matematis Operasi Himpunan

Misalkan U adalah himpunan semesta, dan A, B ⊆ U:

1. **Irisan (*Intersection* / ∩)**:
   

> **A ∩ B = {x | x ∈ A ∧ x ∈ B}**

   *(Dua himpunan disebut Saling Lepas / Disjoint jika A ∩ B = ∅)*.

2. **Gabungan (*Union* / ∪)**:
   

> **A ∪ B = {x | x ∈ A ∨ x ∈ B}**

3. **Selisih (*Difference* / A - B atau A ∖ B)**:
   

> **A - B = {x | x ∈ A ∧ x ∉ B} = A ∩ B^c**

4. **Komplemen (*Complement* / A^c atau \bar{A})**:
   

> **A^c = {x ∈ U | x ∉ A} = U - A**

5. **Beda Setangkup (*Symmetric Difference* / ⊕ atau Δ)**:
   

> **A ⊕ B = (A ∪ B) - (A ∩ B) = (A - B) ∪ (B - A)**

   *(Elemen yang menjadi anggota salah satu himpunan, tetapi bukan anggota keduanya)*."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Diketahui A = {1, 2, 3, 4} dan B = {3, 4, 5, 6}. Berapakah hasil dari selisih A - B?",
                        options: [
                            "{1, 2}",
                            "{3, 4}",
                            "{5, 6}",
                            "{1, 2, 5, 6}"
                        ],
                        correct_idx: 0,
                        explanation: "A - B adalah anggota A yang BUKAN merupakan anggota B. Karena 3 dan 4 ada di B, maka yang tersisa murni di A adalah {1, 2}."
                    },
                    QuizQuestion {
                        question: "Berdasarkan data di atas (A = {1, 2, 3, 4} dan B = {3, 4, 5, 6}), berapakah hasil beda setangkup A ⊕ B?",
                        options: [
                            "{3, 4}",
                            "{1, 2, 3, 4, 5, 6}",
                            "{1, 2, 5, 6}",
                            "∅"
                        ],
                        correct_idx: 2,
                        explanation: "Beda setangkup A ⊕ B adalah gabungan dikurangi irisan: (A ∪ B) - (A ∩ B) = {1, 2, 3, 4, 5, 6} - {3, 4} = {1, 2, 5, 6}."
                    }
                ],
                key_concepts: &["Irisan (∩)", "Gabungan (∪)", "Selisih (A - B)", "Komplemen (A^c)", "Beda Setangkup (⊕)"],
            },
            Chunk {
                id: "5.2",
                title: "Hukum Aljabar Himpunan & De Morgan",
                reading_time: "4 menit",
                diffuse_content: r#"# Mengapa Aljabar Himpunan Mirip Aljabar Logika?

Perhatikan kemiripan luar biasa antara simbol logika dan simbol himpunan:
- Logika: ∧ (Dan) ⇔ Himpunan: ∩ (Irisan)
- Logika: ∨ (Atau) ⇔ Himpunan: ∪ (Gabungan)
- Logika: ¬ (Negasi) ⇔ Himpunan: ^c (Komplemen)

Karena kemiripan ini, semua hukum logika yang sudah kamu pelajari di Pertemuan 3 berlaku persis di teori himpunan!

Contoh: Hukum De Morgan untuk himpunan:

> **(A ∩ B)^c = A^c ∪ B^c**

*"Bukan anggota dari (A dan B) artinya: Bukan anggota A, ATAU Bukan anggota B"*."#,
                focused_content: r#"# Tabel Hukum Aljabar Himpunan

| Nama Hukum | Hukum Irisan & Gabungan |
| :--- | :--- |
| **Identitas** | A ∩ U = A dan A ∪ ∅ = A |
| **Dominasi** | A ∩ ∅ = ∅ dan A ∪ U = U |
| **Komplemen** | A ∩ A^c = ∅ dan A ∪ A^c = U |
| **Idempoten** | A ∩ A = A dan A ∪ A = A |
| **Involusi** | (A^c)^c = A |
| **Komutatif** | A ∩ B = B ∩ A dan A ∪ B = B ∪ A |
| **Asosiatif** | A ∩ (B ∩ C) = (A ∩ B) ∩ C |
| **Distributif** | A ∩ (B ∪ C) = (A ∩ B) ∪ (A ∩ C) |
| **De Morgan** | (A ∩ B)^c = A^c ∪ B^c dan (A ∪ B)^c = A^c ∩ B^c |"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Bentuk ekuivalen dari komplemen gabungan dua himpunan (A ∪ B)^c menurut Hukum De Morgan adalah...",
                        options: [
                            "A^c ∪ B^c",
                            "A^c ∩ B^c",
                            "A ∩ B",
                            "(A ∩ B)^c"
                        ],
                        correct_idx: 1,
                        explanation: "Hukum De Morgan untuk himpunan: komplemen dari gabungan berubah menjadi irisan dari masing-masing komplemennya, yaitu A^c ∩ B^c."
                    }
                ],
                key_concepts: &["Hukum De Morgan Himpunan", "Hukum Distributif", "Involusi ((A^c)^c = A)", "Himpunan Saling Lepas (Disjoint)"],
            },
            Chunk {
                id: "5.3",
                title: "Prinsip Inklusi-Eksklusi (PIE)",
                reading_time: "5 menit",
                diffuse_content: r#"# Masalah Menghitung Ganda (Double Counting)

Bayangkan di kelas Teknik Informatika ada:
- 25 mahasiswa suka ngoding **Python**
- 20 mahasiswa suka ngoding **JavaScript**
- 10 mahasiswa suka **KEDUANYA (Python dan JavaScript)**

Berapa total jumlah mahasiswa yang menyukai minimal salah satu bahasa?
Jika kamu langsung menjumlahkan 25 + 20 = 45, kamu **SALAH BESAR!**
Kenapa? Karena 10 anak yang suka keduanya **terhitung dua kali** (sekali di kelompok Python, sekali lagi di kelompok JS).

Untuk mendapatkan angka sebenarnya, kamu harus mengurangkan yang terhitung ganda:

> **Total = 25 + 20 - 10 = 35  orang!**

Inilah yang disebut **Prinsip Inklusi-Eksklusi**!"#,
                focused_content: r#"# Rumus Formal Prinsip Inklusi-Eksklusi

Prinsip Inklusi-Eksklusi (*Inclusion-Exclusion Principle*) digunakan untuk menentukan kardinalitas dari gabungan himpunan-himpunan hingga.

### 1. Dua Himpunan:

> **|A ∪ B| = |A| + |B| - |A ∩ B|**

### 2. Tiga Himpunan:

> **|A ∪ B ∪ C| = (|A| + |B| + |C|) - (|A ∩ B| + |A ∩ C| + |B ∩ C|) + |A ∩ B ∩ C|**

*(Perhatikan pola tanda: Tambah suku tunggal → Kurang irisan berpasangan → Tambah kembali irisan ketiganya)*.

### Contoh Soal Kuliah:
Dari 100 mahasiswa, 40 mengambil mata kuliah Matematika Diskrit, 50 mengambil Algoritma Pemrograman, dan 15 mengambil kedua mata kuliah tersebut.
Berapa mahasiswa yang **tidak mengambil satupun** dari kedua mata kuliah itu?
1. Cari yang mengambil minimal satu:
   

> **|M ∪ A| = 40 + 50 - 15 = 75**

2. Cari sisanya dari semesta 100:
   

> **100 - 75 = 25  mahasiswa**

."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Diketahui |A| = 15, |B| = 25, dan |A ∩ B| = 8. Berapakah kardinalitas gabungan |A ∪ B|?",
                        options: [
                            "40",
                            "32",
                            "48",
                            "17"
                        ],
                        correct_idx: 1,
                        explanation: "Berdasarkan Prinsip Inklusi-Eksklusi: |A ∪ B| = |A| + |B| - |A ∩ B| = 15 + 25 - 8 = 32."
                    }
                ],
                key_concepts: &["Prinsip Inklusi-Eksklusi", "Double Counting", "Kardinalitas Gabungan", "Irisan Tiga Himpunan"],
            }
        ],
    }
}
