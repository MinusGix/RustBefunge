use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
}

#[derive(Debug)]
struct Board {
    grid: HashMap<(i32, i32), char>,
    starting_pos: Position,
    current_pos: Position,
    width: i32,
    height: i32,
    direction: Direction,
    stack: Vec<u8>,

    _output: String,

    _in_string_mode: bool,
}

impl Board {
    // Stack functions
    fn pop(&mut self) -> u8 {
        match self.stack.pop() {
            Some(val) => val,
            None => 0,
        }
    }

    fn peek(&self) -> u8 {
        match self.stack.get(self.stack.len() - 1) {
            Some(val) => *val,
            None => 0,
        }
    }

    fn push(&mut self, val: u8) {
        self.stack.push(val);
    }

    // Grid functions

    fn set(&mut self, x: i32, y: i32, val: char) {
        self.grid.insert((x, y), val);
    }

    fn get_byte (&self, x: i32, y: i32) -> u8 {
        return self.get(x, y) as u8;
    }

    fn get(&self, x: i32, y: i32) -> char {
        match self.grid.get(&(x, y)) {
            Some(val) => *val,
            None => '\x00',
        }
    }

    fn get_current_cell(&self) -> char {
        self.get(self.current_pos.x, self.current_pos.y)
    }

    fn step(&mut self) -> bool {
        let character = self.get_current_cell();
        let mut is_done = false;

        // If it's currently parsing for a string
        if self._in_string_mode {
            match character {
                '"' => self._in_string_mode = false,
                _ => self.push(character as u8),
            };
        } else {
            match character {
                // Directions/Movement
                '>' => self.direction = Direction::Right,
                '<' => self.direction = Direction::Left,
                '^' => self.direction = Direction::Up,
                'v' => self.direction = Direction::Down,
                // Move in random direction
                '?' => match rand::thread_rng().gen_range(0, 4) {
                    0 => self.direction = Direction::Right,
                    1 => self.direction = Direction::Left,
                    2 => self.direction = Direction::Up,
                    _ => self.direction = Direction::Down,
                },
                // Right if 0, left otherwise
                '_' => match self.pop() {
                    0 => self.direction = Direction::Right,
                    _ => self.direction = Direction::Left,
                },
                // Down if 0, up otherwise
                '|' => match self.pop() {
                    0 => self.direction = Direction::Down,
                    _ => self.direction = Direction::Up,
                },
                // Skip a spot
                '#' => self.move_forward(),

                // 'constants'
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    self.push(character.to_digit(10).unwrap() as u8)
                }

                // Operators
                '+' => {
                    let num1 = self.pop();
                    let num2 = self.pop();

                    self.push(num1 + num2);
                }
                '-' => {
                    let num1 = self.pop();
                    let num2 = self.pop();

                    self.push(num2 - num1);
                }
                '*' => {
                    let num1 = self.pop();
                    let num2 = self.pop();

                    self.push(num1 * num2);
                }
                '/' => {
                    let num1 = self.pop();
                    let num2 = self.pop();

                    if num1 == 0 {
                        panic!("Attempted division by 0.");
                    }

                    self.push(num2 / num1);
                }
                '%' => {
                    let num1 = self.pop();
                    let num2 = self.pop();

                    self.push(num2 % num1);
                }
                '!' => match self.pop() {
                    0 => self.push(1),
                    _ => self.push(0),
                },
                '`' => {
                    let num1 = self.pop();
                    let num2 = self.pop();

                    if num2 > num1 {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }

                // Stack
                ':' => {
                    let val = self.peek();
                    self.push(val);
                }
                '\\' => {
                    let val1 = self.pop();
                    let val2 = self.pop();

                    self.push(val1);
                    self.push(val2);
                }
                '$' => {
                    self.pop();
                }

                // Special
                '"' => self._in_string_mode = true,
                '@' => is_done = true,

                // IO
                // Output popped number
                '.' => {
                    let num = self.pop();
                    self._output.push_str(&format!("{} ", num));
                },
                // Output popped ascii character
                ',' =>{
                    let chr = self.pop() as char;
                    self._output.push(chr);
                },
                // Put a character at a position on the grid
                'p' => {
                    let val = self.pop();
                    let x = self.pop();
                    let y = self.pop();

                    self.set(x as i32, y as i32, val as char);
                },
                // Get a number at a position on the grid
                'g' => {
                    let x = self.pop();
                    let y = self.pop();

                    let val = self.get_byte(x as i32, y as i32);

                    self.push(val);
                },
                // Push user-input number
                '&' => {
                    let res = self._get_input();
                    
                    let num = res.parse::<u8>();

                    let num: u8 = match num {
                        Ok(v) => v,
                        Err(_) => 0,
                    };
                    
                    self.push(num);
                },
                // Push user-input character ascii value
                '~' => {
                    let res = self._get_input();

                    for chr in res.bytes() {
                        self.push(chr);
                    };
                },

                _ => (),
            };
        }

        if is_done {
            return is_done;
        }

        self.move_forward();

        return is_done;
    }

