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
    pub fn new(n: usize, white_turn: bool) {
        assert!(n != 0);
        let mut s: Vec<Vec<Option<Turn>>> = vec![vec![None; 2 * n]; 2 * n];
        println!(s);
        // BoardState {
        //     size: 1,
        //     state: nil,
        //     turn: Turn.White,
        // }
    }

    // 白い駒
    pub fn white_piece() -> char {
        WHITE
    }

    // 黒い駒
    pub fn black_piece() -> char {
        BLACK
    }
}