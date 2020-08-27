//! Display and data structure.  
//! 表示と、データ構造です。  
use crate::computer_player::evaluation::Evaluation;
use std::fmt;
use std::time::Instant;

/// Circle and cross mark. It corresponds to the stone in Go.  
/// 丸と十字の印です。囲碁で言うところの石に当たります。  
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// 〇
    Nought,
    /// ×
    Cross,
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::look_and_model::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
}

/// It is a game that can be playout, so please use the result instead of the evaluation value.  
/// プレイアウトできるゲームなので、評価値ではなく結果を使います。  
#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::look_and_model::GameResult::*;
        match self {
            Win => write!(f, "win"),
            Draw => write!(f, "draw"),
            Lose => write!(f, "lose"),
        }
    }
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
impl Default for Position {
    fn default() -> Self {
        Position {
            starting_turn: Piece::Nought,
            starting_board: [None; BOARD_LEN],
            starting_pieces_num: 0,
            turn: Piece::Nought,
            board: [None; BOARD_LEN],
            history: [' '; SQUARES_NUM],
            pieces_num: 0,
            pv: String::new(),
            info_enabled: true,
        }
    }
}
impl Position {
    /// Display of square.  
    /// マスの表示。  
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.board[index] {
            format!(" {} ", piece)
        } else {
            "   ".to_string()
        }
    }
    /// Display of position.  
    /// 局面の表示。  
    pub fn pos(&self) -> String {
        let s = &mut format!(
            "[Next {} piece(s) | Go {}]

",
            self.pieces_num + 1,
            self.turn
        );
        s.push_str(&format!(
            "  +---+---+---+---+---+---+---+ Please select a file. Example `do d`
6 |{0}|{1}|{2}|{3}|{4}|{5}|{6}| 列を選んでください。例 `do d`
  +---+---+---+---+---+---+---+
5 |{7}|{8}|{9}|{10}|{11}|{12}|{13}|    a b c d e f g
  +---+---+---+---+---+---+---+
4 |{14}|{15}|{16}|{17}|{18}|{19}|{20}|
  +---+---+---+---+---+---+---+
3 |{21}|{22}|{23}|{24}|{25}|{26}|{27}|
  +---+---+---+---+---+---+---+
2 |{28}|{29}|{30}|{31}|{32}|{33}|{34}|
  +---+---+---+---+---+---+---+
1 |{35}|{36}|{37}|{38}|{39}|{40}|{41}|
  +---+---+---+---+---+---+---+
    a   b   c   d   e   f   g",
            self.cell(0),
            self.cell(1),
            self.cell(2),
            self.cell(3),
            self.cell(4),
            self.cell(5),
            self.cell(6),
            self.cell(7),
            self.cell(8),
            self.cell(9),
            self.cell(10),
            self.cell(11),
            self.cell(12),
            self.cell(13),
            self.cell(14),
            self.cell(15),
            self.cell(16),
            self.cell(17),
            self.cell(18),
            self.cell(19),
            self.cell(20),
            self.cell(21),
            self.cell(22),
            self.cell(23),
            self.cell(24),
            self.cell(25),
            self.cell(26),
            self.cell(27),
            self.cell(28),
            self.cell(29),
            self.cell(30),
            self.cell(31),
            self.cell(32),
            self.cell(33),
            self.cell(34),
            self.cell(35),
            self.cell(36),
            self.cell(37),
            self.cell(38),
            self.cell(39),
            self.cell(40),
            self.cell(41)
        ));
        s.to_string()
    }

    /// Display results.  
    /// 結果の表示。  
    pub fn result(result: GameResult, winner: Option<Piece>) -> Option<String> {
        match result {
            // ぜったい None が返ってこない仕様のときは .unwrap() でヌル・チェックを飛ばせだぜ☆（＾～＾）
            GameResult::Win => Some(format!("win {}", winner.unwrap()).to_string()),
            GameResult::Draw => Some(format!("draw").to_string()),
            GameResult::Lose => None,
        }
    }
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
impl Search {
    pub fn new(start_pieces_num: usize) -> Self {
        Search {
            start_pieces_num: start_pieces_num,
            nodes: 0,
            stopwatch: Instant::now(),
            evaluation: Evaluation::default(),
        }
    }

