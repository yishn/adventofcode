const fs = require('fs')

let input = fs.readFileSync('./input7.txt', 'utf8').trim()
let data = input.split('\n').map(x => x.trim().split(/[\[\]]/))

let hasABBA = str => str.split('').some((x, i) => x != str[i + 1]
    && str.slice(i, i + 4) == x + str[i + 1] + str[i + 1] + x)
let supportsTLS = match => match.filter((_, i) => i % 2 == 0).some(hasABBA)
    && !match.filter((_, i) => i % 2 != 0).some(hasABBA)

console.log('Part 1:\t' + data.filter(supportsTLS).length)

let isABA = str => str.length == 3 && str[0] == str[2] && str[0] != str[1]
let getABAs = str => str.split('').map((x, i) => str.slice(i, i + 3)).filter(isABA)
let getAllABAs = match => getABAs(match.filter((_, i) => i % 2 == 0).join(' '))
let hasBAB = aba => str => str.split('').some((x, i) => str.slice(i, i + 3) == aba[1] + aba[0] + aba[1])
let supportsSSL = match => getAllABAs(match).some(aba => match.filter((_, i) => i % 2 != 0).some(hasBAB(aba)))

console.log('Part 2:\t' + data.filter(supportsSSL).length)