    fn _get_input (&self) -> String {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => 0,
            Err(_) => panic!("No input received from user!"),
        };

        // Remove the \n
        input.pop();

        return input;
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Right => self.move_by(1, 0),
            Direction::Left => self.move_by(-1, 0),
            Direction::Down => self.move_by(0, 1),
            Direction::Up => self.move_by(0, -1),
        }
    }

    fn move_by(&mut self, x_mod: i32, y_mod: i32) {
        self.current_pos.x += x_mod;
        self.current_pos.y += y_mod;

        if self.current_pos.x >= self.width {
            self.current_pos.x = 0;
        }

        if self.current_pos.x < 0 {
            self.current_pos.x = self.width - 1;
        }

        if self.current_pos.y >= self.height {
            self.current_pos.y = 0;
        }

        if self.current_pos.y < 0 {
            self.current_pos.y = self.height - 1;
        }
    }  

    // Only works for ascii :)
    fn get_printable_character(&self, x: i32, y: i32) -> char {
        let character = self.get(x, y);
        let byte = character as u8;

        if (byte <= 32) || (byte >= 128 && byte <= 157) {
            return ' ';
        }

        return character;
    }

    fn get_board_text(&self) -> String {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let at_pos = self.current_pos.x == x && self.current_pos.y == y;

                result.push('|');

                if at_pos {
                    // white background color code
                    result.push_str("\x1b[47m");
                }

                result.push(self.get_printable_character(x, y));

                if at_pos {
                    // reset current colors/formatting
                    result.push_str("\x1b[0m");
                }
            }
            result.push_str("|\n");
        }

        return result;
    }

    fn get_stack_text(&self) -> String {
        let mut result = String::from("[");

        for item in self.stack.iter().rev() {
            result.push_str(&format!(" {}", item));
        }

        result.push(']');

        return result;
    }

    fn new(width: i32, height: i32) -> Board {
        Board {
            grid: HashMap::new(),
            starting_pos: Position { x: 0, y: 0 },
            current_pos: Position { x: 0, y: 0 },
            width: width,
            height: height,
            direction: Direction::Right,
            stack: Vec::new(),

            _output: String::new(),

            _in_string_mode: false,
        }
    }
}

#[derive(Debug)]
struct Position {
    // It's I32 despite not needing negative numbers, because dealing with possible negative numbers (move modifiers) with u32 is a PAIN
    // Was going to try using u32.checked_add but it requires u32 as a parameter! Which doesn't help me because if I want to use a negative
    // number as modifier I need to use a signed int.
    // I could use some fancy if statements in the move_by function but this is easier.
    x: i32,
    y: i32,
}

fn main() {
    let mut board = Board::new(10, 10);
    board.set(1, 0, '~');
    board.set(2, 0, 'v');
    board.set(2, 1, ',');
    board.set(2, 2, '>');
    board.set(3, 2, '^');
    board.set(3, 0, '<');

    loop {
        println!("{}", board.get_board_text());
        println!("{}", board.get_stack_text());
        println!("{}", board._output);
        
        if board.step() {
            // If it's truthy, then the program ended via @
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
