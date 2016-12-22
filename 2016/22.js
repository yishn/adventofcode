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
