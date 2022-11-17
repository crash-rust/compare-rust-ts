pub fn options() {
    // !rust从语言层面直接解决了null和undefined的问题
    // !当我们尝试获取一个可能存在或者可能不存在的值的时候，你只能使用Option<T>枚举包裹
    let mut money: Option<u32> = Option::None;

    println!("money is = {money:?}");

    money = Some(1);
    plus_money(money);
}

fn plus_money(money: Option<u32>) {
    // !两个完全不同的类型没法执行相加操作
    // println!("now money is = {:?}", money + 1);
    let money = match money {
        Some(num) => num + 1,
        _ => 0,
    };

    println!("now money is = {money}");
}
