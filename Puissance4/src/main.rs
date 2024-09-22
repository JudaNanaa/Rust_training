use std::io;

use colored::Colorize;
struct Game {
    tab : [[char; 7]; 6],
    width : i8,
    height : i8,
}

fn get_input(prompt: String) -> u8 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input = input.trim().to_string();
        match input.parse::<u8>() {
            Err(_) => println!("Bad input: '{}'", input),
            Ok(val) => {
                if val >= 1 && val <= 7 {
                    return val;
                } else {
                    println!("Not a good number: '{}'", input)
                }
            },
        }
    }
}

impl Game {
    fn new() -> Game {
        let tab = [[' '; 7]; 6];
        Game {
            tab,
            width : 7,
            height : 6,
        }
    }
    fn print(&self) {
        std::process::Command::new("clear").status().unwrap();
        for index in 1..=7 {
            print!("  {} ", index);
        }
        println!("");
        for line in self.tab {
            for elem in line {
                match elem {
                    'r' | 'R' => print!("| {} ", elem.to_string().red()), 
                    _ => print!("| {} ", elem.to_string().blue()),
                }
            }
            println!("|");
            for _ in line {
                print!("----");
            }
            println!("-");
        }
    }
    fn add_token_tab(&mut self, x : u8, c : char) -> i32 {
        let mut y = 0;
        while y < self.height - 1 {
            if self.tab[(y + 1) as usize][(x - 1) as usize] != ' ' {
                break;
            }
            y += 1;
        }
        if self.tab[y as usize][(x - 1) as usize] != ' ' {
            println!("This collumn is full");
            return 0;
        } else {
            self.tab[y as usize][(x - 1) as usize] = c;
        }
        1
    }
    fn check_horizontaly(&self, x : i8, y : i8, c : char, depth : i32) -> u32 {
        if y < 0 || y == self.height || x < 0 || x == self.width {
            return 0;
        }
        if self.tab[y as usize][x as usize] != c { 
            return 0;
        }
        if depth == 4 {
            return 1;
        }
        self.check_horizontaly(x + 1, y, c, depth + 1)
    }
    fn check_verticaly(&self, x : i8, y : i8, c : char, depth : i32) -> u32 {
        if y < 0 || y == self.height || x < 0 || x == self.width {
            return 0;
        }
        if self.tab[y as usize][x as usize] != c { 
            return 0;
        }
        if depth == 4 {
            return 1;
        }
        self.check_verticaly(x, y + 1, c, depth + 1)
    }
    fn check_diagonaly_up(&self, x : i8, y : i8, c : char, depth : i32) -> u32 {
        if y < 0 || y == self.height || x < 0 || x == self.width {
            return 0;
        }
        if self.tab[y as usize][x as usize] != c { 
            return 0;
        }
        if depth == 4 {
            return 1;
        }
        self.check_diagonaly_up(x + 1, y - 1, c, depth + 1)
    }
    fn check_diagonaly_down(&self, x : i8, y : i8, c : char, depth : i32) -> u32 {
        if y < 0 || y == self.height || x < 0 || x == self.width {
            return 0;
        }
        if self.tab[y as usize][x as usize] != c { 
            return 0;
        }
        if depth == 4 {
            return 1;
        }
        return self.check_diagonaly_down(x + 1, y + 1, c, depth + 1)
    }

    fn change_char_horizontaly(&mut self, x : i8, y : i8, c : char, depth : i32) {
        if self.tab[y as usize][x as usize] == 'r' {
            self.tab[y as usize][x as usize] = 'R';
        } else {
            self.tab[y as usize][x as usize] = 'B';
        }
        if depth == 4 {
            return;
        }
        self.change_char_horizontaly(x + 1, y, c, depth + 1);
    }
    fn change_char_verticaly(&mut self, x : i8, y : i8, c : char, depth : i32) {
        if self.tab[y as usize][x as usize] == 'r' {
            self.tab[y as usize][x as usize] = 'R';
        } else {
            self.tab[y as usize][x as usize] = 'B';
        }
        if depth == 4 {
            return;
        }
        self.change_char_verticaly(x, y + 1, c, depth + 1);
    }
    fn change_char_diagonaly_up(&mut self, x : i8, y : i8, c : char, depth : i32) {
        if self.tab[y as usize][x as usize] == 'r' {
            self.tab[y as usize][x as usize] = 'R';
        } else {
            self.tab[y as usize][x as usize] = 'B';
        }
        if depth == 4 {
            return;
        }
        self.change_char_diagonaly_up(x + 1, y - 1, c, depth + 1);
    }
    fn change_char_diagonaly_down(&mut self, x : i8, y : i8, c : char, depth : i32) {
        if self.tab[y as usize][x as usize] == 'r' {
            self.tab[y as usize][x as usize] = 'R';
        } else {
            self.tab[y as usize][x as usize] = 'B';
        }
        if depth == 4 {
            return;
        }
        self.change_char_horizontaly(x + 1, y + 1, c, depth + 1);
    }

    fn check_victory(&mut self, c : char) -> bool {
        for y in 0..6 {
            for x in 0..7 {
                if self.check_horizontaly(x, y, c, 1) == 1 {
                    self.change_char_horizontaly(x, y, c, 1);
                    return true;
                }
                if self.check_verticaly(x, y, c, 1) == 1 {
                    self.change_char_verticaly(x, y, c, 1);
                    return true;
                }
                if self.check_diagonaly_up(x, y, c, 1) == 1 {
                    self.change_char_diagonaly_up(x, y, c, 1);
                    return true;
                }
                if self.check_diagonaly_down(x, y, c, 1) == 1 {
                    self.change_char_diagonaly_down(x, y, c, 1);
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let mut tab = Game::new();
    loop {
        tab.print();
        loop {
            if tab.add_token_tab(get_input(format!("Player {} to play !", "Red".red())), 'r') == 1 {
                break;
            }
            tab.print();
        }
        tab.print();
        if tab.check_victory('r') == true {
            tab.print();
            println!("Player Red win !!");
            break;
        }
        loop {
            if tab.add_token_tab(get_input(format!("Player {} to play !", "Blue".blue())), 'b') == 1 {
                break;
            }
        }
        if tab.check_victory('b') == true{
            tab.print();
            println!("Player Blue win !!");
            break;
        }
    }
}
