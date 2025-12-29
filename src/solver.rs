use std::{
    collections::HashSet,
    ops::{Bound, RangeBounds as _},
    sync::LazyLock,
};

use log::debug;

use crate::letters::{EnteredLetters, Letter, LetterState};

// NOTE: could be stored more efficiently (at 3 bytes per word rather than 6). space is not actually significant.
static VALID_WORDS_STRING: &str = include_str!("../data/valid-wordle-words.txt");
static VALID_WORDS: LazyLock<Vec<[u8; 5]>> = LazyLock::new(read_words);

pub type PossibleSolutions = [Option<OverflowList>; 6];

pub fn get_valid_words(letters: &EnteredLetters) -> PossibleSolutions {
    let mut valid_counts = [const { None }; 6];

    let mut rules_so_far = HashSet::new();
    #[allow(clippy::needless_range_loop, reason = "not an improvement")]
    for row_idx in 0..6 {
        if !letters.is_row_complete(row_idx) {
            break;
        }

        let row = letters.get_row(row_idx);
        debug_assert!(row.iter().all(|l| l.is_some()));
        let row = row.iter().flatten().cloned().collect::<Vec<_>>();
        debug_assert_eq!(row.len(), 5);
        WordleRule::extract_rules(&mut rules_so_far, &row);
        debug!("Rules for row {row_idx}: {rules_so_far:?}");

        let valid_count = VALID_WORDS
            .iter()
            .enumerate()
            .filter(|(_, word)| rules_so_far.iter().all(|x| x.is_valid_for(word)))
            .fold(OverflowList::new(), |l, (idx, _)| l.push(idx as u16));

        valid_counts[row_idx] = Some(valid_count);
    }

    valid_counts
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum WordleRule {
    LetterInPosition {
        value: u8,
        index: usize,
    },
    LetterNotInPosition {
        value: u8,
        index: usize,
    },
    LetterCount {
        value: u8,
        range: (Bound<usize>, Bound<usize>),
    },
}

impl WordleRule {
    fn is_valid_for(&self, target: &[u8; 5]) -> bool {
        match *self {
            WordleRule::LetterInPosition { value, index } => target[index] == value,
            WordleRule::LetterNotInPosition { value, index } => target[index] != value,
            WordleRule::LetterCount { value, range } => {
                range.contains(&target.iter().filter(|&&x| x == value).count())
            }
        }
    }

    fn extract_rules(rules: &mut HashSet<Self>, row: &[Letter]) {
        for (idx, letter) in row.iter().enumerate() {
            if letter.state == LetterState::Correct {
                rules.insert(WordleRule::LetterInPosition {
                    value: letter.value as u8,
                    index: idx,
                });
            }

            if letter.state == LetterState::Incorrect {
                rules.insert(WordleRule::LetterNotInPosition {
                    value: letter.value as u8,
                    index: idx,
                });

                // the letter is used too many times (including more than 0 times)
                let positive_occurence_count = row
                    .iter()
                    .filter(|l| l.value == letter.value && l.state != LetterState::Incorrect)
                    .count();

                rules.insert(WordleRule::LetterCount {
                    value: letter.value as u8,
                    range: (
                        Bound::Included(positive_occurence_count),
                        Bound::Included(positive_occurence_count),
                    ),
                });
            }

            if letter.state == LetterState::Present {
                rules.insert(WordleRule::LetterNotInPosition {
                    value: letter.value as u8,
                    index: idx,
                });

                // Set upper and lower bounds
                let min_bound = row
                    .iter()
                    .filter(|l| l.value == letter.value && l.state == LetterState::Present)
                    .count();
                let has_max_bound = row
                    .iter()
                    .any(|l| l.value == letter.value && l.state == LetterState::Incorrect);
                rules.insert(WordleRule::LetterCount {
                    value: letter.value as u8,
                    range: (
                        Bound::Included(min_bound),
                        if has_max_bound {
                            Bound::Included(min_bound)
                        } else {
                            Bound::Unbounded
                        },
                    ),
                });
            }
        }
    }
}

fn read_words() -> Vec<[u8; 5]> {
    let mut vec = Vec::with_capacity(VALID_WORDS_STRING.lines().count());
    for line in VALID_WORDS_STRING.lines() {
        debug_assert_eq!(line.len(), 5);

        let array = line.chars().map(|c| c as u8).collect::<Vec<_>>();
        vec.push(array.try_into().unwrap());
    }

    vec
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverflowList {
    local_items: [u16; 10],
    total_count: usize,
}

impl OverflowList {
    pub fn new() -> Self {
        Self {
            local_items: [u16::MAX; 10],
            total_count: 0,
        }
    }

    pub fn push(mut self, new_item: u16) -> Self {
        if let Some(empty_slot) = self.local_items.iter_mut().find(|&&mut i| i == u16::MAX) {
            *empty_slot = new_item;
        }

        self.total_count += 1;

        self
    }

    pub fn get_count(&self) -> usize {
        self.total_count
    }

    pub fn get_words(&self) -> Option<Vec<String>> {
        if self.total_count > self.local_items.len() {
            None
        } else {
            Some(
                self.local_items
                    .iter()
                    .take_while(|&&i| i != u16::MAX)
                    .map(|i| VALID_WORDS[*i as usize].to_vec())
                    .map(|w| String::from_utf8(w).unwrap())
                    .collect(),
            )
        }
    }
}
