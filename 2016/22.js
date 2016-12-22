const fs = require('fs')

let input = fs.readFileSync('./input22.txt', 'utf8').trim()

let [width, height] = input.split('\n').slice(-1)[0].match(/x(\d+)-y(\d+)/).slice(1).map(x => +x + 1)
let data = input.split('\n').slice(2)
    .map(x => x.trim().split(/\s+/))
    .map(([a, , c, d, ]) => ({
        pos: a.match(/x(\d+)-y(\d+)/).slice(1).map(x => +x),
        used: +c.slice(0, -1),
        available: +d.slice(0, -1)
    }))
    .reduce(
        (data, x) => (data[x.pos[1]][x.pos[0]] = x, data),
        [...Array(height)].map(_ => [...Array(width)])
    )

let equals = (a, b) => a.every((x, i) => x == b[i])
let isViablePair = (a, b) => !equals(a.pos, b.pos) && 0 < a.used && a.used <= b.available

function* listVertices() {
    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            yield [x, y]
        }
    }
}

function* listViablePairs([data, ]) {
    for (let [x1, y1] of listVertices()) {
        for (let [x2, y2] of listVertices()) {
            let a = data[y1][x1], b = data[y2][x2]
            if (isViablePair(a, b)) yield [a, b]
        }
    }
}

console.log('Part 1:\t' + [...listViablePairs([data, null])].length)

let isValid = ([x, y]) => 0 <= x && x < width && 0 <= y && y < height
let getNeighbors = ([x, y]) => [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]].filter(isValid)
let vertexDistance = (v, w) => v.map((x, i) => Math.abs(x - w[i])).reduce((sum, x) => sum + x)
let vertexCompare = p => (v, w) => vertexDistance(p, v) - vertexDistance(p, w)

let copy = ([data, pos]) => [data.map(x => x.map(y => Object.assign({}, y))), pos]
let isEndState = ([, pos]) => equals(pos, [0, 0])
let repr = state => JSON.stringify(state)

function* listMoves([data, pos]) {
    for (let [x, y] of [...listVertices()].sort(vertexCompare(pos))) {
        let a = data[y][x]

        for (let [nx, ny] of [...getNeighbors([x, y])].sort(vertexCompare(pos)).reverse()) {
            let b = data[ny][nx]

            if (isViablePair(a, b)) {
                yield [a, b]
            }
        }
    }
}

function bfs(start, isEnd) {
    let queue = [[start, 0]]
    let parents = {[repr(start)]: true}

    while (queue.length > 0) {
        let [state, distance] = queue.shift()

        if (isEnd(state)) return [state, distance]

        for (let [a, b] of listMoves(state)) {
            let newState = copy(state)

            if (equals(a.pos, state[1]))
                newState[1] = b.pos

            let [newA, newB] = [a, b].map(x => newState[0][x.pos[1]][x.pos[0]])

            newB.available -= a.used
            newB.used += a.used
            newA.available += a.used
            newA.used = 0

            let key = repr(newState)
            if (key in parents) continue

            parents[key] = true
            queue.push([newState, distance + 1])
        }
    }

    return null
}

console.log('Part 2:\t' + bfs([data, [width - 1, 0]], isEndState))
