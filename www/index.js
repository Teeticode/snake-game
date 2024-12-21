import init, { greet } from "snakegame";

init()
  .then((_) => {
    greet("Dennis");
    console.log("Greetings from WebAssembly!");
  })
  .catch((err) => {
    console.error("Failed to initialize or load WebAssembly module:", err);
    throw err;
  });
