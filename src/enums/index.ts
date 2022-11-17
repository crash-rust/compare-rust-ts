enum Color {
  Red = 'red',
  Green = 'green'
}

export function enums() {
  // ! ts当中的枚举本质就是枚举值本身取了别名
  let color_red = Color.Red;
  let color_green = Color.Green;

  console.log(`red & green => ${color_red} ${color_green}`);
}
