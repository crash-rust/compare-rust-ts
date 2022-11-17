#[derive(Debug)]
enum Color {
    Red(String),
    Green(String),
}

pub fn enums() {
    // !rust中的枚举本质上就是一个函数
    let color_red = Color::Red("red".to_string());
    let color_green = Color::Green("green".to_string());

    // !直接这么拿，是拿不到内部存储的值的
    println!("red & green  => {color_red:?} {color_green:?}");

    // !需要获取值，可以配合match表达式进行模式匹配
    match color_red {
        Color::Red(val) => println!("color_red => {val:?}"),
        _ => println!("not found color_red"),
    }

    match color_green {
        Color::Green(val) => println!("color_green => {val:?}"),
        _ => println!("not found color_green"),
    }
}
