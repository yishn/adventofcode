const fs = require('fs')

let input = fs.readFileSync('./input23.txt', 'utf8').trim()
let instructions = () => input.split('\n').map(x => x.trim().split(' '))
let equals = (a, b) => a.length == b.length && a.every((x, i) => x == b[i])
let get = (x, state) => isNaN(x) ? state[x] : +x

function optimize(state, instructions, i) {
    let [command, ...args] = instructions[i]

    if (i < instructions.length - 2) {
        let commands = instructions.slice(i, i + 2).map(([x, ]) => x)
        let condition = null
        let condition2 = null
        let storage = null

        if (equals(commands, ['dec', 'inc'])) {
            condition = args[0]
            storage = instructions[i + 1][1]
        } else if (equals(commands, ['inc', 'dec'])) {
            condition = instructions[i + 1][1]
            storage = args[0]
        }

        if (i < instructions.length - 4 && instructions[i + 3][0] == 'dec') {
            condition2 = instructions[i + 3][1]
        }

        if (condition && equals(instructions[i + 2], ['jnz', condition, '-2'])) {
            if (condition2 && equals(instructions[i + 4], ['jnz', condition2, '-5'])) {
                state[storage] += state[condition] * state[condition2]
                state[condition] = 0
                state[condition2] = 0

                return [true, i + 5]
            } else {
                state[storage] += state[condition]
                state[condition] = 0

                return [true, i + 3]
            }
        }
    }

    return [false, i]
}

function run(state, instructions) {
    let i = 0

    while (i < instructions.length) {
        let [command, ...args] = instructions[i]
        let [success, j] = optimize(state, instructions, i)

        if (success) {
            i = j
            continue
        }

        if (command == 'cpy' && isNaN(args[1])) {
            state[args[1]] = get(args[0], state)
        } else if (command == 'inc') {
            state[args[0]]++
        } else if (command == 'dec') {
            state[args[0]]--
        } else if (command == 'jnz' && args[0] != '0' && state[args[0]] != 0) {
            i += get(args[1], state) - 1
        } else if (command == 'tgl') {
            let j = get(args[0], state) + i

            if (0 <= j && j < instructions.length) {
                let changeMap = {tgl: 'inc', dec: 'inc', inc: 'dec', jnz: 'cpy', cpy: 'jnz'}
                instructions[j][0] = changeMap[instructions[j][0]]
            }
        }

        i++
    }

    return state
}

console.log('Part 1:\t' + run({'a': 7, 'b': 0, 'c': 0, 'd': 0}, instructions())['a'])
console.log('Part 2:\t' + run({'a': 12, 'b': 0, 'c': 0, 'd': 0}, instructions())['a'])
