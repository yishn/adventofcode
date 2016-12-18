const fs = require('fs')

let input = fs.readFileSync('./input18.txt', 'utf8').trim()
let row = input.split('').map(x => x == '^')

let printProgress = p => process.stdout.write(p < 1 ? '\r[' + [
    Array(Math.round(p * 20)).fill('='),
    Array(20 - Math.round(p * 20)).fill(' ')
].map(x => x.join('')).join('>') + `] ${Math.round(p * 100)}% ` : '\r' + Array(30).fill(' ').join('') + '\r')

function generateRow(prevRow) {
    return [...Array(prevRow.length)].map((_, i) => (prevRow[i - 1] || false) != (prevRow[i + 1] || false))
}

function generateMap(startRow, n) {
    let result = [startRow]
    let lastProgress = -1

    while (result.length < n) {
        result.push(generateRow(result.slice(-1)[0]))

        let progress = result.length * 0.9 / (n - 1)

        if (progress - lastProgress >= 0.05) {
            printProgress(progress)
            lastProgress = progress
        }
    }

    return result
}

function countSafe(map) {
    let result = map.reduce((sum, r) => sum + r.filter(x => !x).length, 0)
    printProgress(1)
    return result
}

console.log('Part 1:\t' + countSafe(generateMap(row, 40)))
console.log('Part 2:\t' + countSafe(generateMap(row, 400000)))
