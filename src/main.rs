mod log;
mod look_and_model;
mod position;
mod test;

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
}

fn file_to_num(file: char) -> u8 {
    match file {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        _ => panic!(Log::print_fatal_t(
            "(Err.32) Invalid file.",
            Table::default().char("file", file)
        )),
    }
}
