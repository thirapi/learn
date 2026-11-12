pub mod modul1_pendahuluan;
pub mod modul2_logika;
pub mod modul3_inferensi;
pub mod modul4_himpunan;
pub mod modul5_operasi_himpunan;
pub mod modul6_relasi;
pub mod modul7_fungsi;
pub mod flashcards;
pub use flashcards::*;

#[derive(Clone, Debug)]
pub struct QuizQuestion {
    pub question: &'static str,
    pub options: [&'static str; 4],
    pub correct_idx: usize,
    pub explanation: &'static str,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub id: &'static str,
    pub title: &'static str,
    pub reading_time: &'static str,
    pub diffuse_content: &'static str,
    pub focused_content: &'static str,
    pub quiz: Vec<QuizQuestion>,
    pub key_concepts: &'static [&'static str],
}

#[derive(Clone, Debug)]
pub struct Module {
    pub id: usize,
    pub number: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub chunks: Vec<Chunk>,
}

pub fn get_all_modules() -> Vec<Module> {
    vec![
        modul1_pendahuluan::get_module(),
        modul2_logika::get_module(),
        modul3_inferensi::get_module(),
        modul4_himpunan::get_module(),
        modul5_operasi_himpunan::get_module(),
        modul6_relasi::get_module(),
        modul7_fungsi::get_module(),
    ]
}
