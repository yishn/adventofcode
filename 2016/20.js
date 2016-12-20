const fs = require('fs')

let input = fs.readFileSync('./input20.txt', 'utf8').trim()
let data = input.split('\n').map(x => x.match(/(\d+)-(\d+)/).slice(1)).map(x => x.map(y => +y))
let includes = n => ([a, b]) => a <= n && n <= b
let isValid = ranges => n => !ranges.some(includes(n))
let count = ranges => ranges.reduce((sum, [min, max]) => sum + (max - min + 1), 0)

function searchForLowestValid(ranges) {
    let candidates = [0]

    for (let [min, max] of ranges) {
        if (max < 4294967295) candidates.push(max + 1)
    }

    return Math.min(...candidates.filter(isValid(ranges)))
}

console.log('Part 1:\t' + searchForLowestValid(data))

function getDisjointRanges(ranges) {
    let result = []

    for (let [min, max] of ranges) {
        let minCollisionRange = result.findIndex(includes(min))
        let maxCollisionRange = result.findIndex(includes(max))

        let [newMin, newMax] = [min, max]

        if (minCollisionRange >= 0 && maxCollisionRange >= 0) {
            [newMin, newMax] = [result[minCollisionRange][0], result[maxCollisionRange][1]]
        } else if (minCollisionRange >= 0) {
            [newMin, newMax] = [result[minCollisionRange][0], max]
        } else if (maxCollisionRange >= 0) {
            [newMin, newMax] = [min, result[maxCollisionRange][1]]
        }

        result = [...result.filter(([a, b]) => newMin > a || b > newMax), [newMin, newMax]]
    }

    return result
}

console.log('Part 2:\t' + (4294967295 - count(getDisjointRanges(data)) + 1))
