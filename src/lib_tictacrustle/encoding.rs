struct Encoding;

impl Encoding {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, board: &Board) -> String {
        let mut encoded = String::new();
        for row in board.iter() {
            for cell in row.iter() {
                encoded.push(match cell {
                    Cell::Empty => '0',
                    Cell::X => '1',
                    Cell::O => '2',
                });
            }
        }
        encoded
    }

    pub fn decode(&self, encoded: &str) -> Board {
        let mut board = Board::new();
        for (i, c) in encoded.chars().enumerate() {
            let row = i / 3;
            let col = i % 3;
            board[row][col] = match c {
                '0' => Cell::Empty,
                '1' => Cell::X,
                '2' => Cell::O,
                _ => panic!("Invalid character in encoded string"),
            };
        }
        board
    }

    pub fn to_number(&self, board: &Board) -> u16 {
        let mut number = 0;
        for row in board.iter() {
            for cell in row.iter() {
                number = number * 3
                    + match cell {
                        Cell::Empty => 0,
                        Cell::X => 1,
                        Cell::O => 2,
                    };
            }
        }
        number
    }
}
