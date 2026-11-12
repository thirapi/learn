use super::{Chunk, Module, QuizQuestion};

pub fn get_module() -> Module {
    Module {
        id: 0,
        number: 1,
        title: "Pendahuluan Matematika Diskrit",
        subtitle: "Fondasi Dunia Digital: Diskrit vs Kontinu",
        icon: "01",
        chunks: vec![
            Chunk {
                id: "1.1",
                title: "Dunia Diskrit vs Kontinu",
                reading_time: "3 menit",
                diffuse_content: r#"# Analogi: Jam Analog vs Jam Digital

Bayangkan kamu sedang melihat dua buah jam di dinding:
1. **Jam Analog (Kontinu)**: Jarum detiknya bergerak meluncur secara halus tanpa henti. Di antara detik ke-1 dan detik ke-2, terdapat tak hingga banyaknya titik waktu (1.1, 1.001, 1.0000001 detik). Ini adalah dunia **matematika kontinu** (kalkulus).
2. **Jam Digital (Diskrit)**: Angka di layar melompat seketika dari `12:00:01` ke `12:00:02`. Tidak ada angka di antaranya. Setiap detakan terpisah secara jelas (*distinct*). Ini adalah dunia **matematika diskrit**.

### Analogi Lain: Lukisan Cat Minyak vs Foto Digital
- Lukisan cat minyak memiliki gradasi warna yang menyatu secara mulus (kontinu).
- Foto digital di layar laptopmu tersusun atas jutaan **piksel** individual berbentuk kotak (diskrit). Jika kamu perbesar (*zoom in*), kamu akan melihat unit-unit kotak terkecil yang terpisah!

> **Kunci Intuisi**: Diskrit artinya **terpisah, terhitung, dan memiliki batas yang tegas antar elemennya**."#,
                focused_content: r#"# Definisi Formal: Matematika Diskrit

**Matematika Diskrit (*Discrete Mathematics*)** adalah cabang matematika yang mengkaji objek-objek diskrit, yaitu objek yang terdiri dari elemen-elemen yang berbeda dan tidak saling terhubung secara mulus (*distinct and separated values*).

### Karakteristik Objek Diskrit:
1. **Himpunan Terhitung (*Countable Sets*)**: Objek-objeknya dapat dipasangkan satu-ke-satu dengan himpunan bilangan bulat positif ℤ⁺ = {1, 2, 3, …}.
2. **Ketiadaan Nilai Perantara Tak Hingga**: Di antara dua bilangan bulat berurutan n dan n+1, tidak ada bilangan bulat lain.
3. **Representasi Presisi**: Tidak memerlukan aproksimasi limit seperti pada kalkulus kontinu.

### Mengapa Bukan Kontinu?
Pada kalkulus kontinu, variabel x ∈ ℝ dapat mengambil sembarang nilai riil di antara dua titik (0.0001, 0.000001). Sebaliknya, sistem diskrit memodelkan keadaan (*states*) yang jelas terdefinisi: hidup/mati, benar/salah, 0/1."#,
                quiz: vec![
                    QuizQuestion {
                        question: "Manakah di bawah ini yang merupakan contoh dari sistem diskrit?",
                        options: [
                            "Suhu udara yang naik perlahan dari pagi ke siang",
                            "Jumlah mahasiswa yang hadir di kelas (misal: 35 orang)",
                            "Kecepatan mobil yang melaju di jalan tol",
                            "Tinggi air sungai yang mengalir deras"
                        ],
                        correct_idx: 1,
                        explanation: "Jumlah mahasiswa adalah bilangan bulat terhitung (diskrit). Kamu tidak bisa memiliki 35.4 orang mahasiswa! Sebaliknya suhu, kecepatan, dan aliran air adalah nilai kontinu."
                    },
                    QuizQuestion {
                        question: "Apa perbedaan paling mendasar antara matematika kontinu dan matematika diskrit?",
                        options: [
                            "Matematika diskrit hanya mempelajari angka nol",
                            "Matematika kontinu tidak memiliki rumus",
                            "Matematika diskrit mempelajari objek yang terpisah dan terhitung, bukan rentang mulus",
                            "Matematika kontinu hanya digunakan oleh anak SD"
                        ],
                        correct_idx: 2,
                        explanation: "Diskrit berurusan dengan objek-objek terpisah (himpunan terhitung, bit, simpul graf), sedangkan kontinu berurusan dengan variabel tak hingga pada interval mulus (kalkulus)."
                    }
                ],
                key_concepts: &["Diskrit vs Kontinu", "Piksel vs Lukisan", "Himpunan Terhitung", "Objek Terpisah"],
            },
            Chunk {
                id: "1.2",
                title: "Mengapa Diskrit Jantung Informatika?",
                reading_time: "4 menit",
                diffuse_content: r#"# Mengapa Komputer Membutuhkan Matematika Diskrit?

Komputer modern pada dasarnya adalah kumpulan miliaran sakelar transistor mikro yang hanya mengenal dua kondisi: **MENYALA (1)** atau **MATI (0)**. 

Karena alamiahnya bekerja dengan bilangan bulat biner (0 dan 1), komputer tidak bisa menyimpan nilai kontinu tak terhingga secara murni tanpa memotongnya menjadi potongan-potongan diskrit.

### Penerapan Nyata di Dunia Software:
1. **Algoritma & Struktur Data**: Array, Linked List, Tree, dan Graph semuanya adalah objek diskrit.
2. **Basis Data (Database & SQL)**: Relasi antartabel di SQL diturunkan langsung dari **Teori Relasi & Himpunan**.
3. **Kriptografi & Keamanan**: Enkripsi password menggunakan teori bilangan bulat modular diskrit.
4. **Jaringan Komputer**: Menemukan rute tercepat paket data internet menggunakan **Teori Graf** (*Shortest Path Algorithm*).

> **Aha! Moment**: Belajar Matematika Diskrit sama dengan **mempelajari tata bahasa (grammar) asli dari komputer**!"#,
                focused_content: r#"# Struktur Diskrit dalam Ilmu Komputer

Hampir seluruh pilar fundamental *Computer Science* diturunkan dari struktur matematika diskrit:

| Cabang Matematika Diskrit | Implementasi di Teknik Informatika |
| :--- | :--- |
| **Logika Proposisi & Predikat** | Desain sirkuit digital, kondisi percabangan `if-else`, kecerdasan buatan (*automated theorem proving*). |
| **Teori Himpunan (*Set Theory*)** | Desain skema basis data relasional, operasi aljabar relasional SQL (`UNION`, `INTERSECT`, `JOIN`). |
| **Relasi & Fungsi** | Relasi kunci primer (*Foreign Key*), pemodelan fungsional (*pure functions*, *lambda calculus*). |
| **Kombinatorika & Teori Peluang** | Analisis kompleksitas algoritma (notasi Big-O: \mathcal{O}(n \log n)), simulasi *machine learning*. |
| **Teori Graf & Pohon (*Tree*)** | Struktur direktori filesystem, routing protokol jaringan (OSPF/BGP), representasi DOM di browser. |"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Mengapa komputer modern bekerja berbasis logika diskrit?",
                        options: [
                            "Karena programmer tidak suka matematika kalkulus",
                            "Karena sirkuit transistor komputer beroperasi pada level biner diskrit (tegangan tinggi/rendah: 1 dan 0)",
                            "Karena komputer lama tidak punya layar berwarna",
                            "Karena bilangan diskrit lebih mahal daripada bilangan kontinu"
                        ],
                        correct_idx: 1,
                        explanation: "Transistor mikroprosesor bekerja seperti sakelar digital: hanya ada kondisi ON (1) atau OFF (0), yang merupakan nilai diskrit terpisah."
                    }
                ],
                key_concepts: &["Transistor & Bit", "Struktur Data Diskrit", "Basis Data Relasional", "Kompleksitas Algoritma"],
            },
            Chunk {
                id: "1.3",
                title: "Strategi 'Learn How to Learn' untuk Diskrit",
                reading_time: "3 menit",
                diffuse_content: r#"# Cara Otak Menguasai Matematika Abstrak

Banyak mahasiswa merasa matematika diskrit itu 'sulit' karena terlalu banyak simbol asing (∀, ∃, ¬, ∧, ⊆, ⊕). 

Namun sains kognitif membuktikan bahwa siapapun bisa memahaminya dengan 3 trik:

### 1. Jangan Hanya Membaca Pasif (*Passive Reading Illusion*)
Membaca slide berulang-ulang memberikan rasa 'merasa sudah paham' (*illusion of competence*). Begitu kertas ujian datang, pikiran mendadak kosong. **Solusi**: Gunakan **Active Recall** (tutup materi, uji dirimu sendiri dengan kuis).

### 2. Hubungkan dengan Analogi (Feynman Technique)
Jika kamu tidak bisa menjelaskan sebuah konsep rumit kepada temanmu menggunakan bahasa sehari-hari yang sederhana, berarti kamu belum benar-benar memahaminya.

### 3. Spaced Repetition (Pengulangan Berjarak)
Otak manusia berevolusi untuk melupakan informasi yang tidak sering dipanggil. Dengan mengulang sedikit materi pada interval hari ke-1, hari ke-3, dan hari ke-7, konsep akan berpindah ke **memori jangka panjang** secara permanen."#,
                focused_content: r#"# Mekanisme Pembelajaran Kognitif: Diffuse vs Focused Mode

Menurut Dr. Barbara Oakley (*Learning How to Learn*), otak beroperasi dalam dua mode utama:

1. **Focused Mode (Mode Terpusat)**:
   - Pola aktivasi saraf pada korteks prefrontal.
   - Digunakan saat menyelesaikan pembuktian langkah-demi-langkah, tabel kebenaran, dan perhitungan rumus presisi.
2. **Diffuse Mode (Mode Difus / Santai)**:
   - Pola konektivitas jaringan default (*Default Mode Network*).
   - Mengaitkan konsep abstrak baru dengan pengalaman hidup atau analogi visual.
   - Terjadi saat kamu beristirahat, berjalan santai, atau merenungkan gambaran besar (*big picture*).

### Siklus Belajar Efektif di Platform Ini:

> **Diffuse (Pahami Analogi) → Focused (Kuasai Rumus Formal) → Active Recall (Uji Ingatan) → SRS (Kunci di Memori)**

"#,
                quiz: vec![
                    QuizQuestion {
                        question: "Apa yang dimaksud dengan 'illusion of competence' saat belajar?",
                        options: [
                            "Merasa pintar karena sudah membaca slide berulang kali, padahal belum bisa mengerjakan soal tanpa melihat kunci",
                            "Merasa gagal padahal nilainya selalu A",
                            "Kemampuan menghafal rumus dalam 5 detik",
                            "Kemampuan mengajar orang lain dengan sangat fasih"
                        ],
                        correct_idx: 0,
                        explanation: "Ilusi kompetensi adalah jebakan umum membaca pasif: otak mengenali teks di buku/slide sehingga terasa mudah, namun saat diuji mandiri (active recall) ternyata belum mampu mereproduksi pemahamannya."
                    }
                ],
                key_concepts: &["Focused Mode", "Diffuse Mode", "Active Recall", "Illusion of Competence", "Spaced Repetition"],
            }
        ],
    }
}
