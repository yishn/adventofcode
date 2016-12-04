const fs = require('fs')

let input = fs.readFileSync('./input4.txt', 'utf8').trim()
let rooms = input.split('\n')
    .map(x => x.match(/([a-z-]+)-(\d+)\[([a-z]+)\]/))
    .map(([, str, id, check]) => [str, +id, check])

let histogram = str => str.replace(/-/g, '').split('').reduce((hist, v) => (hist[v] = (hist[v] | 0) + 1, hist), {})
let compare = hist => (v, w) => hist[w] - hist[v] || (v < w ? -1 : +(v != w))
let checksum = hist => Object.keys(hist).sort(compare(hist)).slice(0, 5).join('')
let real = ([str, , check]) => checksum(histogram(str)) == check
let sum = array => array.reduce((sum, x) => sum + x)
let realRooms = rooms.filter(real)

console.log('Part 1:\t', sum(realRooms.map(([, x]) => x)))

const alpha = 'abcdefghijklmnopqrstuvwxyz'

let shiftAlpha = n => alpha.slice(n % alpha.length) + alpha.slice(0, n % alpha.length)
let shift = (n, str) => str.split('').map(v => v == '-' ? ' ' : shiftAlpha(n)[alpha.indexOf(v)]).join('')
let decrypt = rooms => rooms.map(([str, id]) => [shift(id, str), id])

console.log('Part 2:\t', decrypt(realRooms).find(([str]) => str[0] == 'n'))
