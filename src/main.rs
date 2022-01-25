use std::thread::sleep;
use std::time::Duration;

use rand::seq::SliceRandom;
use rand::Rng;

pub mod boardstate;
use boardstate::BoardState;

fn main() {
    println!("オセロしますよ！");

    let size: usize = 4;
    let mut cpu_flag = true;
    let mut i_am_white: bool = false;

    // 黒、白の順番決める
    loop {
        println!("先行 {0} は1を入力。後攻 {1} は2を入力して下さい。", BoardState::black_piece(), BoardState::white_piece());

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).ok();
        if let Ok(n) = input_string.trim().parse::<usize>() {
            match n {
                1 => {
                    break;
                }
                2 => {
                    i_am_white = true;
                    break;
                }
                _ => {
                    err_not_range();
                }
            }
        }
    }

    // ゲーム実行
    loop {
        // 盤面の作成
        let mut bs = BoardState::new(size / 2, false);

        // 盤面を表示
        preview_board(&bs);

        // どちらのターン
        preview_turn(&bs);

        if (cpu_flag && (i_am_white || bs.is_it_white_turn()) && !(i_am_white && bs.is_it_white_turn())) {
            // 乱数
            let mut rng = rand::thread_rng();

            if (cpu_flag) {
                sleep(Duration::from_millis(250));
            }

            println!("\nCPU操作中...\n");

            sleep(Duration::from_millis(750));

            // 置けるマス目を重み付けしつつVecで管理
            let mut option: Vec<(usize, usize)> = Vec::new();
            let mut option_corners: Vec<(usize, usize)> = Vec::new();
        }
    }
}

// 盤面を表示
fn preview_board(bs: &BoardState) {
    let v = bs.show_board();
    let n = bs.get_size();

    print!("  ");
    for i in 1..=n {
        print!("{:2}", i)
    }
    println!("");

    for i in 0..n {
        print!("{:2}", i + 1);
        for j in 0..n {
            print!(" {}", v[i][j]);
        }
        println!("");
    }
}

// どちらかのターンを表示
fn preview_turn(bs: &BoardState) {
    println!("{} のターン", bs.which_turn())
}

/// 整数の入力が不正である旨のメッセージ
fn err_not_int() {
    println!("半角数字で整数を入力してください。");
}

/// 入力が不適切な旨のメッセージ
fn err_input() {
    println!("入力が不適切です。");
}

// 入力値が範囲外のメッセージ
fn err_not_range() {
    println!("入力値が範囲外です。");
}