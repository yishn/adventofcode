const crypto = require('crypto')

let input = 'abbhdwsy'

let md5 = str => crypto.createHash('md5').update(str).digest('hex')
let printCode = code => code.reduce((s, x) => s + (x != null ? x : '_'), '')

function solve(length, predicate, update) {
    let code = Array.apply(null, {length})
    let index = 0

    while (code.filter(x => x != null).length < code.length) {
        let str = input + index
        let hash = md5(str)

        if (predicate(hash)) {
            code = update(code, hash)
            console.log(printCode(code))
        }

        index++
    }

    return code.join('')
}

let interesting = hash => hash.slice(0, 5).split('').every(x => x == 0)
let update5 = (code, hash) => (code[code.findIndex(x => x == null)] = hash[5], code)

console.log('Part 1:\t' + solve(8, interesting, update5))

let valid = length => hash => interesting(hash) && !isNaN(hash[5]) && +hash[5] < length
let update6 = (code, hash) => (code[hash[5]] = code[hash[5]] || hash[6], code)

console.log()
console.log('Part 2:\t' + solve(8, valid(8), update6))
