use simplelog::*;

pub fn init() {
    // logディレクトリがなければ作成する。
    if std::fs::metadata("log").is_err() {
        if std::fs::create_dir("log").is_err() {
            panic!("Failed to create log directory");
        }
    }
    // ログファイル名の解決
    let log_file_name = format!(
        "log/log-{}.log",
        chrono::Local::now().format("%Y-%m-%d-%H%M%S")
    );
    // ログファイルの作成
    let log_file = std::fs::File::create(log_file_name);
    // 起動時エラーなのでpanicで落とす
    if log_file.is_err() {
        panic!("Failed to create log file: {}", log_file.err().unwrap())
    }

    let config = simplelog::ConfigBuilder::new()
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day]-[hour]:[minute]:[second]"
        ))
        .set_level_padding(LevelPadding::Off)
        .set_thread_mode(ThreadLogMode::Both)
        .build();

    // ロガーの初期化
    let (terminal_level, file_level) = if cfg!(debug_assertions) {
        (LevelFilter::Debug, LevelFilter::Debug)
    } else {
        (LevelFilter::Warn, LevelFilter::Info)
    };

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            terminal_level,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        // ファイルsimplelog.logにはInfo以上を表示する。
        simplelog::WriteLogger::new(file_level, config.clone(), log_file.unwrap()),
    ])
    .unwrap();

    log::info!("Logging initialized");
}
