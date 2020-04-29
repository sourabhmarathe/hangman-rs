pub struct Hangman {
    pub m_challenge: String,
    pub m_guessed_word: String,
    pub m_guesses: Vec<char>,
    pub m_lives: u8,
}

impl Hangman {
    pub fn _set_challenge(&mut self, challenge_word: String) {
        self.m_challenge = challenge_word;
    }

    pub fn _set_guessed_word(&mut self, new_word: String) {
        self.m_guessed_word = new_word;
    }

    pub fn _set_lives(&mut self, num_lives: u8) {
        self.m_lives = num_lives;
    }

    pub fn add_guess(&mut self, guess: char) {
        self.m_guesses.push(guess);
    }

    pub fn get_challenge(&self) -> &String {
        &self.m_challenge
    }

    pub fn get_lives(&self) -> &u8 {
        &self.m_lives
    }

    pub fn check_for_win(&self) -> bool {
        self.m_guessed_word == self.m_challenge
    }

    pub fn check_for_loss(&self) -> bool {
        !self.m_lives == 0
    }


    pub fn is_valid_guess(&self, guess: char) -> bool {
        let mut result: bool = true;
        for val in &self.m_guesses {
            if guess == *val {
                result = false;
            }
        };
        result
    }
    
    pub fn apply_guess(&mut self, guess: char) {
        self.add_guess(guess);
        let mut updated_guess: String = String::with_capacity(self.m_challenge.len());
        {
            let challenge_chars = self.m_challenge.chars();
            let mut found_change: bool = false;
            for c in challenge_chars.enumerate() {
                if c.1 == guess {
                    found_change = true;
                    updated_guess.push(c.1);
                } else {
                    if self.m_guesses.contains(&c.1) {
                        updated_guess.push(c.1);
                    } else {
                        updated_guess.push('_');
                    }
                }
            }
            self.m_guessed_word = updated_guess;
            if !found_change {
                self.m_lives = self.m_lives - 1;
                if self.m_lives == 0 {
                    println!("You lose.");
                    return
                }
            }
        }
    }
}