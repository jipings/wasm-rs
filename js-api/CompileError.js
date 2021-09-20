try {
    throw new WebAssembly.CompileError('Hello', 'someFile', 10);
} catch (e) {
    // console.log(e instanceof CompileError); // true
    console.log(e.message);                 // "Hello"
    console.log(e.name);                    // "CompileError"
    console.log(e.fileName);                // "someFile"
    console.log(e.lineNumber);              // 10
    console.log(e.columnNumber);            // 0
    console.log(e.stack);                   // 返回代码运行的位置
}

console.log("======== RuntimeError ===========")

try {
    throw new WebAssembly.RuntimeError('Hello', 'someFile', 10);
  } catch (e) {
    // console.log(e instanceof RuntimeError); // true
    console.log(e.message);                 // "Hello"
    console.log(e.name);                    // "RuntimeError"
    console.log(e.fileName);                // "someFile"
    console.log(e.lineNumber);              // 10
    console.log(e.columnNumber);            // 0
    console.log(e.stack);                   // 返回代码运行的位置
  }  