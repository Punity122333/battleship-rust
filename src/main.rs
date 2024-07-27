use indexmap::IndexMap;
use rand::Rng;

fn make_board() -> IndexMap<String, i32> {
    let mut board: IndexMap<String, i32> = IndexMap::new();
    for i in 1..=9 {
        for j in 1..=9 {
            board.insert(format!("{}{}", i, j), 0);
        }
    }
    board
}

fn fill_secret_board(secret_board1: &mut IndexMap<String, i32>,secret_board2: &mut IndexMap<String, i32>) {
    let ver_or_hor = rand::thread_rng().gen_range(0..2);
    println!("{}",ver_or_hor);
    
    for _i in 1..=3 {
        let start_row = rand::thread_rng().gen_range(1..=5);
        let start_col = rand::thread_rng().gen_range(1..=9);
        let end_col = start_col + 2;
        for i in start_col..=end_col {
            secret_board1.insert(format!("{}{}", i, start_row), 1);
        }
        let start_row = rand::thread_rng().gen_range(1..=5);
        let start_col = rand::thread_rng().gen_range(1..=9);
        let end_col = start_col + 2;
        for i in start_col..=end_col {
            secret_board2.insert(format!("{}{}", i, start_row), 1);
        }
    }

    for _i in 1..=3 {
        let start_row = rand::thread_rng().gen_range(1..=9);
        let start_col = rand::thread_rng().gen_range(1..=5);
        let end_row = start_row + 2;
        for i in start_row..=end_row {
            secret_board1.insert(format!("{}{}", start_col, i), 1);
        }
        let start_row = rand::thread_rng().gen_range(1..=9);
        let start_col = rand::thread_rng().gen_range(1..=5);
        let end_row = start_row + 2;
        for i in start_row..=end_row {
            secret_board2.insert(format!("{}{}", start_col, i), 1);
            
        }
    }

    
}

fn print_board(board1: &IndexMap<String, i32>,board2: &IndexMap<String, i32>)  {
    print!("|------------------|\n|");
    
    for i in 1..=9 {
        for j in 1..=9 {
            print!("{} ", board1.get(&format!("{}{}", i, j)).unwrap_or(&0));
            if j == 9 && i != 9{
                print!("|\n|");
            }
            else if j == 9{
                print!("|\n");
            }
            
        }
    }
    print!("|------------------|\n|");
    for i in 1..=9 {
        for j in 1..=9 {
            print!("{} ", board2.get(&format!("{}{}", i, j)).unwrap_or(&0));
            if j == 9 && i != 9{
                print!("|\n|");
            }
            else if j == 9{
                print!("|\n");
            }
            
        }
    }
    println!("|------------------|\n"); 
}

fn convert_alphabet_to_number(coord: &str) -> usize {
    // Split the input by comma and get the first and second elements (row and column)
    let mut coords: Vec<&str> = coord.split(",").collect();
    coords[1] = coords[1].trim();
    let alphadict: IndexMap<&str, i32> = IndexMap::from([("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5), ("f", 6),("g", 7), ("h", 8), ("i", 9)]);
    let row: usize = *alphadict.get(coords[0]).unwrap() as usize;
    let col: usize = coords[1].parse::<usize>().unwrap();
    let final_coord: usize = row*10+col;
    final_coord
}



fn main() {
    let mut secret_board1: IndexMap<String, i32> = make_board();
    let mut secret_board2: IndexMap<String, i32> = make_board();
    fill_secret_board(&mut secret_board1, &mut secret_board2);
    
    let mut board1: IndexMap<String, i32> = make_board();
    let mut board2: IndexMap<String, i32> = make_board();
    // let alphadict: IndexMap<&str, i32> = IndexMap::from([("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5), ("f", 6)]);
    print_board(&board1, &board2);
    let mut player: i32 = 1;
    loop {
        let mut placement: String = String::new();
        println!("Player {}, enter your coordinates (e.g., a1):", player);
        std::io::stdin().read_line(&mut placement).expect("Failed to read line");
        if placement.trim() == "q" {
            break
        }
        let placement_int: i32 = convert_alphabet_to_number(&placement) as i32;
        placement = placement_int.to_string();
        if player == 1 {
            if board1.get(&placement.clone()).unwrap() != &0 {
                println!("Already occupied. Try again.");
                continue;
            } else {
                board1.insert(placement.clone(), player);
            }
            if secret_board1.get(&placement.clone()).unwrap_or(&0) != &0 {
                secret_board1.insert(placement.clone(), 0);
                println!("Hit!");
            } else {
                println!("Miss!");
            }
        } else {
            if board2.get(&placement.clone()).unwrap() != &0 {
                println!("Already occupied. Try again.");
                continue;
            } else {
                board2.insert(placement.clone(), player-1);
            }
            if secret_board2.get(&placement.clone()).unwrap_or(&0) != &0 {
                secret_board2.insert(placement.clone(), 0);
                println!("Hit!");
            } else {
                println!("Miss!");
            }

        }

        
        player = 3 - player;
        print_board(&board1, &board2);

    }
}