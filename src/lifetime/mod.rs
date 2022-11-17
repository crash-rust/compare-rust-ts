pub fn lifetime() {
    let a_string = "A_String".to_string();
    {
        let b_string = "B_String".to_string();
        let res = longest_str(&a_string, &b_string);
        println!("res is {res}");
    }
}
// ! 理解rust生命周期的概念
fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// fn longest_str<'a, 'b>(a: &'a str, b: &'b str) -> &'a str
// where
//     'b: 'a,
//     // 'a:'b
// {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }
