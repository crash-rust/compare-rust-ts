use std::rc::Rc;

pub fn rcs() {
    // read_name();
    fix_read_name();
}

fn read_name() {
    // let name = String::from("jaylen");

    // for _ in 0..10 {
    //     let cur_name = name;
    //     println!("cur_name is {cur_name}");
    // }
}

fn fix_read_name() {
    // !rust当中字符串是智能指针，它拥有数据的所有权，进入for循环第一次的时候name就转移所有权了
    // !为了解决单线程中数据所有权转移的问题，我们引入另外一个新概念：Rc引用计数，它允许一个数据值拥有多个所有者
    let name = Rc::new(String::from("jaylen"));

    for _ in 0..10 {
        let cur_name = name.clone();

        println!("cur_naem = {cur_name}");
    }
}
