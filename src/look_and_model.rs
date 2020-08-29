//! Display and data structure.  
//! 表示と、データ構造です。  
use crate::computer_player::Evaluation;
use crate::{
    EvaluationWay, GameResult, Piece, Position, Search, SearchDirection, SearchInfo, BOARD_LEN,
    FILE_LEN, SQUARES_NUM,
};
use std::fmt;
use std::time::Instant;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::look_and_model::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
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

impl Search {
    pub fn new(start_pieces_num: usize) -> Self {
        Search {
            start_pieces_num: start_pieces_num,
            nodes: 0,
            stopwatch: Instant::now(),
            evaluation: Evaluation::default(),
        }
    }
}

impl SearchInfo {
    pub fn new(way: &EvaluationWay, ways_weight: &[u8; FILE_LEN]) -> Self {
        SearchInfo {
            way: *way,
            ways_weight: *ways_weight,
            nps: 0,
            nodes: 0,
            pv: "".to_string(),
            search_direction: SearchDirection::Forward,
            chosen_file: None,
            leaf: false,
            pieces_num: None,
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

    /// Information during a forward/backward search.  
    /// 前向き/後ろ向き 探索中の情報。  
    pub fn to_string(&self) -> String {
        format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}{}{}{}{}{}, \"way\":{:?}, \"choose\":\"{}\", \"total\":{}, \"a\":{}, \"b\":{}, \"c\":{}, \"d\":{}, \"e\":{}, \"f\":{}, \"g\":{}, \"pv\":[{}] }}",
            self.nps,
            self.nodes,
            if let Some(file) = self.chosen_file {
                match self.search_direction {
                    SearchDirection::Forward => format!(", \"push\":\"{}\"", file),
                    SearchDirection::Backward => format!(", \"pop\" :\"{}\"", file),
                }
            } else {
                "            ".to_string()
            },
            if let Some(pieces_num) = self.pieces_num {
                format!(", \"pieces\":{}", pieces_num)
            } else {
                "            ".to_string()
            },
            if self.leaf {
                ", \"leaf\": true"
            } else {
                "              "
            },
            if let Some(result) = self.result {
                format!(", \"result\":{:6}", format!("\"{}\"", result.to_string()))
            } else {
                "                 ".to_string()
            },
            if let Some(comment) = &self.comment {
                format!(", \"{}\":\"{}\"", self.turn, comment).to_string()
            } else {
                format!(", \"{}\":\"\"", self.turn).to_string()
            },
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
            self.pv,
        )
        .to_string()
    }
}
