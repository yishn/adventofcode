const fs = require('fs')

let input = fs.readFileSync('./input15.txt', 'utf8').trim()
let data = input.split('\n')
    .map(line => line.match(/#\d+\D+(\d+)\D+0\D+(\d+)/))
    .map(([, m, start]) => [+m, +start])

let mod = (a, m) => (a % m + m) % m

function solveChineseRemainder(image, mods) {
    if (image.length != mods.length) return null

    let product = mods.reduce((p, x) => p * x)

    for (let i = 0; i < product; i++) {
        if (mods.every((m, j) => mod(i - image[j], m) == 0)) {
            return i
        }
    }

    return null
}

let image = data.map(([, start], i) => -start - i - 1)
let mods = data.map(([m, ]) => m)

console.log('Part 1:\t' + solveChineseRemainder(image, mods))

data.push([11, 0])
image = data.map(([, start], i) => -start - i - 1)
mods = data.map(([m, ]) => m)

console.log('Part 2:\t' + solveChineseRemainder(image, mods))
