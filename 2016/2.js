const fs = require('fs')

let input = fs.readFileSync('./input2.txt', 'utf8').trim()
let instructions = input.split('\n').map(x => x.trim().toUpperCase().split(''))

function solve(keypad, instructions, start) {
    let directions = {'U': [0, -1], 'L': [-1, 0], 'D': [0, 1], 'R': [1, 0]}
    let valid = ([x, y]) => y in keypad && keypad[y][x] != null

    if (!valid(start)) return ''

    let [passcode, ] = instructions.reduce(([passcode, position], chunk) => {
        [x, y] = chunk.reduce((position, uldr) => {
            let newPosition = directions[uldr].map((x, i) => position[i] + x)
            return valid(newPosition) ? newPosition : position
        }, position)

        return [passcode + keypad[y][x], [x, y]]
    }, ['', start])

    return passcode
}

let normalKeypad = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9']
]

console.log(solve(normalKeypad, instructions, [1, 1]))

let weirdKeypad = [
    [null, null, '1', null, null],
    [null,  '2', '3',  '4', null],
    [ '5',  '6', '7',  '8',  '9'],
    [null,  'A', 'B',  'C', null],
    [null, null, 'D', null, null]
]

console.log(solve(weirdKeypad, instructions, [0, 2]))
