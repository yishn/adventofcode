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

function* listViablePairs(data) {
    for (let [x1, y1] of listVertices()) {
        for (let [x2, y2] of listVertices()) {
            let a = data[y1][x1], b = data[y2][x2]
            if (isViablePair(a, b)) yield [a, b]
        }
    }
}

console.log('Part 1:\t' + [...listViablePairs(data)].length)

let isValid = ([x, y]) => 0 <= x && x < width && 0 <= y && y < height
let getNeighbors = ([x, y]) => [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]].filter(isValid)
let copy = data => data.map(x => x.map(y => Object.assign({}, y)))

function repr(data, xpos) {
    return data
        .map((row, y) => row.map(({used}, x) => used == 0 ? '_' : equals([x, y], xpos) ? 'x' : '.').join(' '))
        .join('\n') + '\n'
}

function* listMoves(data, [x, y]) {
    let b = data[y][x]

    for (let [nx, ny] of getNeighbors([x, y])) {
        let a = data[ny][nx]
        if (isViablePair(a, b)) yield [a, b]
    }
}

function move(data, p, q) {
    let newData = copy(data)
    let [newA, newB] = [p, q].map(x => newData[x[1]][x[0]])

    newB.available -= newA.used
    newB.used += newA.used
    newA.available += newA.used
    newA.used = 0

    return newData
}

function bfs(start, xpos, isEnd) {
    let queue = [[...start, 0]]
    let parents = {[repr(start[0], xpos)]: true}

    while (queue.length > 0) {
        let [data, pos, distance] = queue.shift()

        if (isEnd(data)) return [data, pos, distance]

        for (let [a, b] of listMoves(data, pos)) {
            if (equals(a.pos, xpos)) continue

            let newData = move(data, a.pos, b.pos)
            let key = repr(newData, xpos)

            if (key in parents) continue

            parents[key] = true
            queue.push([newData, a.pos, distance + 1])
        }
    }

    return null
}

let xpos = [width - 1, 0]
let pos = data.map(x => x.findIndex(y => y.used == 0))
    .reduce((p, x, y) => x >= 0 ? [x, y] : p, [-1, -1])

let [, , distance] = bfs([data, pos], xpos, x => x[0][width - 2].used == 0)

console.log('Part 2:\t' + (distance + 5 * (width - 2) + 1))
