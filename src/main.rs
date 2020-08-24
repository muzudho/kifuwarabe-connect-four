mod command_line_seek;
mod engine;
mod log;
mod look_and_model;
mod position;
mod test;
mod uxi_protocol;

use crate::engine::*;
use crate::log::LogExt;
use crate::test::test;
use casual_logger::{Level, Log, Table};

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
