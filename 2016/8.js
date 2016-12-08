const fs = require('fs')

let input = fs.readFileSync('./input8.txt', 'utf8').trim()
let instructions = input.split('\n')
    .map(x => x.match(/(rect|rotate (row|column)) ([xy]=)?(\d+)( by |x)(\d+)/))
    .map(([, type, , , x, , y]) => [type, +x, +y])

let printScreen = screen => console.log(screen.map(row => row.map(v => v ? '#' : '.').join('')).join('\n'))
let newArray = length => Array.apply(null, {length})

let shiftArray = (a, s) => [...a.slice(-s % a.length), ...a.slice(0, -s % a.length)]
let countOn = screen => !isNaN(screen) ? +screen : screen.reduce((sum, x) => sum + countOn(x), 0)

let rotateRow = (screen, y, s) => {
    screen[y] = shiftArray(screen[y], s)
}

let rotateColumn = (screen, x, s) => {
    let column = screen.map(row => row[x])
    let shifted = shiftArray(column, s)
    screen.forEach((row, i) => row[x] = shifted[i])
}

let drawRect = (screen, x, y) => {
    for (let i = 0; i < x; i++) {
        for (let j = 0; j < y; j++) {
            screen[j][i] = true
        }
    }
}

function getScreen(instructions, width, height) {
    let screen = [...Array(height)].map(_ => Array(width).fill(false))

    instructions.forEach(([type, x, y]) => {
        let dict = {
            'rect': drawRect,
            'rotate column': rotateColumn,
            'rotate row': rotateRow
        }

        dict[type](screen, x, y)
    })

    return screen
}

let screen = getScreen(instructions, 50, 6)
console.log('Part 1:\t' + countOn(screen))
console.log('Part 2:')
printScreen(screen)
