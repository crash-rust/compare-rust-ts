export function arrays() {
  let nums = [1, 2, 3];

  console.log(`nums is = ${nums}`);

  // !我们尝试让数组做更新，发现ts当中数组是可以动态更新的
  nums.push(4);
  console.log(`now nums is = ${nums}`);
}
