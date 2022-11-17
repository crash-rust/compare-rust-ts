export function rcs() {
  readName();
}

function readName() {
  let name = 'jaylen';
  for (let i = 0; i < 10; i++) {
    let cur_name = name;
    console.log(`cur_name is ${cur_name}`);
  }
}
