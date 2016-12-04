const fs = require('fs')

let input = fs.readFileSync('./input2.txt', 'utf8').trim()
let instructions = input.split('\n').map(x => x.trim().toUpperCase().split(''))

const directions = {'U': [0, -1], 'L': [-1, 0], 'D': [0, 1], 'R': [1, 0]}

let valid = (kp, [x, y]) => y in kp && kp[y][x] != null
let add = (a, b) => a.map((x, i) => x + b[i])
let goDirection = (kp, p, uldr) => valid(kp, add(p, directions[uldr])) ? add(p, directions[uldr]) : p
let followChunk = (kp, s, chunk) => chunk.reduce((...pd) => goDirection(kp, ...pd), s)
let followInstr = (kp, s, instr) => instr.reduce((acc, chunk) => [...acc, followChunk(kp, acc.slice(-1)[0], chunk)], [s])
let getCode = (kp, positions) => positions.map(([x, y]) => kp[y][x]).join('')
let solve = (kp, s, instr) => getCode(kp, followInstr(kp, s, instr).slice(1))

let normalKeypad = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9']
]

console.log('Part 1:\t', solve(normalKeypad, [1, 1], instructions))

let weirdKeypad = [
    [null, null, '1', null, null],
    [null,  '2', '3',  '4', null],
    [ '5',  '6', '7',  '8',  '9'],
    [null,  'A', 'B',  'C', null],
    [null, null, 'D', null, null]
]

console.log('Part 2:\t', solve(weirdKeypad, [0, 2], instructions))
