use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 3,
        number: 4,
        title: "Teori Himpunan Bagian 1",
        subtitle: "Konsep Dasar, Penyajian, & Himpunan Kuasa",
        icon: "04",
        chunks: vec![
            Chunk {
                id: "4.1",
                title: "Konsep Himpunan & Cara Penyajian",
                reading_time: "4 menit",
                diffuse_content: r#"# Analogi: Keranjang Belanja & Folder Komputer

Bayangkan sebuah keranjang belanja atau folder di komputermu:
- Di dalam folder `Dokumen Kuliah`, ada file `catatan.txt`, `tugas.pdf`, dan `foto.png`.
- Setiap file di dalam folder tersebut terdefinisi secara jelas dan unik. Jika kamu mendownload file yang sama dua kali ke dalam folder yang sama, komputermu tidak akan menganggapnya sebagai dua folder berbeda.

### Apa Itu Himpunan?
Himpunan adalah kumpulan benda atau objek yang **terdefinisi dengan jelas (*well-defined*)**:
- *"Kumpulan mahasiswa UDINUS yang tingginya di atas 170 cm"* → **Himpunan**, karena tinggi badan bisa diukur dengan meteran secara pasti!
- *"Kumpulan mahasiswa berwajah tampan"* → **Bukan Himpunan**, karena tampan itu relatif/subjektif!"#,
                focused_content: r#"# Definisi & Notasi Himpunan

**Himpunan (*Set*)** adalah kumpulan objek-objek yang terdefinisi dengan jelas (*well-defined collection of objects*). Objek di dalam himpunan disebut **elemen / anggota (*members/elements*)**.

Jika x adalah anggota himpunan A, ditulis x ∈ A. Jika bukan, ditulis x ∉ A.

### 4 Cara Penyajian Himpunan:
1. **Enumerasi / Mendaftar Anggota**:
   - A = {2, 4, 6, 8}
   - Urutan anggota tidak berpengaruh: {1, 2} = {2, 1}
   - Duplikasi tidak menambah anggota: {1, 1, 2} = {1, 2}
2. **Notasi Pembentuk Himpunan (*Set Builder*)**:
   - B = {x | x  bilangan bulat positif genap, x < 10}
3. **Simbol Baku Matematika**:
   - ℕ = {0, 1, 2, 3, …} (Bilangan Alami / Natural)
   - ℤ⁺ = {…, -2, -1, 0, 1, 2, …} (Bilangan Bulat / Integer)
   - ℝ = Himpunan Bilangan Riil
4. **Diagram Venn**:
   - Visualisasi grafis bidang tertutup (lingkaran) di dalam persegi panjang semesta U."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Manakah kumpulan di bawah ini yang merupakan HIMPUNAN yang terdefinisi dengan jelas (well-defined)?",
                        options: [
                            "Kumpulan makanan yang sangat lezat di kantin",
                            "Kumpulan game yang paling seru di Steam",
                            "Kumpulan mahasiswa yang memiliki IPK di atas 3.50",
                            "Kumpulan pakaian yang modis"
                        ],
                        correct_idx: 2,
                        explanation: "IPK di atas 3.50 memiliki kriteria ukur yang pasti dan objektif berdasarkan transkrip nilai, sedangkan lezat, seru, dan modis adalah penilaian subjektif."
                    },
                    QuizQuestion {
                        question: "Jika A = {1, 2, 2, 3, 3, 3}, berapa jumlah elemen unik dalam himpunan A?",
                        options: [
                            "6 elemen",
                            "3 elemen",
                            "1 elemen",
                            "Tidak terdefinisi"
                        ],
                        correct_idx: 1,
                        explanation: "Dalam teori himpunan, pengulangan elemen tidak dianggap menambah anggota baru. Himpunan {1, 2, 2, 3, 3, 3} sama persis dengan {1, 2, 3}, yaitu memiliki 3 elemen unik."
                    }
                ],
                key_concepts: &["Himpunan (Set)", "Elemen (∈)", "Enumerasi", "Set Builder Notation", "Diagram Venn"],
            },
            Chunk {
                id: "4.2",
                title: "Kardinalitas & Himpunan Kosong",
                reading_time: "3 menit",
                diffuse_content: r#"# Analogi: Menghitung Isi Dompet & Dompet Kosong

- **Kardinalitas**: Bayangkan kamu membuka dompetmu dan menghitung berapa banyak kartu identitas berbeda di dalamnya: KTP, KTM, SIM. Ada 3 kartu berbeda. Maka kardinalitas dompetmu adalah **3**!
- **Himpunan Kosong (∅)**: Bayangkan sebuah kotak kosong. Kotak itu ada, tetapi tidak berisi benda apapun di dalamnya.
  - Perhatikan: Sebuah kotak kosong **BERBEDA** dengan kotak yang berisi angka 0! Kotak berisi angka 0 memiliki 1 buah isi (yaitu angka 0), sedangkan kotak kosong benar-benar tidak memiliki isi (|∅| = 0)."#,
                focused_content: r#"# Kardinalitas & Himpunan Khusus

### 1. Kardinalitas (|A| atau n(A))
Kardinalitas adalah jumlah elemen berbeda yang termuat di dalam suatu himpunan hingga (*finite set*).
- Jika A = {x | x  huruf vokal dalam abjad}, maka A = {a, i, u, e, o}, sehingga |A| = 5.

### 2. Himpunan Kosong (*Empty Set* / *Null Set*)
Himpunan yang tidak memiliki satu pun elemen.
- Dilambangkan dengan simbol ∅ atau {}.
- Kardinalitas himpunan kosong adalah nol: |∅| = 0.
- Contoh: A = {x | x  bilangan bulat ganjil yang habis dibagi 2} → A = ∅.
- **Penting**: Himpunan {0} BUKAN himpunan kosong karena memiliki 1 elemen (yaitu 0). Himpunan {∅} juga bukan himpunan kosong karena berisi sebuah elemen berupa himpunan kosong!

### 3. Himpunan Semesta (U atau S)
Himpunan yang memuat seluruh objek yang sedang dibicarakan dalam suatu konteks tertentu."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Berapa kardinalitas dari himpunan A = {x | x adalah bilangan bulat positif kelipatan 5 yang kurang dari 20}?",
                        options: [
                            "2",
                            "3",
                            "4",
                            "5"
                        ],
                        correct_idx: 1,
                        explanation: "Elemen dari A adalah {5, 10, 15}. Angka 20 tidak termasuk karena syaratnya 'kurang dari 20'. Jadi kardinalitas |A| = 3."
                    },
                    QuizQuestion {
                        question: "Manakah pernyataan yang BENAR mengenai himpunan kosong ∅?",
                        options: [
                            "|∅| = 1",
                            "∅ = {0}",
                            "|∅| = 0",
                            "∅ tidak bisa menjadi himpunan bagian dari himpunan manapun"
                        ],
                        correct_idx: 2,
                        explanation: "Himpunan kosong tidak memiliki anggota apapun, sehingga jumlah elemennya adalah |∅| = 0."
                    }
                ],
                key_concepts: &["Kardinalitas (|A|)", "Himpunan Kosong (∅)", "Himpunan Semesta (U)", "Himpunan Hingga vs Tak Hingga"],
            },
            Chunk {
                id: "4.3",
                title: "Himpunan Bagian & Power Set",
                reading_time: "5 menit",
                diffuse_content: r#"# Analogi: Memilih Topping Pizza & Power Set

Bayangkan sebuah kedai pizza menawarkan 3 pilihan topping: **{Keju, Sosis, Jamur}**.
Ada berapa banyak variasi pesanan pizza yang bisa dibuat oleh pelanggan?

1. **Tanpa topping sama sekali**: {} (Pizza polos / Himpunan kosong)
2. **1 topping**: {Keju}, {Sosis}, {Jamur} (3 variasi)
3. **2 topping**: {Keju, Sosis}, {Keju, Jamur}, {Sosis, Jamur} (3 variasi)
4. **Semua 3 topping**: {Keju, Sosis, Jamur} (1 variasi)

Total variasi = 1 + 3 + 3 + 1 = 8 kemungkinan!
Perhatikan polanya: 2³ = 8!

Kumpulan dari seluruh variasi kombinasi ini disebut **Power Set (Himpunan Kuasa)**."#,
                focused_content: r#"# Himpunan Bagian (*Subset*) & Himpunan Kuasa (*Power Set*)

### 1. Himpunan Bagian (*Subset* / ⊆)
Himpunan A dikatakan himpunan bagian dari B (A ⊆ B) jika dan hanya jika setiap elemen dari A juga merupakan elemen dari B:

> **A ⊆ B ⇔ ∀ x (x ∈ A → x ∈ B)**

- **Sifat Fundamental**:
  1. Himpunan kosong adalah subset dari setiap himpunan: ∅ ⊆ A untuk sembarang A.
  2. Setiap himpunan adalah subset dari dirinya sendiri: A ⊆ A.

### 2. Himpunan Bagian Sejati (*Proper Subset* / ⊂)
A ⊂ B jika A ⊆ B dan A ≠ B (ada minimal satu elemen B yang tidak ada di A).

### 3. Himpunan Kuasa (*Power Set* / P(A))
Himpunan kuasa dari A adalah himpunan yang memuat **seluruh himpunan bagian** dari A.

> **P(A) = {S | S ⊆ A}**

Jika himpunan A memiliki n elemen (|A| = n), maka jumlah anggota himpunan kuasanya adalah:

> **|P(A)| = 2ⁿ**

### Contoh:
Jika A = {1, 2}, maka:

> **P(A) = {∅, {1}, {2}, {1, 2}}**

Banyak anggota |𝒫(A)| = 2² = 4."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Jika sebuah himpunan S memiliki 4 elemen unik (|S| = 4), berapa banyak anggota dari himpunan kuasa P(S)?",
                        options: [
                            "8",
                            "12",
                            "16",
                            "24"
                        ],
                        correct_idx: 2,
                        explanation: "Banyak anggota himpunan kuasa dihitung dengan rumus 2^n. Untuk n=4, maka 2^4 = 16."
                    },
                    QuizQuestion {
                        question: "Manakah himpunan bagian yang PASTI dimiliki oleh sembarang himpunan A?",
                        options: [
                            "Himpunan {0}",
                            "Himpunan kosong ∅ dan himpunan A itu sendiri",
                            "Himpunan bilangan bulat",
                            "Himpunan {1, 2}"
                        ],
                        correct_idx: 1,
                        explanation: "Himpunan kosong (∅) selalu menjadi subset dari himpunan apapun, dan setiap himpunan selalu merupakan subset dari dirinya sendiri (A ⊆ A)."
                    }
                ],
                key_concepts: &["Subset (⊆)", "Proper Subset (⊂)", "Power Set (P(A))", "Rumus Kardinalitas 2^n"],
            }
        ],
    }
}
