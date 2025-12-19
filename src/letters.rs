use log::{info, warn};

/// The set of entered letters
#[derive(Default, Clone)]
pub struct EnteredLetters([Option<Letter>; 5 * 6]);

impl EnteredLetters {
    #[must_use]
    pub fn get(&self, row_idx: usize, tile_idx: usize) -> Option<Letter> {
        debug_assert!(row_idx < 6);
        debug_assert!(tile_idx < 5);

        self.0[row_idx * 5 + tile_idx]
    }

    pub fn push_letter(&mut self, letter: char) {
        let index = self.0.iter().position(|x| x.is_none());

        if let Some(index) = index {
            info!("Inserting letter {letter} at index {index}");
            self.0[index] = Some(Letter::new(letter));
        }
    }

    pub fn remove_letter(&mut self, row_idx: usize, tile_idx: usize) {
        debug_assert!(row_idx < 6);
        debug_assert!(tile_idx < 5);

        let index = row_idx * 5 + tile_idx;

        let existing = self.0[index];
        if let Some(existing) = existing {
            info!("Removing letter {existing:?} at index {index}");
            self.0[index] = None;
        } else {
            warn!("Tried to remove non-existent letter at index {index}");
        }
    }

    pub fn cycle_letter(&mut self, row_idx: usize, tile_idx: usize) {
        debug_assert!(row_idx < 6);
        debug_assert!(tile_idx < 5);

        let index = row_idx * 5 + tile_idx;

        if let Some(letter) = &mut self.0[index] {
            let new_state = match letter.state {
                LetterState::Incorrect => LetterState::Present,
                LetterState::Present => LetterState::Correct,
                LetterState::Correct => LetterState::Incorrect,
            };

            info!("Cycling letter {letter:?} at index {index} to {new_state:?}");
            letter.state = new_state;
        }
    }

    #[inline]
    pub fn get_row(&self, row_idx: usize) -> &[Option<Letter>] {
        debug_assert!(row_idx < 6);

        &self.0[row_idx * 5..(row_idx + 1) * 5]
    }

    /// Determines if a row is fully populated
    #[must_use]
    pub fn is_row_complete(&self, row_idx: usize) -> bool {
        self.get_row(row_idx).iter().all(Option::is_some)
    }

    /// Gets the state of the letter to be shown in the keyboard
    ///
    /// This function does not check if the letter state is valid.
    #[must_use]
    pub fn get_letter_state(&self, letter: char) -> Option<LetterState> {
        let mut best = None::<LetterState>;
        for row_idx in (0..6).filter(|&i| self.is_row_complete(i)) {
            let row = self.get_row(row_idx);
            let best_row = row
                .iter()
                .flatten()
                .filter(|a| a.value == letter)
                .map(|a| a.state)
                .max();

            if let Some(best_row) = best_row {
                if let Some(best_val) = &mut best {
                    *best_val = (*best_val).max(best_row);
                } else {
                    best = Some(best_row);
                }
            }
        }

        best
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Letter {
    pub value: char,
    pub state: LetterState,
}

impl Letter {
    pub fn new(letter: char) -> Self {
        Self {
            value: letter,
            state: LetterState::Incorrect,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LetterState {
    /// The letter is not used in the solution
    Incorrect,
    /// The letter is used in the solution, but not in this spot
    Present,
    /// The letter is used in this spot
    Correct,
}
