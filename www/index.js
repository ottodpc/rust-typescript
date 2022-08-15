import init, { person } from "rust_typescript";

init()
  .then(() => {
    const p = person("Cyprien");
    console.log('person', p)
  })
  .catch((err) => console.log("err", err));
