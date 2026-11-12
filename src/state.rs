use std::collections::{HashMap, HashSet};
use crate::curriculum::{Module, Chunk, FlashcardItem, get_all_modules, get_default_flashcards};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tab {
    Belajar,
    Flashcards,
    Lab,
    Feynman,
}

pub struct State {
    pub modules: Vec<Module>,
    pub active_module_idx: usize,
    pub active_chunk_idx: usize,
    pub is_diffuse_mode: bool,
    pub active_tab: Tab,
    pub selected_answers: HashMap<String, usize>,
    pub flashcards: Vec<FlashcardItem>,
    pub current_card_idx: usize,
    pub card_flipped: bool,
    pub card_levels: HashMap<usize, u32>,
    pub feynman_notes: HashMap<usize, String>,
    pub feynman_input: String,
    pub completed_chunks: HashSet<String>,
    pub daily_streak: u32,
    pub dark_mode: bool,
    pub sidebar_open: bool,

    // Lab state: Truth Table
    pub truth_p: bool,
    pub truth_q: bool,
    pub truth_op: usize, // 0: AND, 1: OR, 2: IMPLIES, 3: IFF, 4: XOR

    // Lab state: Relation property tester on set {1, 2, 3}
    pub rel_pairs: HashSet<(u32, u32)>,
}

impl Default for State {
    fn default() -> Self {
        let mut rel_pairs = HashSet::new();
        // Default relation: {(1, 1), (2, 2), (3, 3), (1, 2), (2, 1)}
        rel_pairs.insert((1, 1));
        rel_pairs.insert((2, 2));
        rel_pairs.insert((3, 3));
        rel_pairs.insert((1, 2));
        rel_pairs.insert((2, 1));

        Self {
            modules: get_all_modules(),
            active_module_idx: 0,
            active_chunk_idx: 0,
            is_diffuse_mode: true,
            active_tab: Tab::Belajar,
            selected_answers: HashMap::new(),
            flashcards: get_default_flashcards(),
            current_card_idx: 0,
            card_flipped: false,
            card_levels: HashMap::new(),
            feynman_notes: HashMap::new(),
            feynman_input: String::new(),
            completed_chunks: HashSet::new(),
            daily_streak: 1,
            dark_mode: false,
            sidebar_open: true,
            truth_p: true,
            truth_q: false,
            truth_op: 2, // Default: Implikasi (p -> q)
            rel_pairs,
        }
    }
}

impl State {
    pub fn current_module(&self) -> &Module {
        &self.modules[self.active_module_idx]
    }

    pub fn current_chunk(&self) -> &Chunk {
        &self.current_module().chunks[self.active_chunk_idx]
    }

    pub fn mark_chunk_completed(&mut self, chunk_id: &str) {
        self.completed_chunks.insert(chunk_id.to_string());
    }

    pub fn progress_percentage(&self) -> f32 {
        let total_chunks: usize = self.modules.iter().map(|m| m.chunks.len()).sum();
        if total_chunks == 0 {
            return 0.0;
        }
        (self.completed_chunks.len() as f32 / total_chunks as f32) * 100.0
    }

    // Relation property checks for set {1, 2, 3}
    pub fn is_rel_reflexive(&self) -> bool {
        (1..=3).all(|x| self.rel_pairs.contains(&(x, x)))
    }

    pub fn is_rel_symmetric(&self) -> bool {
        self.rel_pairs.iter().all(|&(a, b)| self.rel_pairs.contains(&(b, a)))
    }

    pub fn is_rel_antisymmetric(&self) -> bool {
        self.rel_pairs.iter().all(|&(a, b)| {
            if a != b && self.rel_pairs.contains(&(b, a)) {
                false
            } else {
                true
            }
        })
    }

    pub fn is_rel_transitive(&self) -> bool {
        for &(a, b) in &self.rel_pairs {
            for &(b2, c) in &self.rel_pairs {
                if b == b2 && !self.rel_pairs.contains(&(a, c)) {
                    return false;
                }
            }
        }
        true
    }
}
