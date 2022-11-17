export function boxs() {
  let box_1 = new Number(1);

  console.log(`box1 = ${box_1}`);

  let ref_box1 = box_1;

  console.log(`ref_box1 = ${ref_box1}`);

  // !遗憾的是ts当中没有方式可以打印出变量的实际地址
}
