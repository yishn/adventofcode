const fs = require('fs')

let input = fs.readFileSync('./input3.txt', 'utf8').trim()

let newArray = n => Array.apply(null, {length: n})
let segment = (array, n) => newArray(array.length / n).map((_, i) => array.slice(i * n, i * n + 3))

let dataRows = input.split('\n').map(x => x.trim().split(/\s+/).map(y => +y))
let rows = dataRows.length
let dataCols = segment(newArray(rows * 3).map((_, i) => dataRows[i % rows][Math.floor(i / rows)]), 3)

let sign = (i, j) => i == j ? -1 : 1
let possible = t => t.every((_, i) => t.reduce((sum, x, j) => sum + sign(i, j) * x, 0) > 0)

console.log(dataRows.filter(possible).length)
console.log(dataCols.filter(possible).length)
