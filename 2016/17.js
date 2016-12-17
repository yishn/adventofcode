const crypto = require('crypto')

let input = 'gdjjyniy'

let md5 = str => crypto.createHash('md5').update(str).digest('hex')
let valid = ([x, y]) => 1 <= Math.min(x, y) && Math.max(x, y) <= 4
let direction = ([a, b], [c, d]) => c > a ? 'R' : c < a ? 'L' : d > b ? 'D' : 'U'

function getNeighbors([x, y], path) {
    let hash = md5(input + path.join(''))

    return [
        [x, y - 1],
        [x, y + 1],
        [x - 1, y],
        [x + 1, y]
    ].filter((v, i) => valid(v) && 'bcdef'.includes(hash[i]))
}

function getPaths(start, end) {
    let queue = [[start, []]]
    let visited = {[[start, []]]: true}
    let paths = []

    while (queue.length > 0) {
        let [vertex, path] = queue.shift()

        if (vertex[0] == end[0] && vertex[1] == end[1]) {
            paths.push(path)
            continue
        }

        for (let neighbor of getNeighbors(vertex, path)) {
            let newPath = [...path, direction(vertex, neighbor)]

            if ([neighbor, newPath] in visited) continue
            visited[[neighbor, newPath]] = true

            queue.push([neighbor, newPath])
        }
    }

    return paths
}

let paths = getPaths([1, 1], [4, 4])

console.log('Part 1:\t' + paths[0].join(''))
console.log('Part 2:\t' + paths.slice(-1)[0].length)
