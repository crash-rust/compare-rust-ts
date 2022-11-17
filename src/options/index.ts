export function options() {
  let money = undefined;

  console.log(`money is = ${money}`);

  // !注意看，这里money的类型失去了保证，既可能是undefined又可以是number，处于同一层次上了
  money = 1;
  plusMoney(money);
}

function plusMoney(money: number) {
  console.log(`now money is = ${money}`);
}
