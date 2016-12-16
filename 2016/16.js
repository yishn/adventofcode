let input = '10111011111001111'

let printProgress = p => process.stdout.write(p < 1 ? '\r[' + [
    Array(Math.round(p * 20)).fill('='),
    Array(20 - Math.round(p * 20)).fill(' ')
].map(x => x.join('')).join('>') + `] ${Math.round(p * 100)}% ` : '\r' + Array(30).fill(' ').join('') + '\r')

let dragonCurve = x => [x, [...x].reverse().map(x => x == '0' ? '1' : '0').join('')].join('0')

function generateData(data, size) {
    printProgress(0)

    while (data.length < size) {
        data = dragonCurve(data)
        printProgress(Math.min(data.length * 0.5 / (size - 1), 0.5))
    }

    return data.slice(0, size)
}

function checksum(data) {
    let result = []
    let p = 0.5

    printProgress(p)

    while (result.length % 2 == 0) {
        result = []

        for (let i = 0; i < data.length - 1; i += 2) {
            result.push(data[i] == data[i + 1] ? '1' : '0')
        }

        data = result
        p += (1 - p) * 0.5
        printProgress(p)
    }

    printProgress(1)
    return result.join('')
}

let data = generateData(input, 35651584)

console.log('Part 1:\t' + checksum(data.slice(0, 272)))
console.log('Part 2:\t' + checksum(data))
