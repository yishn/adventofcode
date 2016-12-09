const fs = require('fs')

let input = fs.readFileSync('./input9.txt', 'utf8').trim()

function count1(str) {
    let i = str.indexOf('(')

    if (i < 0) {
        return str.length
    } else if (i != 0) {
        return i + count1(str.slice(i))
    }

    let [marker, x, y] = str.match(/\((\d+)x(\d+)\)/)
    x = +x
    y = +y

    return y * x + count1(str.slice(marker.length + x))
}

console.log('Part 1:\t' + count1(input))

function count2(str) {
    let i = str.indexOf('(')

    if (i < 0) {
        return str.length
    } else if (i != 0) {
        return i + count2(str.slice(i))
    }

    let [marker, x, y] = str.match(/\((\d+)x(\d+)\)/)
    x = +x
    y = +y

    let chunk = str.slice(marker.length, marker.length + x)
    return y * count2(chunk) + count2(str.slice(marker.length + x))
}

console.log('Part 2:\t' + count2(input))
