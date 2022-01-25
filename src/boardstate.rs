// どちらのターンか判定する列挙型
// 駒判別に使用、Copyトレイトで実装
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    White,
    Black,
}

const WHITE: char = '○';
const BLACK: char = '●';
const NO_PIECE: char = '.';

// size-> 盤面のサイズ、高さと幅一緒
// stateは二次元配列、各要素はOption<Turn>型
// NONE-> 駒が置かれていない状態
// Some(Turn::White)-> 白い駒が置かれてる状態
// Some(Turn::Black)-> 黒い駒が置かれてる状態
// turnは現在どちらのターンなのか情報を所持
#[derive(Debug)]
pub struct BoardState {
    size: usize,
    state: Vec<Vec<Option<Turn>>>,
    turn: Turn,
}

impl BoardState {
    // 新しい盤面を作成する
    pub fn new(n: usize, white_turn: bool) -> BoardState {
        assert!(n != 0);
        let mut s: Vec<Vec<Option<Turn>>> = vec![vec![None; 2 * n]; 2 * n];
        s[n - 1][n - 1] = Some(Turn::White);
        s[n - 1][n] = Some(Turn::Black);
        s[n][n - 1] = Some(Turn::Black);
        s[n][n] = Some(Turn::White);

        BoardState {
            size: 2 * n,
            state: s,
            turn: if white_turn { Turn::White } else { Turn::Black },
        }
    }

    // 盤面の大きさを取得
    pub fn get_size(&self) -> usize {
        self.size
    }

    // 盤面の状態をchar型の二次元配列で出力する
    pub fn show_board(&self) -> Vec<Vec<char>> {
        let n = self.size;
        let mut v: Vec<Vec<char>> = vec![vec![NO_PIECE; n]; n];

        for i in 0..n {
            for j in 0..n {
                if let Some(t) = &self.state[i][j] {
                    v[i][j] = match t {
                        Turn::Black => BLACK,
                        Turn::White => WHITE,
                    }
                }
            }
        }
        v
    }

    // 白い駒
    pub fn white_piece() -> char {
        WHITE
    }

    // 黒い駒
    pub fn black_piece() -> char {
        BLACK
    }

    // どっちのターンの駒の文字を出力
    pub fn which_turn(&self) -> char {
        match self.turn {
            Turn::Black => BLACK,
            Turn::White => WHITE,
        }
    }
}