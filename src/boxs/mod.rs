pub fn boxs() {
    let box_1 = Box::new(1);

    println!("box1 = {box_1}");

    let ref_box1 = &box_1;

    // !取box_1的引用，此时ref_box只是box1的内存别名的感觉，二者找到的是同一个堆数据
    println!("ref_box1 address = {:p}", ref_box1);
    println!("box1 address = {:p}", box_1);

    // !尝试将box_1给到box_2
    let box_2 = box_1;
    println!("box2 is {box_2}");
    // !报错了，box_1失去了数据的所有权
    // println!("box1 is {box_1}");
}
