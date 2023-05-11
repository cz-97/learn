use std::fs::{read_to_string, write};
use std::io;

/// > 进入SumatraPDF后打开高级设置, 生成SumatraPDF-settings.txt
/// 
/// > 然后将change-theme.exe放在可执行文件所在目录
fn main() -> io::Result<()> {

    let path = "./SumatraPDF-settings.txt";

    let colors = [
        [
            "MainWindowBackground = #80fff200",
            "TextColor = #000000",
            "BackgroundColor = #ffffff",
        ],
        [
            "MainWindowBackground = #3B3B3B",
            "TextColor = #ffffff",
            "BackgroundColor = #3B3B3B",
        ],
    ];

    let s = read_to_string(path)?;

    let x = match s.find(colors[0][1]) {
        Some(_) => 0,
        None => 1,
    };

    write(
        path,
        s.replace(colors[x][0], colors[1 - x][0])
            .replace(colors[x][1], colors[1 - x][1])
            .replace(colors[x][2], colors[1 - x][2]),
    )?;

    Ok(())
}
