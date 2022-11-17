class Person {
  name: string;
  language: string[];

  constructor(name: string, language: string[]) {
    this.name = name;
    this.language = language;
  }

  master() {
    let masterlang = '';
    for (let l of this.language) {
      masterlang += l;
    }

    console.log(`master language  = ${masterlang}`);
  }
}

export function structs() {
  let jaylen = new Person('jaylen', ['Rust', ' Typescript']);
  jaylen.master();
}
