const fs = require('fs')

let input = fs.readFileSync('./input10.txt', 'utf8').trim()
let readyBot = state => id => !isNaN(id[0]) && state[id].length == 2

function parseInput(input) {
    let initialState = {}
    let instructions = {}

    for (let line of input.split('\n')) {
        if (line.indexOf('bot') == 0) {
            let [, id, t1, low, t2, high] = line.match(/(\d+)\D+\b(bot|output) (\d+)\D+\b(bot|output) (\d+)/)
            if (t1 == 'output') low = 'o' + low
            if (t2 == 'output') high = 'o' + high
            instructions[id] = [low, high]
        } else {
            let [, value, id] = line.match(/(\d+)\D+(\d+)/)
            initialState[id] = [...(initialState[id] || []), +value]
        }
    }

    return [initialState, instructions]
}

function* step(state, instructions) {
    let readyIds = Object.keys(state).filter(readyBot(state))
    if (readyIds.length == 0) return

    for (let id of readyIds) {
        let [low, high] = instructions[id]
        let [x, y] = state[id]

        state[id] = []
        state[low] = [...(state[low] || []), Math.min(x, y)]
        state[high] = [...(state[high] || []), Math.max(x, y)]
    }

    yield state
    yield* step(state, instructions)
}

let [state, instructions] = parseInput(input)
let compare17with61 = state => id => Math.max(...state[id]) == 61 && Math.min(...state[id]) == 17
let rightBot = state => id => readyBot(state)(id) && compare17with61(state)(id)

for (state of step(state, instructions)) {
    let id = Object.keys(state).find(rightBot(state))
    if (id) console.log('Part 1:\t' + id)
}

let product = ['o0', 'o1', 'o2'].map(x => state[x]).reduce((p, x) => p * x)

console.log('Part 2:\t' + product)
