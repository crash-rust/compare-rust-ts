export function strings() {
  // !大家还记得这个构造函数嘛？实际上这个只是一个临时对象，这一行之后就没用了这个对象
  let hello = new String('hello');

  console.log(`string = ${hello}`);

  let move_string = hello;

  console.log(`move_string is ${move_string}`);
  // !ts当中当然可以，本质上ts中的string就是放在栈上的，做的只是值拷贝而已
  console.log(`origin_string is ${hello}`);
}
