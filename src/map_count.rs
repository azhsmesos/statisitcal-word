
use std::{fs};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

/// read_word_frequency 统计单词出现频率
fn _read_word_frequency(folder_path: &str) {
    let meta_file_data = fs::metadata(folder_path).unwrap();
    if meta_file_data.is_dir() {

    }
    let file_path_list = fs::read_dir(folder_path).unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    let mut word_count_map = HashMap::new();
    for file_path in file_path_list {
        let reader = BufReader::new(fs::File::open(file_path).unwrap());
        for line in reader.lines().filter_map(|line| line.ok()) {
            for word in line.split_whitespace() {
                let count = word_count_map.entry(word.to_lowercase()).or_insert(0);
                *count += 1;
            }
        }
    }
    tracing::info!("该文件目录下的单词个数为：{:#?}", word_count_map);
    let count = word_count_map.values().count();
    tracing::info!("单词个数（去重）：{}", count);
}