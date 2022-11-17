// !rust当中 trait可以当做ts的interface来使用，实现了interface就具有了相关行为功能
// !rust当中有很多官方trait，只要实现这些trait就可以使用自定义这些trait中的行为参与到流程中执行操作：更像是钩子hook的感觉
trait Fly {
    fn fly(&self);

    // !rust当中，可以对trait中定义的行为添加默认的行为描述，也就是默认方法
    // !ts当中的interface就不具备这种功能，这个功能十分强大，设想一下，继承和组合的本质作用都是啥？终究到底我想应该是代码的复用吧
    // !利用rust当中trait的默认方法，是不是也可以做到继承中的代码复用~rust中没有继承这个概念，rust是将组合式发挥到极致的语言
    fn eat(&self) {
        println!("bird eat");
    }
}

struct Bird;

impl Fly for Bird {
    fn fly(&self) {
        println!("bird fly");
    }
}

pub fn traits() {
    let bird = Bird;
    bird.fly();
    bird.eat();
}
