mod map_count;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    // 初始化工作
    init();
    let args: Vec<String> = env::args().collect();

    let folder_path = match args.get(1) {
        Some(path) => path,
        None => {
            tracing::info!("Please provide a folder path as a command line argument...");
            return Ok(())
        }
    };
    tracing::info!("您当前查看的目录是：{}", folder_path);
    let mut word_count = 0;
    let mut char_count = 0;
    let mut line_count = 0;
    let word_regex = Regex::new(r"\w+").unwrap();

    for entry in WalkDir::new(folder_path) {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if file_path.is_file() {
            let file = File::open(&file_path)?;
            // 防止非utf8转string编码问题
            let mut reader = BufReader::with_capacity(1024 * 1024, file);
            let mut buf = vec![];
            while let Ok(bytes_read) = reader.read_until(b'\n', &mut buf) {
                if bytes_read == 0 {
                    break
                }
                let line = std::str::from_utf8(&buf).unwrap_or("");
                let count = word_regex.find_iter(&line).count();
                let count2 = line.chars().count();
                word_count += count;
                char_count += count2;
                line_count += 1;
                buf.clear();
            }
        }
    }
    tracing::info!("【代码行数为: {}， 单词数量为: {}， 字符数量为: {}】", line_count, word_count, char_count);
    Ok(())
}

fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();
}
