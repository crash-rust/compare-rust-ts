pub fn arrays() {
    // !rust也有类型自动推导的功能，这里我们并没有指定类型，但是rust能自动推断出来
    let nums = [1, 2, 3];

    println!("nums is = {nums:?}");

    // !rust中数组的个数和类型在编译的时候就确定下来了，我们没法直接使用数组做push改动原数组
    // !想要做到动态改变元素个数，我们可以使用vector
    let mut nums = vec![1, 2, 3];

    println!("nums is = {nums:?}");

    nums.push(4);

    // !通过使用vector我们就能够做到跟ts当中的数组一样的功能
    // !当你不确定使用array和vector的时候，一律使用vector
    println!("now nums is = {nums:?}");
}
