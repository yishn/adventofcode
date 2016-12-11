const fs = require('fs')

let input = fs.readFileSync('./input06.txt', 'utf8').trim()
let data = input.split('\n').map(x => x.trim().split(''))

data = data[0].map((_, i) => data.map(x => x[i]))

let histogram = arr => arr.reduce((hist, x) => (hist[x] = (hist[x] || 0) + 1, hist), {})
let histCompare = hist => (x, y) => hist[y] - hist[x]
let retrieve = (data, compare) => data.map(x => x.sort(compare(x))[0]).join('')

console.log('Part 1:\t' + retrieve(data, x => histCompare(histogram(x))))

let reverseCompare = compare => (x, y) => compare(y, x)

console.log('Part 2:\t' + retrieve(data, x => reverseCompare(histCompare(histogram(x)))))
