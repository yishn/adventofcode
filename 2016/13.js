let input = 1364

function countBinaryOnes(x) {
    if (x <= 1) return x

    let i = Math.floor(Math.log2(x))
    return 1 + countBinaryOnes(x - Math.pow(2, i))
}

function valid([x, y]) {
    if (x < 0 || y < 0) return false

    let sum = x*x + 3*x + 2*x*y + y + y*y + input
    return countBinaryOnes(sum) % 2 == 0
}

function neighbors([x, y]) {
    return [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]].filter(valid)
}

function bfs(start, end, maxDist = Infinity) {
    let queue = [[start, 0]]
    let parents = {[start]: null}

    while (queue.length > 0) {
        let [vertex, dist] = queue.shift()

        if (vertex.join(',') == end.join(','))
            return parents

        for (let neighbor of neighbors(vertex)) {
            if (neighbor in parents) continue
            if (dist + 1 > maxDist) continue

            parents[neighbor] = vertex
            queue.push([neighbor, dist + 1])
        }
    }

    return parents
}

function getPath(parents, end) {
    let path = [end]

    while (parents[path[0]] != null) {
        path.unshift(parents[path[0]])
    }

    return path
}

let parents = bfs([1, 1], [31, 39])
console.log('Part 1:\t' + (getPath(parents, [31, 39]).length - 1))

parents = bfs([1, 1], [-1, -1], 50)
console.log('Part 2:\t' + Object.keys(parents).length)
