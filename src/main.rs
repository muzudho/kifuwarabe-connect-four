mod command_line_seek;
mod computer_player;
mod engine;
mod log;
mod look_and_model;
mod performance_measurement;
mod position;
mod test;
mod uxi_protocol;
mod win_lose_judgment;

use crate::computer_player::Evaluation;
use crate::engine::*;
use crate::log::LogExt;
use crate::test::test;
use casual_logger::{Level, Log, Table};
use std::time::Instant;

fn main() {
    // Log file name.
    // ログ ファイル名。
    Log::set_file_name("kifuwarabe-connect-four");
    // Log level.
    // ログ レベル。
    Log::set_level(Level::Debug);
    // Log file retention days.
    // ログ ファイル保持日数。
    Log::set_retention_days(2);
    // Remove old log files. This is determined by the date in the filename.
    // 古いログファイルを削除します。これは、ファイル名の日付によって決定されます。
    Log::remove_old_logs();
    println!("Hello, world!");

    // Test.
    // テスト。
    if Log::enabled(Level::Debug) {
        test();
    }

    let mut engine = Engine::default();
    engine.title();

    // End the loop with 'quit'. Forced termination with [Ctrl]+[C].
    // 'quit' でループを終了。 [Ctrl]+[C] で強制終了。
    loop {
        let mut line: String = String::new();
        // Wait for command line input from standard input.
        // 標準入力からのコマンドライン入力を待機します。
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            // Tips. You can separate error numbers by simply specifying the line number.
            // テクニック。 エラー番号は行番号を振っておくだけで少しはばらけます。
            Err(e) => panic!(Log::print_fatal(&format!(
                "(Err.373) Failed to read line. / {}",
                e
            ))),
        };

        if let Some(response) = engine.enter(&line) {
            match response {
                Response::Quit => {
                    break;
                }
            }
        }
    }

    // Wait for logging to complete.
    // ロギングが完了するまで待ちます。
    Log::flush();
}

/// The addresses of the squares start with 0 and end with 41.  
/// The array starts at 0, so the size is 42.  
/// マスの番地は 0 から始まり 41 で終わります。  
/// 配列は 0 から始まるのでサイズは 42 です。  
pub const BOARD_LEN: usize = 42;

/// There are 7 columns from a to g.  
/// a～gの7列です。  
pub const FILE_LEN: usize = 7;

/// There are 6 rows from 1 to 6.  
/// 1～6の6列です。  
pub const RANK_LEN: usize = 6;

/// The maximum number of stones that can be placed on the board.  
/// Since there are only 42 squares, it will be 42.  
/// 盤上に置ける石の最大数。  
/// 42マスしかないから42です。  
pub const SQUARES_NUM: usize = 42;

/// Circle and cross mark. It corresponds to the stone in Go.  
/// 丸と十字の印です。囲碁で言うところの石に当たります。  
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// 〇
    Nought,
    /// ×
    Cross,
}

/// ThinkingEngine.  
/// 思考エンジン。  
pub struct Engine {
    /// Starting position.  
    /// 初期局面。  
    pos: Position,
}

/// It is a game that can be playout, so please use the result instead of the evaluation value.  
/// プレイアウトできるゲームなので、評価値ではなく結果を使います。  
#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
pub struct Position {
    /// Turn. The stone to be placed next at the start.  
    /// 開始局面での手番。次に置かれる石。  
    pub starting_turn: Piece,

    /// The board at the start.  
    /// 開始時の盤面。  
    pub starting_board: [Option<Piece>; BOARD_LEN],

    /// The number of stones on the board at the start.  
    /// 開始時に盤の上に有った石の数。  
    pub starting_pieces_num: usize,

    /// Turn. The stone to be placed next.  
    /// 手番。次に置かれる石。  
    pub turn: Piece,

    /// The current board.  
    /// 現在の盤面。  
    pub board: [Option<Piece>; BOARD_LEN],

    /// Match record. An array of files where the pieces will be placed.  
    /// 棋譜。駒を置いた筋を並べたもの。  
    pub history: [char; SQUARES_NUM],

    /// The number of stones currently on the board.  
    /// 現在、盤の上に有る石の数。  
    pub pieces_num: usize,

    /// Principal variation.
    /// 今読んでる読み筋。
    pub pv: String,

    /// Display info during search. It is not info level in the log.  
    /// 探索中の info 表示を行います。 ログの情報レベルのことではありません。  
    pub info_enabled: bool,
}

/// Proceeding from the root toward the leaves is called a forward search.  
/// The process of returning from the leaves toward the root is called backward search.  
/// 根から葉に向かって進んでいることを前向き探索と呼びます。  
/// 葉から根に戻っていることを後ろ向き探索と呼びます。  
pub enum SearchDirection {
    /// Forward search.
    /// 前向き探索。
    Forward,
    /// Backward search.
    /// 後ろ向き探索。
    Backward,
}

/// Search.  
/// 探索部。  
pub struct Search {
    /// The number of stones on the board at the start of this search.  
    /// この探索の開始時に盤の上に有った石の数。  
    pub start_pieces_num: usize,
    /// Number of state nodes searched.  
    /// 探索した状態ノード数。  
    pub nodes: u32,
    /// Start the stopwatch when this structure is created.  
    /// この構造体を生成した時点からストップ・ウォッチを開始します。  
    pub stopwatch: Instant,
    /// Evaluation.
    /// 評価値。
    pub evaluation: Evaluation,
}

/// Win evaluation and draw evaluation.  
/// 勝ち評価値と、引き分け評価値。  
#[derive(Clone, Copy, Debug)]
pub enum EvaluationWay {
    Win,
    Draw,
}

/// # Return
///
/// square on top row.
/// 最上段のマス。
fn file_to_num(file: char) -> u8 {
    match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!(Log::print_fatal_t(
            "(Err.32) Invalid file.",
            Table::default().char("file", file)
        )),
    }
}
