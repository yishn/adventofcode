const fs = require('fs')

let input = fs.readFileSync('./input12.txt', 'utf8').trim()
let instructions = input.split('\n').map(x => x.trim().split(' '))

function run(state, instructions) {
    let i = 0

    while (i < instructions.length) {
        let [command, ...args] = instructions[i]

        if (command == 'cpy') {
            state[args[1]] = isNaN(args[0]) ? state[args[0]] : +args[0]
        } else if (command == 'inc') {
            state[args[0]]++
        } else if (command == 'dec') {
            state[args[0]]--
        } else if (command == 'jnz' && args[0] != 0 && state[args[0]] != 0) {
            i += +args[1] - 1
        }

        i++
    }

    return state
}

let initState = {'a': 0, 'b': 0, 'c': 0, 'd': 0}

console.log('Part 1:\t' + run(initState, instructions)['a'])

initState = {'a': 0, 'b': 0, 'c': 1, 'd': 0}

console.log('Part 2:\t' + run(initState, instructions)['a'])