    /// Information during a forward/backward search.
    /// 前向き/後ろ向き 探索中の情報。
    pub fn info_str(
        nps: u64,
        nodes: u32,
        pv: &str,
        search_direction: SearchDirection,
        file: char,
        leaf: bool,
        pieces_num: Option<usize>,
        search_info: &SearchInfo,
    ) -> String {
        format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}{}{}{}{}{}, \"pv\":[{}] }}",
            nps,
            nodes,
            match search_direction {
                SearchDirection::Forward => {
                    format!(", \"push\":\"{}\"", file)
                }
                SearchDirection::Backward => {
                    format!(", \"pop\" :\"{}\"", file)
                }
            },
            if let Some(pieces_num) = pieces_num {
                format!(", \"pieces\":{}", pieces_num)
            } else {
                "            ".to_string()
            },
            if leaf {
                ", \"leaf\": true"
            } else {
                "              "
            },
            if let Some(result) = search_info.result {
                format!(", \"result\":{:6}", format!("\"{}\"", result.to_string()))
            } else {
                "                 ".to_string()
            },
            if let Some(comment) = &search_info.comment {
                format!(", \"{}\":\"{}\"", search_info.turn, comment).to_string()
            } else {
                format!(", \"{}\":\"\"", search_info.turn).to_string()
            },
            pv,
        )
        .to_string()
    }
}

/// Win evaluation and draw evaluation.  
/// 勝ち評価値と、引き分け評価値。  
#[derive(Clone, Copy, Debug)]
pub enum EvaluationWay {
    Win,
    Draw,
}

/// It is for displaying the thinking process.  
/// 思考過程の表示用です。  
pub struct SearchInfo {
    /// Win evaluation or Draw evaluation.  
    /// 勝ち評価または、引き分け評価。  
    pub way: EvaluationWay,

    /// Weight of move probability.  
    /// 指し手確率の重み。  
    ///
    /// [a, b, c, d, e, f, g]  
    pub ways_weight: [u8; FILE_LEN],

    /// Chosen file.  
    /// 選んだ列。  
    pub chosen_file: Option<char>,

    /// Result.  
    /// 結果。  
    pub result: Option<GameResult>,

    /// Turn.  
    /// 手番。  
    pub turn: Piece,

    /// Comment.  
    /// コメント。  
    pub comment: Option<String>,
}
impl SearchInfo {
    pub fn new(way: &EvaluationWay, ways_weight: &[u8; FILE_LEN]) -> Self {
        SearchInfo {
            way: *way,
            ways_weight: *ways_weight,
            chosen_file: None,
            result: None,
            turn: Piece::Nought,
            comment: None,
        }
    }
    pub fn get_total_weight(&self) -> u16 {
        let mut sum: u16 = 0;
        for i in 0..FILE_LEN {
            sum += self.ways_weight[i] as u16;
        }
        sum
    }
    pub fn info_choose_str(&self) -> String {
        format!(
            "info json {{ \"way\":{:?}, \"choose\":\"{}\", \"total\":{}, \"a\":{}, \"b\":{}, \"c\":{}, \"d\":{}, \"e\":{}, \"f\":{}, \"g\":{} }}",
            self.way,
            if let Some(file) = self.chosen_file {
                file
            }else{
                ' '
            },
            self.get_total_weight(),
            self.ways_weight[0],
            self.ways_weight[1],
            self.ways_weight[2],
            self.ways_weight[3],
            self.ways_weight[4],
            self.ways_weight[5],
            self.ways_weight[6],
        )
        .to_string()
    }
}
