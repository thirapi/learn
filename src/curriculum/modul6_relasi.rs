use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 5,
        number: 6,
        title: "Teori Relasi",
        subtitle: "Representasi Matriks, Graf, & 4 Sifat Relasi",
        icon: "06",
        chunks: vec![
            Chunk {
                id: "6.1",
                title: "Perkalian Kartesian & Konsep Relasi",
                reading_time: "4 menit",
                diffuse_content: r#"# Analogi: Menu Makanan & Minuman di Kafe

Bayangkan sebuah kafe memiliki menu:
- Makanan (A): `{Burger, Pizza}`
- Minuman (B): `{Es Teh, Kopi, Jus}`

Jika pelanggan boleh memilih pasangan 1 makanan dan 1 minuman, apa saja seluruh pasangan yang mungkin?

> **{(Burger, Es Teh), (Burger, Kopi), (Burger, Jus), (Pizza, Es Teh), (Pizza, Kopi), (Pizza, Jus)}**

Kumpulan seluruh pasangan yang mungkin ini disebut **Perkalian Kartesian (A × B)**.

Lalu, apa itu **Relasi**?
Relasi adalah **himpunan bagian dari pasangan di atas yang memenuhi aturan tertentu**.
Misalnya: *"Hanya paket combo hemat"* → hanya {(Burger, Es Teh), (Pizza, Kopi)}.
Maka paket combo hemat itu adalah sebuah **Relasi**!"#,
                focused_content: r#"# Perkalian Kartesian & Definisi Relasi Biner

### 1. Perkalian Kartesian (*Cartesian Product* / A × B)
Perkalian kartesian dari himpunan A dan B adalah himpunan semua pasangan terurut (a, b) dengan a ∈ A dan b ∈ B:

> **A × B = {(a, b) | a ∈ A ∧ b ∈ B}**

Jika |A| = m dan |B| = n, maka banyak elemen perkalian kartesian adalah:

> **|A × B| = m × n**

### 2. Definisi Relasi Biner
Relasi biner R antara himpunan A dan B adalah himpunan bagian dari perkalian kartesian A × B:

> **R ⊆ A × B**

- Jika (a, b) ∈ R, kita katakan a berelasi dengan b oleh R, dinotasikan dengan a R b.
- Jika (a, b) ∉ R, dinotasikan dengan a \not R b."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Jika A = {1, 2} dan B = {x, y, z}, berapakah jumlah elemen dari perkalian kartesian A x B?",
                        options: [
                            "5",
                            "6",
                            "8",
                            "9"
                        ],
                        correct_idx: 1,
                        explanation: "Banyak elemen perkalian kartesian |A x B| = |A| * |B| = 2 * 3 = 6 pasangan terurut."
                    },
                    QuizQuestion {
                        question: "Secara matematis formal, apa definisi dari relasi biner R dari himpunan A ke B?",
                        options: [
                            "R adalah gabungan A ∪ B",
                            "R adalah irisan A ∩ B",
                            "R adalah himpunan bagian dari perkalian kartesian A x B (R ⊆ A x B)",
                            "R adalah komplemen dari A"
                        ],
                        correct_idx: 2,
                        explanation: "Relasi biner R dari himpunan A ke B didefinisikan secara formal sebagai sembarang himpunan bagian dari hasil kali kartesian A x B."
                    }
                ],
                key_concepts: &["Pasangan Terurut (a, b)", "Perkalian Kartesian (A × B)", "Relasi Biner (R ⊆ A × B)", "Notasi a R b"],
            },
            Chunk {
                id: "6.2",
                title: "4 Cara Representasi Relasi",
                reading_time: "5 menit",
                diffuse_content: r#"# Bagaimana Komputer Menyimpan Relasi?

Jika kamu seorang software engineer, bagaimana kamu menyimpan hubungan "A berteman dengan B"?
Ada 4 cara:

1. **Daftar Pasangan**: Seperti array data JSON: `[("Andi", "Budi"), ("Budi", "Citra")]`.
2. **Diagram Panah**: Menggambar dua kolom lalu menarik garis panah dari kiri ke kanan.
3. **Matriks Boolean (0 dan 1)**: Kotak grid tabel baris dan kolom. Jika ada hubungan, tulis `1`. Jika tidak, tulis `0`. Komputer sangat menyukai cara ini karena hemat memori!
4. **Graf Berarah (*Directed Graph / Digraph*)**: Titik-titik simpul yang dihubungkan panah (seperti sistem *Following/Followers* di Twitter/X dan Instagram)!"#,
                focused_content: r#"# Representasi Relasi

Misalkan relasi R pada A = {1, 2, 3} dengan aturan a R b ⇔ a ≤ b:
R = {(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (3, 3)}

### 1. Diagram Panah
Elemen himpunan A di kiri dihubungkan dengan panah ke elemen B di kanan.

### 2. Matriks Nol-Satu / Matriks Boolean (M_R)
Matriks berukuran m × n di mana elemen baris i dan kolom j bernilai:

> **m_{ij} = \begin{cases} 1, & jika  (a_i, b_j) ∈ R \\ 0, & jika  (a_i, b_j) ∉ R \end{cases}**

Untuk relasi di atas:

> **M_R = \begin{pmatrix} 1 & 1 & 1 \\ 0 & 1 & 1 \\ 0 & 0 & 1 \end{pmatrix}**

### 3. Graf Berarah (*Directed Graph / Digraph*)
- Setiap elemen himpunan digambar sebagai simpul (*vertex / node*).
- Jika (a, b) ∈ R, digambar busur berarah (*directed edge*) dari simpul a ke simpul b.
- Jika (a, a) ∈ R, digambar loop melingkar pada simpul a."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Pada representasi matriks relasi MR berukuran 3x3, apa arti nilai elemen m[2][1] = 1?",
                        options: [
                            "Elemen ke-2 berelasi dengan elemen ke-1 ((a2, b1) ∈ R)",
                            "Elemen ke-1 berelasi dengan elemen ke-2",
                            "Matriks tidak simetris",
                            "Relasi tersebut adalah relasi kosong"
                        ],
                        correct_idx: 0,
                        explanation: "Indeks baris menyatakan elemen asal (a2) dan kolom menyatakan elemen tujuan (b1). Nilai 1 berarti pasangan (a2, b1) merupakan anggota dari relasi R."
                    }
                ],
                key_concepts: &["Diagram Panah", "Matriks Boolean 0/1", "Graf Berarah (Digraph)", "Simpul & Busur Loop"],
            },
            Chunk {
                id: "6.3",
                title: "4 Sifat Relasi & Relasi Invers",
                reading_time: "5 menit",
                diffuse_content: r#"# Mengenali Sifat Relasi di Dunia Nyata

Sebuah relasi pada himpunan A dapat memiliki sifat-sifat istimewa:

1. **Refleksif (Semua Punya Loop ke Diri Sendiri)**:
   - Contoh: Relasi *"sama umurnya dengan"*. Tentu saja Andi sama umurnya dengan dirinya sendiri. Setiap elemen berelasi dengan dirinya sendiri!
2. **Simetris (Hubungan Dua Arah / Bolak-balik)**:
   - Contoh: Relasi *"berteman di Facebook"*. Jika Andi berteman dengan Budi, otomatis Budi berteman dengan Andi.
3. **Antisimetris (Tidak Boleh Ada Dua Arah Kecuali ke Diri Sendiri)**:
   - Contoh: Relasi *"lebih tua daripada"*. Jika Andi lebih tua dari Budi, maka mustahil Budi lebih tua dari Andi!
4. **Transitif (Rantai Keterhubungan)**:
   - Contoh: Relasi *"keturunan dari"*. Jika Ani anak dari Budi, dan Budi anak dari Candra, maka Ani adalah keturunan dari Candra."#,
                focused_content: r#"# Definisi 4 Sifat Relasi Biner pada Himpunan A

### 1. Refleksif (*Reflexive*)
Relasi R bersifat refleksif jika untuk **setiap** a ∈ A, berlaku (a, a) ∈ R:

> **∀ a ∈ A, \; (a, a) ∈ R**

- **Ciri Matriks**: Diagonal utama matriks M_R bernilai 1 semua.
- **Ciri Graf**: Setiap simpul memiliki loop ke dirinya sendiri.

### 2. Simetris (*Symmetric*)
Jika (a, b) ∈ R, maka (b, a) ∈ R untuk setiap a, b ∈ A:

> **∀ a, b ∈ A, \; (a, b) ∈ R ⇒ (b, a) ∈ R**

- **Ciri Matriks**: Matriks simetris terhadap diagonal utama (M_R = M_R^T).

### 3. Antisimetris (*Antisymmetric*)
Jika (a, b) ∈ R dan (b, a) ∈ R, maka haruslah a = b:

> **∀ a, b ∈ A, \; ((a, b) ∈ R ∧ (b, a) ∈ R) ⇒ a = b**

*(Artinya: jika a ≠ b, tidak boleh ada panah bolak-balik)*.

### 4. Transitif (*Transitive*)
Jika (a, b) ∈ R dan (b, c) ∈ R, maka (a, c) ∈ R:

> **∀ a, b, c ∈ A, \; ((a, b) ∈ R ∧ (b, c) ∈ R) ⇒ (a, c) ∈ R**

### Relasi Invers (R⁻¹):

> **R⁻¹ = {(b, a) | (a, b) ∈ R}**

Matriks dari relasi invers adalah transpose dari matriks aslinya: M_{R⁻¹} = (M_R)^T."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Diketahui A = {1, 2, 3} dan R = {(1, 1), (2, 2), (3, 3), (1, 2)}. Apakah sifat dari relasi R?",
                        options: [
                            "Refleksif",
                            "Simetris",
                            "Transitif dan Simetris",
                            "Relasi kosong"
                        ],
                        correct_idx: 0,
                        explanation: "Relasi R memuat semua pasangan loop identitas: (1, 1), (2, 2), dan (3, 3) untuk seluruh elemen di A, sehingga R bersifat Refleksif. (R tidak simetris karena ada (1, 2) tapi tidak ada (2, 1))."
                    },
                    QuizQuestion {
                        question: "Apa ciri visual pada matriks relasi MR yang menandakan bahwa relasi tersebut bersifat SIMETRIS?",
                        options: [
                            "Semua elemen diagonal utama bernilai 0",
                            "Matriks sama dengan transpose-nya (MR = MR^T), simetris terhadap diagonal utama",
                            "Semua baris bernilai 1",
                            "Semua kolom bernilai 0"
                        ],
                        correct_idx: 1,
                        explanation: "Relasi simetris berarti jika m[i][j] = 1 maka m[j][i] = 1, yang merupakan definisi matematis dari matriks simetris (MR = MR^T)."
                    }
                ],
                key_concepts: &["Refleksif (∀a (a,a)∈R)", "Simetris ((a,b)∈R → (b,a)∈R)", "Antisimetris", "Transitif", "Relasi Invers (R^-1)"],
            }
        ],
    }
}
