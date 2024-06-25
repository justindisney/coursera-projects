use cli_utils::colors::{Color, ColorString};

#[test]
fn test_red_coloring() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn test_bold() {
    let mut color_string = ColorString {
        color: Color::Bold,
        string: "Bold".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[1mBold\x1b[0m");
}

#[test]
fn test_whitespace() {
    let mut color_string = ColorString {
        color: Color::Bold,
        string: " ".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[1m \x1b[0m");
}