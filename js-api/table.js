
const table = new WebAssembly.Table({ element: "anyfunc", initial: 1, maximum: 10 });
table.grow(1);
console.log(table.length);

WebAssembly.instantiateStreaming(fetch('table.wasm'))
    .then(function(obj) {
        const tbl = obj.instance.exports.tbl;
        console.log(tbl.get(0)());
        console.log(tbl.get(1)());
    })