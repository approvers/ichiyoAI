use std::fs::{create_dir, File};
use std::io::Write;

pub fn create_temp_file(content: String) {
    match create_dir("temp") {
        Ok(_) => {
            println!("一時フォルダを作成しました。");

            let mut temp_file =
                File::create("temp/temp.txt").expect("一時ファイルの作成に失敗しました。");
            temp_file
                .write_all(content.as_ref())
                .expect("一時ファイルへの書き込みに失敗しました。");

            println!("一時ファイルを作成しました。")
        }
        Err(_) => panic!("一時ディレクトリを作成できませんでした。"),
    }
}
