use std::thread::sleep;
use std::time::Duration;

use rand::seq::SliceRandom;
use rand::Rng;

pub mod boardstate;
use boardstate::BoardState;

fn main() {

    // スタート ==============================
    println!("オセロしますよ！");

    let size: usize = 16;
    let mut cpu_flag = true;
    let mut i_am_white: bool = false;

    // 黒、白の順番決める ==============================
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
        } else {
            error_not_int();
        }
    }

    // 盤面の作成
    let mut bs = BoardState::new(size / 2, false);

    // ゲーム実行 ==============================
    loop {
        // 盤面を表示
        preview_board(&bs);

        // どちらのターン
        preview_turn(&bs);

        if cpu_flag && (i_am_white || bs.is_it_white_turn()) && !(i_am_white && bs.is_it_white_turn()) {
            // 乱数
            let mut rng = rand::thread_rng();

            if (cpu_flag) {
                sleep(Duration::from_millis(250));
            }

            println!("\nCPU操作中...\n");

            sleep(Duration::from_millis(750));

            // 置けるマス目を重み付けしつつVecで管理
            let mut options: Vec<(usize, usize)> = Vec::new();
            let mut options_corners: Vec<(usize, usize)> = Vec::new();
            let vec = &bs.cnt_reversable();
            let n = bs.get_size();
            for i in 0..n {
                for j in 0..n {
                    if vec[i][j] > 0 {
                        for _ in 0..vec[i][j] {
                            options.push((i, j));
                        }
                        if (i == 0 || i == n - 1) && (j == 0 || j == n - 1) {
                            options_corners.push((i, j));
                        }
                    }
                }
            }

            // ランダムに選ぶ
            let &(i, j) = if options_corners.is_empty() {
                options
            } else {
                options_corners
            }
            .choose(&mut rng)
            .unwrap();

            // マス目更新
            let can_continue = bs.put(i, j);

            // 続行出来ない時はループを抜けてゲーム終了
            if !can_continue {
                break;
            }
            continue;
        }

        // 自分の番の場合
        println!("駒を置く場所を，行番号，列番号の順で，Return区切りで入力してください。");
        println!("もうゲームを終わって結果を見たい場合は，1つ目の数字として0を入力してください。");

        // 行番号の受け取り
        let row_num: usize;
        loop {
            let mut row_num_string = String::new();
            std::io::stdin().read_line(&mut row_num_string).ok();
            if let Ok(n) = row_num_string.trim().parse::<usize>() {
                if n < size + 1 || (n == size + 1) {
                    row_num = n;
                    break;
                } else {
                    error_not_int();
                }
            }
        }

        // 終了処理
        if row_num == 0 {
            println!("終了しますか？「はい」ならyes、「いいえ」ならそれ以外を入力して下さい。");
            let mut y_or_no = String::new();
            std::io::stdin().read_line(&mut y_or_no).ok();
            if y_or_no.trim() == "yes" {
                break;
            } else {
                continue;
            }
        }

        // 列番の受け取り
        let column_num: usize;
        loop {
            let mut column_num_string = String::new();
            std::io::stdin().read_line(&mut column_num_string).ok();
            if let Ok(n) = column_num_string.trim().parse::<usize>() {
                if n > 0 && n <= size {
                    column_num = n;
                    break;
                } else {
                    err_not_range();
                }
            } else {
                error_not_int();
            }
        }

        // 置けるマス目かどうか判定
        let v = bs.cnt_reversable();
        if v[row_num - 1][column_num - 1] == 0 {
            println!("そこには置けません。");
            continue;
        }

        // マス目更新
        let can_continue = bs.put(row_num - 1, column_num - 1);

        // 続行できないときはループを抜けてゲームを終了
        if !can_continue {
            break;
        }
    }

    // 盤面表示
    preview_board(&bs);

    // 結果表示
    show_result(&bs);
}

// 盤面を表示
fn preview_board(bs: &BoardState) {
    let v = bs.show_board();
    let n = bs.get_size();

    //println!("{:?}", v);

    print!("  ");
    for i in 1..=n {
        print!("{:2}", i);
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

// 入力値が範囲外のメッセージ
fn err_not_range() {
    println!("入力値が範囲外です。");
}

// 整数の入力値が不正であるメッセージ
fn error_not_int() {
    println!("半角数字で整数を入力して下さい。");
}

// 結果を表示する
fn show_result(bs: &BoardState) {
    let ((c1, s1), (c2, s2)) = bs.count_pieces();
    if s1 > s2 {
        println!("{0}が{1}個，{2}が{3}個で{0}の勝ち！", c1, s1, c2, s2);
    } else if s1 < s2 {
        println!("{0}が{1}個，{2}が{3}個で{2}の勝ち！", c1, s1, c2, s2);
    } else {
        println!("{0}が{1}個，{2}が{3}個で引き分け！", c1, s1, c2, s2);
    }
}