mod hangman;

fn main() {
    println!("Welcome to hangman");

    let mut entered_challenge: String = String::new();
    println!("Enter the challenge word: ");
    match std::io::stdin().read_line(&mut entered_challenge) {
        Ok(_) => println!("Received challenge word: {}", entered_challenge),
        Err(error) => println!("error: {}", error),
    };
    entered_challenge.truncate(entered_challenge.len() - 1); //Chops off newline char at end of string

    let guess_word: String = "_".repeat(entered_challenge.len());
    println!("Guessed word: {}", guess_word); 
    let num_lives: u8 = 5;
    let mut hangman_game = hangman::Hangman {
        m_challenge: entered_challenge, 
        m_guessed_word: guess_word, 
        m_guesses: Vec::new(), 
        m_lives: num_lives
    };

    println!("Current guessed word: {}", hangman_game.m_guessed_word);
    println!("Current challenge word: {}", hangman_game.get_challenge());

    while !hangman_game.check_for_loss() {
        let user_guess: char;
        let mut user_guess_buf: String = String::new();
        println!("Current guessed word: {}", hangman_game.m_guessed_word);
        println!("Number of lives remaining: {}", hangman_game.get_lives());
        println!("Guesses made so far: {:?}", hangman_game.m_guesses);
        println!("Enter a guess: ");
        match std::io::stdin().read_line(&mut user_guess_buf) {
            Ok(_) => user_guess_buf.pop().unwrap(),
            Err(error) => {
                println!("Error: {}, guess again", error);
                continue;
            }
        };
        user_guess = user_guess_buf.pop().unwrap();
        if hangman_game.is_valid_guess(user_guess) {
            hangman_game.apply_guess(user_guess);
            if hangman_game.check_for_loss() {
                println!("You lost, game over.");
                return
            }
        } else {
            println!("You already guessed that");
        };
        if hangman_game.check_for_win() {
            println!("You win!");
            return
        };

    };

}
