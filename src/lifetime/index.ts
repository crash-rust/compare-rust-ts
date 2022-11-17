export function lifetime() {
  let a_string = 'A_String';
  {
    let b_string = 'B_String';
    let res = longest_str(a_string, b_string);
    console.log(`res is ${res}`);
  }
}

// !ts当中压根就没有这个烦恼，因为a，b始终是在栈上的，只是值赋值在做比较，压根不涉及引用的概念
function longest_str(a: string, b: string): string {
  if (a.length > b.length) {
    return a;
  } else {
    return b;
  }
}
