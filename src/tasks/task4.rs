use crate::task_handler::get_task;

pub fn task4_1() -> String {
    let input = get_task(4)
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|line| {
            line.split(&[',', ' '])
                .filter(|&x| !x.is_empty())
                .map(|val| val.parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    println!("{:#?}", input);
    let mut game_state = GameState::new_game_state(input);
    let best_board = game_state.evaluate_best_board().0;
    best_board.get_score().to_string()
}

struct GameState {
    input: Vec<u8>,
    boards: Vec<Board>,
}

impl GameState {
    fn new_game_state(mut input: Vec<Vec<u8>>) -> GameState {
        let mut game_input = input.remove(0);
        game_input.reverse();
        let boards = input.chunks(5).map(|f| Board::new_board(f)).collect();
        GameState {
            input: game_input,
            boards,
        }
    }

    pub fn evaluate_best_board<'a>(&'a mut self) -> (&'a Board, usize) {
        for input in &self.input {
            {
                self.boards.iter_mut().for_each(|board| {
                    board.mark_number(*input);
                })
            };

            let winning_board = self.check_wins();
            if winning_board.is_some() {
                return self.check_wins().unwrap();
            }
        }

        panic!("no solution found.")
    }

    fn check_wins<'b>(&'b self) -> Option<(&'b Board, usize)> {
        let winning_board = self.boards.iter().position(|board| board.check_win_board());
        match winning_board {
            Some(index) => Some((&self.boards[index], index)),
            None => None,
        }
    }
}

struct Board {
    board: [[Field; 5]; 5],
    last_called: u8,
}

impl Board {
    pub fn new_board(input: &[Vec<u8>]) -> Board {
        let board = input
            .iter()
            .map(|x| -> [Field; 5] {
                x.iter()
                    .copied()
                    .map(|y| -> Field { y.into() })
                    .collect::<Vec<Field>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("Size of board was incorrect"))
            })
            .collect::<Vec<[Field; 5]>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Size of board was incorrect"));
        Board {
            board,
            last_called: 0,
        }
    }

    fn get_columns(&self) -> Vec<Vec<Field>> {
        let v = self.board;
        assert!(!v.is_empty());
        (0..v[0].len())
            .map(|i| {
                v.iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<Field>>()
            })
            .collect()
    }

    fn check_win_vec(line_vec: &Vec<Field>) -> bool {
        line_vec.iter().all(|x| x.chosen)
    }

    pub fn check_win_board(&self) -> bool {
        self.board.iter().any(|x| Self::check_win_vec(&x.to_vec()))
            | self.get_columns().iter().any(|x| Self::check_win_vec(x))
    }

    pub fn mark_number(&mut self, val: u8) {
        let mut flattened = self.board.iter_mut().flatten();
        let option_flattened = flattened.position(|x| x.number == val);
        match option_flattened {
            Some(x) => {
                let x = self.board.iter().flatten().nth(x).unwrap();
                (*x).chosen = true;
            }
            None => (),
        };
    }

    pub fn get_score(&self) -> usize {
        let (_, unmarked) = self.split_marked();
        unmarked.iter().fold(0, |a, b| a + b.number) as usize * self.last_called as usize
    }

    fn split_marked(&self) -> (Vec<Field>, Vec<Field>) {
        self.board.iter().flatten().partition(|field| field.chosen)
    }
}

#[derive(Clone, Copy)]
struct Field {
    number: u8,
    chosen: bool,
}

impl Field {
    pub fn new(number: u8) -> Field {
        Field {
            number,
            chosen: false,
        }
    }

    pub fn set_chosen(&mut self, x: bool) {
        self.chosen = x;
    }
}

impl From<u8> for Field {
    fn from(number: u8) -> Self {
        Field {
            number,
            chosen: false,
        }
    }
}
