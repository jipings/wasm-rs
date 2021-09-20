var importObj = {
    js: {
        import1: () => console.log("hello,"),
        import2: () => console.log("world!")
    }
};
  
  onmessage = function(e) {
    console.log('module received from main thread');
    var mod = e.data;
  
    WebAssembly.instantiate(mod, importObj).then(function(instance) {
      instance.exports.f();
    });
  
    var exports = WebAssembly.Module.exports(mod);
    console.log(exports[0]);
  };
