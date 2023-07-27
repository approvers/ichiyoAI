use std::fs::File;
use std::io::Write;

pub fn create_temp_file(content: String) -> File {
    let mut temp_file = File::create("temp.txt").expect("一時ファイルの作成に失敗しました。");
    temp_file
        .write_all(content.as_ref())
        .expect("一時ファイルへの書き込みに失敗しました。");
    return temp_file;
}
