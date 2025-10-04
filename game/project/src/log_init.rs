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

    // ロガーの初期化
    simplelog::CombinedLogger::init(vec![
        // 標準出力にはWarn以上を表示する。
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        // ファイルsimplelog.logにはInfo以上を表示する。
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::ConfigBuilder::new()
                .set_time_format_custom(format_description!("[year]-[month]-[day]-[hour]:[minute]:[second]"))
                .set_level_padding(LevelPadding::Off)
                .set_thread_mode(ThreadLogMode::Both)
                .build(),
            log_file.unwrap(),
        ),
    ])
    .unwrap();

    log::info!("Logging initialized");
}
