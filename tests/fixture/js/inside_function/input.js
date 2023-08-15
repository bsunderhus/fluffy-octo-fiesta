function fn(name) {
  console.log(/** @swc de-indent */ `
    Hello world!,
    this is ${name}.
    how are you?
  `);
}
