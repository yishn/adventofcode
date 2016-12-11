const fs = require('fs')

let input = fs.readFileSync('./input01.txt', 'utf8').trim()
let instructions = input.split(', ').map(x => [x[0], +x.slice(1)])

let turn = ([x, y], lr) => lr.toUpperCase() == 'L' ? [-y, x] : [y, -x]
let distance = ([x, y]) => Math.abs(x) + Math.abs(y)
let range = n => [...Array(n)].map((_, i) => i + 1)

let [end, _, __, collisions] = instructions.reduce(([position, direction, ...vc], [lr, steps]) => {
    let newDirection = turn(direction, lr)
    let newPositions = range(steps).map(j => newDirection.map((v, i) => position[i] + j * v))

    let newVC = newPositions.reduce(([visited, collisions], v) => {
        if (v in visited) collisions.push(v)
        visited[v] = true

        return [visited, collisions]
    }, vc)

    return [newPositions.slice(-1)[0], newDirection, ...newVC]
}, [[0, 0], [0, 1], {}, []])

let bunny = collisions[0]

console.log('End point:\t', end, distance(end))
console.log('Bunny HQ:\t', bunny, distance(bunny))
