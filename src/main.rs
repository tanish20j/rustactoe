use std::io;

#[derive(PartialEq, Eq,Clone, Copy)]
enum GridPoint {
    PLAYERX,
    PLAYERO,
    EMPTY
}

struct Board {
    grid: Vec<GridPoint>,
    grid_size: u8,
    win_streak: u8,
}

impl Board {
    pub fn default() -> Self{
        let mut new_grid: Vec<GridPoint> = Vec::new();
        for x in 0..9 {
            new_grid.push(GridPoint::EMPTY);
        }
        Self {
            grid: new_grid,
            grid_size: 3,
            win_streak: 3
        }
    }

    pub fn custom(gride_size: u8, win_streak: u8) -> Self{
        let mut new_grid: Vec<GridPoint> = Vec::new();
        for _ in 1..(gride_size*gride_size) {
            new_grid.push(GridPoint::EMPTY);
        }
        Self {
            grid: new_grid,
            grid_size: gride_size,
            win_streak: win_streak
        }
    }

    pub fn get_winner(&self) -> Option<&GridPoint> {
        // let mut i:u8 = 0;
        // let mut j:u8 = 0;
        for i in 0..self.grid_size{
            for j in 0..self.grid_size{
                match &self.grid[ (i*(self.grid_size) + j) as usize] {
                    GridPoint::EMPTY => {
                        continue;
                    },
                    player => {
                        let match_flag = false;
                        if i  >= self.win_streak-1 && j + (self.win_streak-1) < self.grid_size {
                            if (1..self.win_streak).all(|x| player == &self.grid[ ((i-x)*(self.grid_size) + j+x) as usize]) {
                                return Some(player);
                            }
                        }else if i + (self.win_streak-1) < self.grid_size && j + (self.win_streak-1) < self.grid_size  {
                            if (1..self.win_streak).all(|x| player == &self.grid[ ((i+x)*(self.grid_size) + j+x) as usize]) {
                                return Some(player);
                            }
                        }else if j + (self.win_streak-1) < self.grid_size {
                            if (1..self.win_streak).all(|x| player == &self.grid[ ((i)*(self.grid_size) + j+x) as usize]) {
                                return Some(player);
                            }
                        }else if i + (self.win_streak-1) < self.grid_size  {
                            if (1..self.win_streak).all(|x| player == &self.grid[ ((i+x)*(self.grid_size) + j) as usize]) {
                                return Some(player);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn display(&self) {
        println!("Board :");
        for i in 0..self.grid_size{
            for j in 0..self.grid_size{
                match self.grid[(i*(self.grid_size) + j) as usize] {
                    GridPoint::EMPTY =>{
                        match j {
                            0 => print!("     "),
                            x if x == self.grid_size -1 => print!("|     \n"),
                            _ =>  print!("|     "),
                        }
                    },
                    GridPoint::PLAYERO =>{
                        match j {
                            0 => print!("  O  "),
                            x if x == self.grid_size - 1 => print!("|  O  \n"),
                            _ =>  print!("|  O  "),
                        }
                    },
                    GridPoint::PLAYERX =>{
                        match j {
                            0 => print!("  X  "),
                            x if x == self.grid_size-1 => print!("|  X   \n"),
                            _ =>  print!("|  X  "),
                        }
                    },
                }
            }
            match i {
                x if x == self.grid_size-1 => continue,
                _ => {
                    for _ in 0..self.grid_size{
                        print!("------")
                    }
                    println!("")
                }
            }
        }
    }

    pub fn insert(&mut self,i: u8,j: u8,player:GridPoint) -> bool {
        match self.grid[ (i*(self.grid_size) + j) as usize] {
            GridPoint::PLAYERO | GridPoint::PLAYERX => false,
            GridPoint::EMPTY => {
                self.grid[ (i*(self.grid_size) + j) as usize] = player;
                true
            }
        }
    }

    
}

struct Game {
    board: Board,
    running: bool,
    turn_count: u8
}

impl Game {
    pub fn default() -> Self {
        Self { board: Board::default(), running: true, turn_count: 0 }
    }

    fn input(&mut self,player:GridPoint) {
        let mut input = String::new();
        loop {
            
            match player {
                GridPoint::PLAYERO => println!("Enter position for player O "),
                GridPoint::PLAYERX => println!("Enter position for player X "),
                GridPoint::EMPTY => print!(""),
            };
            println!("Enter position row:");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let i: u8 = input.trim().parse().expect("Invalid input");

            println!("Enter position coloums:");
            input.clear();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let j: u8 = input.trim().parse().expect("Invalid input");
            input.clear();
            if i > self.board.grid_size {
                println!("Invalid row position");
                continue;
            }
            if j > self.board.grid_size {
                println!("Invalid coloumn position");
                continue;
            }
            let success = self.board.insert(i, j, player);

            match success {
                true => return,
                false => println!("Invalid position"), 
            };
        }
    }
    pub fn game_loop(&mut self) {
        self.board.display();
        while self.running {
            let player = match self.turn_count % 2 {
                0 => GridPoint::PLAYERX,
                1 => GridPoint::PLAYERO,
                _ => GridPoint::EMPTY,
            };
            self.input(player);
            self.board.display();
            let winner = self.board.get_winner();
            self.turn_count +=1;
            match winner {
                Some(x) => {
                    match x {
                        GridPoint::PLAYERX => {
                            println!("Player X wins");
                            self.running = false
                        },
                        GridPoint::PLAYERO => {
                            println!("Player O wins");
                            self.running = false
                        },
                        _ => {}
                    };
                }
                None => {
                    if self.turn_count >=9 {
                        println!("Its a Tie");
                        self.running = false
                    }
                }
                
            }
            
        }
    }
    
}

fn main() {
    let mut game = Game::default();
    game.game_loop();
}
