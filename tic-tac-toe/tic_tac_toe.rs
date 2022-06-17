use std::io::stdin;
use std::io;
use std::io::Write;

// plays the turn for the active player
fn play_turn(board_array: &mut [char; 9], token: char)
{
    // get user input and validate
    let mut incorrect_format = true;
    let mut input = String::new();
    while incorrect_format
    {
        print!("{}> ", token);
        io::stdout().flush().unwrap();
        input = get_input();
        if validate(&input, board_array)
        {
            incorrect_format = false;
        }
    }

    //convert the input into an integer value
    let position = input.parse::<usize>().unwrap();
    update_board_array(position, board_array, token)
}

//gets user input from the user
fn get_input()-> String
{
    let mut input = String::new();
    stdin().read_line(&mut input).ok()
    .expect("failed to read line");

    // remove exess whitespace
    let trimmed = input.trim();
    return trimmed.to_string();
}

// validates the user input
fn validate(input: &String, 
    board_array: &mut [char; 9])-> bool
{
    // confirm input is correct length
    if input.len() != 1
    {
        println!("input: {}, input size: {}", input, input.len());
        for letter in input.chars(){
            println!("element: {}", letter);
        }
        println!("Enter only a single digit.");
        return false;
    }
    // convert string to char
    let c_vec: Vec<char> = input.chars().collect();
    let c_input = c_vec[0];
    
    // confirm input is a number
    if !c_input.is_numeric()
    {
        println!("Enter a digit between 1-9, not any other character");
        return false;
    }
    // confirm input is between 1-9
    let value = input.parse::<usize>().unwrap();
    if value < 1 || value > 9
    {
        println!("Enter a value between 1-9");
        return false;
    }
    // confirm the location is available
    else if board_array[value - 1] == 'X' 
    || board_array[value - 1] == 'O'
    {
        println!("That location is already taken.");
        return false;
    }
    else 
    {
        return true;
    }
}

// displays the current board
fn display_board(board_array: &mut [char; 9])
{
    println!(" {} | {} | {} ", board_array[0], board_array[1], board_array[2]);
    println!("---+---+---");
    println!(" {} | {} | {} ", board_array[3], board_array[4], board_array[5]);
    println!("---+---+---");
    println!(" {} | {} | {} ", board_array[6], board_array[7], board_array[8]);
}

// determines current turn
fn det_turn(board_array: &[char; 9]) -> char
{
    let mut x = 0;
    let mut o = 0;

    // total up the number of x's and o's
    for token in *board_array
    {
        if token == 'X'
        {
            x += 1;
        }

        else if token == 'O'
        {
            o += 1;
        }
    }

    // if there are more x's, then it is o's turn
    if x > o
    {
        return 'O';
    }

    // otherwise it is x's turn.
    else 
    {
        return 'X';
    }
}

// updates the array used for the board
fn update_board_array(position: usize, 
    board_array: &mut [char; 9], 
    token: char)
{
    board_array[position - 1] = token;
}

// checks if the game has entered an end condition
fn check_game_over(board_array: &[char; 9]) -> bool
{
    // if there are 3 of the same token in a row, that token wins
    for row in 0..3
    {
        if board_array[(row * 3)] == board_array[(row * 3) + 1] 
        && board_array[(row * 3)] == board_array[(row * 3) + 2]
        {
            println!("{} wins!!", board_array[row * 3]);
            return true;
        }
    }

    // if there are 3 of the same token in a column, that token wins
    for col in 0..3
    {
        if board_array[col] == board_array[col + 3]
        && board_array[col] == board_array[col + 6]
        {
            println!("{} wins!!", board_array[col]);
            return true;
        }
    }

    // if there are 3 of the same token in a diagnal, that token wins

    if (board_array[0] == board_array[4] 
    && board_array[0] == board_array[8])
    ||
    (  board_array[2] == board_array[4]
    && board_array[2] == board_array[6])
    {
        println!("{} wins!!", board_array[4]);
        return true;
    }

    // if all the spaces have been filled, then the game ends in a tie
    for tile in *board_array
    {
        if tile != 'X' && tile != 'O'
        {
            return false;
        }
    }

    println!("The game ends in a tie.");
    return true;
}

// plays the game of tic tac toe
fn main()
{
    let mut play_again = true;
    while play_again 
    {
        // set up game
        let mut board = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut not_done = true;
        while not_done
        {
            // play the game
            display_board(&mut board);
            let token = det_turn(&board);
            play_turn(&mut board, token);
            
            // check if the game is over
            if check_game_over(&board)
            {
                not_done = false;
            }
        }

        // ask the user if they wish to play again
        let mut invalid_answer = true;
        while invalid_answer
        {
            print!("Play again(y/n): ");
            io::stdout().flush().unwrap();
            let mut answer = get_input();
            answer = answer.to_lowercase();
            if answer == "n"
            {
                play_again = false;
                invalid_answer = false;
            }

            else if answer == "y"
            {
                invalid_answer = false;
            }
            else
            {
                println!("Enter either 'y' or 'n'")
            }
        }
    }

}