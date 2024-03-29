use crate::task_handler::get_task;

pub fn task_1() -> String {
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
    let mut game_state = GameState::new_game_state(input);
    let best_board = game_state.evaluate_best_board().0;
    best_board.get_score().to_string()
}

pub fn task_2() -> String {
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
    let mut game_state = GameState::new_game_state(input);
    let worst_board = game_state.evaluate_worst_board().0;
    worst_board.get_score().to_string()
}
struct GameState {
    input: Vec<u8>,
    boards: Vec<Board>,
}

impl GameState {
    fn new_game_state(mut input: Vec<Vec<u8>>) -> GameState {
        let game_input = input.remove(0);
        let boards = input.chunks(5).map(Board::new_board).collect();
        GameState {
            input: game_input,
            boards,
        }
    }

    pub fn evaluate_best_board(&mut self) -> (&Board, usize) {
        for input in &self.input {
            self.boards.iter_mut().for_each(|board| {
                board.mark_number(*input);
            });

            let winning_board = self.check_wins();
            if winning_board.is_some() {
                return self.check_wins().unwrap();
            }
        }

        panic!("no solution found.")
    }

    pub fn evaluate_worst_board(&mut self) -> (&Board, usize) {
        for input in &self.input {
            self.boards.iter_mut().for_each(|board| {
                board.mark_number(*input);
            });

            let mut prev_num = 0;
            while prev_num != self.boards.len() {
                prev_num = self.boards.len();
                let winning_board = self.check_wins();
                if winning_board.is_none() {
                    continue;
                }
                if self.boards.len() == 1 {
                    return self.check_wins().unwrap();
                }
                let x = winning_board.unwrap().1.to_owned();
                self.boards.remove(x);
            }
        }

        panic!("no solution found.")
    }

    fn check_wins(&self) -> Option<(&Board, usize)> {
        let winning_board = self.boards.iter().position(|board| board.check_win_board());
        winning_board.map(|index| (&self.boards[index], index))
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
            .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<Field>>())
            .collect()
    }

    fn check_win_vec(line_vec: &[Field]) -> bool {
        line_vec.iter().all(|x| x.chosen)
    }

    pub fn check_win_board(&self) -> bool {
        self.board.iter().any(|x| Self::check_win_vec(&x.to_vec()))
            | self.get_columns().iter().any(|x| Self::check_win_vec(x))
    }

    pub fn mark_number(&mut self, val: u8) {
        let mut flattened = self.board.iter_mut().flatten();
        let option_flattened = flattened.position(|x| x.number == val);
        if let Some(x) = option_flattened {
            let x: &mut Field = self.board.iter_mut().flatten().nth(x).unwrap();
            self.last_called = val;
            (*x).chosen = true;
        };
    }

    pub fn get_score(&self) -> usize {
        let (_, unmarked) = self.split_marked();
        unmarked
            .iter()
            .fold(0, |a, b| a as usize + b.number as usize) as usize
            * self.last_called as usize
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

impl From<u8> for Field {
    fn from(number: u8) -> Self {
        Field {
            number,
            chosen: false,
        }
    }
}
