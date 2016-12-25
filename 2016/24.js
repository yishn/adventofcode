const fs = require('fs')

let input = fs.readFileSync('./input24.txt#', 'utf8').trim()
let data = input.split('\n').map(x => [...x.trim()])

let targets = data
    .map((row, y) => row.map((z, x) => !isNaN(z) ? [[x, y], +z] : null))
    .reduce((arr, row) => [...arr, ...row.filter(x => x)], [])
    .sort(([, a], [, b]) => a - b)
    .map(([x, ]) => x)

let equals = (a, b) => a.length == b.length && a.every((x, i) => x == b[i])
let valid = ([x, y]) => 0 <= x && x < data[0].length && 0 <= y && y < data.length && data[y][x] != '#'
let neighbors = ([x, y]) => [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]].filter(valid)
let sum = arr => arr.reduce((sum, x) => sum + x)

function getDistances(start, targets) {
    let result = targets.map(_ => Infinity)
    let queue = [start]
    let distance = {[start]: 0}

    while (queue.length > 0) {
        let current = queue.shift()
        let index = targets.findIndex(v => equals(current, v))

        if (index >= 0) {
            result[index] = distance[current]
            if (result.every(x => x != Infinity)) break
        }

        for (let neighbor of neighbors(current, data)) {
            if (neighbor in distance) continue

            distance[neighbor] = distance[current] + 1
            queue.push(neighbor)
        }
    }

    return result
}

function* getPermutations(array) {
    if (array.length == 0) yield []

    for (let i = 0; i < array.length; i++) {
        for (let perm of getPermutations(array.filter((_, j) => i != j))) {
            yield [array[i], ...perm]
        }
    }
}

function travelingSalesmanPath(distances) {
    let distance = Infinity
    let path = null
    let permutations = getPermutations([...Array(distances.length - 1)].map((_, i) => i + 1))

    for (let perm of permutations) {
        let d = sum(perm.map((x, i) => i == 0 ? distances[0][x] : distances[perm[i - 1]][x]))

        if (d < distance) {
            distance = d
            path = [0, ...perm]
        }
    }

    return [distance, path]
}

let distances = targets.map(v => getDistances(v, targets))

process.stdout.write('Part 1:\t')
console.log(travelingSalesmanPath(distances))

function travelingSalesmanTour(distances) {
    let distance = Infinity
    let path = null
    let permutations = getPermutations([...Array(distances.length - 1)].map((_, i) => i + 1))

    for (let perm of permutations) {
        let d = sum(perm.map((x, i) => i == 0 ? distances[0][x] + distances[perm.slice(-1)[0]][0] : distances[perm[i - 1]][x]))

        if (d < distance) {
            distance = d
            path = [0, ...perm, 0]
        }
    }

    return [distance, path]
}

process.stdout.write('Part 2:\t')
console.log(travelingSalesmanTour(distances))
