pub fn strings() {
    // !rust当中对字符串类型做出了严格区分，这里看到的String实际上是一个智能指针
    // !它的内存分布是将堆数据地址存放在栈上，实际数据存放在堆中
    // !因此，它具有数据的所有权！
    let hello = String::from("hello");

    println!("string = {hello}");

    let move_hello = hello;

    println!("move_string is = {move_hello}");
    // !如果你接着尝试打印hello变量，这里就报错了，因为你尝试继续使用一个已经将数据所有权移动了的变量，hello失去了对数据的控制权
    // println!("origin_string is = {hello}");

    // !为了解决这个问题，我们可以引入一个概念：引用
    // !让我们重新看这个问题
    let hello = String::from("hello");
    let ref_hello = &hello;

    // !这里ref_hello只是借用了hello的所有权，并没有直接转移了hello的所有权
    // !本质上你可以看成ref_hello只是hello数据的内存别名，hello本身存着堆数据地址，而ref_hello存着hello在栈上的地址
    println!("ref_hello is {ref_hello}");
    println!("origin_hello is {hello}");

    // !我们还可以使用另一种方式，也不会获得字符串所有权，引入新概念：字符串切片&str
    let ref1_hello = hello.as_str();
    println!("ref1_hello is {ref1_hello}");
    println!("origin_hello is {hello}");
}
