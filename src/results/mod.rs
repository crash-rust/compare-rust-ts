pub fn results() {
    // !除了需要定义response结构体，基本跟ts一样，不过这个处理往往是给前端，传递json序列化的话可以使用serde rust第三方库
    let response = login();
    if response.code == 401 {
        println!("response message is {}", response.message);
    }

    // !rust当中对标ts当中的try。。catch的机制，模式匹配的方式看起来会更优雅
    match read_file(true) {
        Ok(val) => println!("file path is {val}"),
        Err(err) => println!("err is {err}"),
    }

    // let boundary_num = read_arr_boundary(vec![1, 2, 3]);

    // #[allow(unused)]
    // let num_res = boundary_num + 1;
    // !rust中编译检查会直接报错，压根不会执行底下这个操作，直接产生一个panic终端程序
    // println!("num_res is {num_res}");
}

#[derive(Debug)]
struct HttpResponse {
    code: u32,
    message: String,
}
fn login() -> HttpResponse {
    HttpResponse {
        code: 401,
        message: "no login".to_string(),
    }
}

// !Result是rust当中的官方枚举类型之一，利用这个类型我们常常用来处理那些可能发生也可能成功的结果，比如读取指定路径的文件
fn read_file(path_exist: bool) -> Result<String, String> {
    if path_exist {
        Ok("~/.cargo/config".to_string())
    } else {
        Err("can not read property of undeinfed: read path".to_string())
    }
}

#[allow(unused)]
fn read_arr_boundary(nums: Vec<u32>) -> u32 {
    nums[nums.len()]
}
