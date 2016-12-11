const fs = require('fs')

let input = fs.readFileSync('./input03.txt', 'utf8').trim()
let dataRows = input.split('\n').map(x => x.trim().split(/\s+/).map(y => +y))

let sign = (i, j) => i == j ? -1 : 1
let possible = t => t.every((_, i) => t.reduce((sum, x, j) => sum + sign(i, j) * x, 0) > 0)

console.log('Part 1:\t', dataRows.filter(possible).length)

let segment = (array, n) => [...Array(Math.ceil(array.length / n))].map((_, i) => array.slice(i * n, (i + 1) * n))
let flatten = array => array.reduce ? array.reduce((acc, x) => acc.concat(flatten(x)), []) : [array]
let dataCols = segment(flatten(dataRows[0].map((_, i) => dataRows.map(x => x[i]))), 3)

console.log('Part 2:\t', dataCols.filter(possible).length)
