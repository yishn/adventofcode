const fs = require('fs')

let input = fs.readFileSync('./input20.txt', 'utf8').trim()

let data = input.split('\n').map(x => x.match(/(\d+)-(\d+)/).slice(1)).map(x => x.map(y => +y))
let includes = (...ns) => ([a, b]) => ns.every(n => a <= n && n <= b)
let isValid = ranges => n => !ranges.some(includes(n))

function searchForLowestValid(ranges) {
    let candidates = [0]

    for (let [min, max] of ranges) {
        if (max < 4294967295) candidates.push(max + 1)
    }

    return Math.min(...candidates.filter(isValid(ranges)))
}

console.log('Part 1:\t' + searchForLowestValid(data))

let notBetween = (a, b) => ns => !includes(...ns)([a, b])
let count = ranges => ranges.reduce((sum, [min, max]) => sum + (max - min + 1), 0)

function getDisjointRanges(ranges) {
    let result = []

    for (let [min, max] of ranges) {
        let minCollisionRange = result.find(includes(min))
        let maxCollisionRange = result.find(includes(max))

        let [newMin, newMax] = [min, max]

        if (minCollisionRange && maxCollisionRange) {
            [newMin, newMax] = [minCollisionRange[0], maxCollisionRange[1]]
        } else if (minCollisionRange) {
            newMin = minCollisionRange[0]
        } else if (maxCollisionRange) {
            newMax = maxCollisionRange[1]
        }

        result = [...result.filter(notBetween(newMin, newMax)), [newMin, newMax]]
    }

    return result
}

console.log('Part 2:\t' + (4294967295 - count(getDisjointRanges(data)) + 1))
