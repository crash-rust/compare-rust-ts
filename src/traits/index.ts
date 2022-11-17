// !ts当中可以对一组特有行为进行抽象，描述某种特征具有的相关操作，这就是接口
// !ts相对interface的能力比较弱，仅仅只是约束的作用大多时候
// interface Fly {
//   fly(): void;
// }

// class Bird implements Fly {
//   fly() {
//     console.log('bird fly');
//   }
// }

// !不用interface，ts当中通过抽象类继承也是能够做到类似rust代码的复用的
// !但是我们只能继承一个类，还是比较鸡肋的
abstract class AbstractBird {
  abstract fly(): void;
  eat() {
    console.log('bird eat');
  }
}

class Bird extends AbstractBird {
  fly() {
    console.log('bird fly');
  }
}

export function traits() {
  let bird = new Bird();
  bird.fly();
  bird.eat();
}
