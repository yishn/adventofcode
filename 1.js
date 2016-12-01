let input = 'L5, R1, L5, L1, R5, R1, R1, L4, L1, L3, R2, R4, L4, L1, L1, R2, R4, R3, L1, R4, L4, L5, L4, R4, L5, R1, R5, L2, R1, R3, L2, L4, L4, R1, L192, R5, R1, R4, L5, L4, R5, L1, L1, R48, R5, R5, L2, R4, R4, R1, R3, L1, L4, L5, R1, L4, L2, L5, R5, L2, R74, R4, L1, R188, R5, L4, L2, R5, R2, L4, R4, R3, R3, R2, R1, L3, L2, L5, L5, L2, L1, R1, R5, R4, L3, R5, L1, L3, R4, L1, L3, L2, R1, R3, R2, R5, L3, L1, L1, R5, L4, L5, R5, R2, L5, R2, L1, L5, L3, L5, L5, L1, R1, L4, L3, L1, R2, R5, L1, L3, R4, R5, L4, L1, R5, L1, R5, R5, R5, R2, R1, R2, L5, L5, L5, R4, L5, L4, L4, R5, L2, R1, R5, L1, L5, R4, L3, R4, L2, R3, R3, R3, L2, L2, L2, L1, L4, R3, L4, L2, R2, R5, L1, R2'

let instructions = input.split(', ').map(x => [x[0], +x.slice(1)])
let turn = ([x, y], lr) => lr.toUpperCase() == 'L' ? [-y, x] : [y, -x]
let distance = ([x, y]) => Math.abs(x) + Math.abs(y)
let range = n => Array.apply(null, {length: n}).map((_, i) => i + 1)

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
