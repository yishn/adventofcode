const crypto = require('crypto')

let input = 'jlmsuwbz'

let md5 = str => crypto.createHash('md5').update(str).digest('hex')
let getInARow = (n, str) => str.split('').filter((x, i) => str.slice(i, i + n) == Array(n).fill(x).join(''))
let times = (n, f) => (...args) => n == 1 ? f(...args) : times(n - 1, f)(f(...args))

let printProgress = p => process.stdout.write(p < 1 ? '\r[' + [
    Array(Math.round(p * 20)).fill('='),
    Array(20 - Math.round(p * 20)).fill(' ')
].map(x => x.join('')).join('>') + `] ${Math.round(p * 100)}% ` : '\r' + Array(30).fill(' ').join('') + '\r')

function generatePad(salt, stretch = x => x) {
    printProgress(0)

    let id = 0
    let candidates = []
    let keys = new Set()

    while (keys.size < 64) {
        let key = md5(stretch(salt + id))
        let threeInARow = getInARow(3, key)

        if (threeInARow.length > 0) {
            candidates.push([id, key, threeInARow[0]])

            let i = candidates.findIndex(([x, ]) => x >= id - 1000)
            candidates.splice(0, i)

            let fiveInARow = getInARow(5, key)

            if (fiveInARow.length > 0) {
                let newKeys = candidates.filter(([x, , y]) => x != id && fiveInARow.some(z => z == y))
                newKeys.forEach(x => keys.add(x))

                printProgress(Math.min(1, keys.size / 64))
            }
        }

        id++
    }

    return [...keys].sort(([x, , ], [y, , ]) => x - y)
}

console.log('Part 1:\t' + generatePad(input)[63][0])
console.log('Part 2:\t' + generatePad(input, times(2016, md5))[63][0])
