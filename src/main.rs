pub mod boardstate;
use boardstate::BoardState;

fn main() {
    println!("オセロしますよ！");

    let size: usize = 8;
    // loop {
    //     println!("盤面サイズを4以上の偶数で入力して下さい。Returnキーで確定します。");

    //     let mut size_string = String::new();
    //     std::io::stdin().read_line(&mut size_string).ok();
    //     if let Ok(n) = size_string.trim().parse::<usize>() {
    //         if (n >= 4) && (n % 2 == 0) {
    //             size = n;
    //             break;
    //         } else {
    //             err_input();
    //         }
    //     } else {
    //         err_not_int();
    //     }
    // }
    let cpu_flag: bool = true;
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

    // 盤面の作成
    let mut bs = BoardState::new(size / 2, false);

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
