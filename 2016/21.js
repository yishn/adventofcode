const fs = require('fs')

let input = fs.readFileSync('./input21.txt', 'utf8').trim()

let instructions = input.split('\n').map(line => {
    let [type, ...words] = line.trim().split(' ').map(x => isNaN(x) ? x : +x)

    if (type == 'swap' || type == 'move') {
        return [type, words[1], words[4]]
    } else if (type == 'rotate') {
        if (words[0] == 'based') return [type, words[5], null]
        return [type, words[0], words[1]]
    } else if (type == 'reverse') {
        return [type, words[1], words[3]]
    }

    return null
})

let rotate = {
    'left': (arr, n) => [...arr.slice(n % arr.length), ...arr.slice(0, n % arr.length)],
    'right': (arr, n) => rotate.left(arr, -n)
}

function scramble(str) {
    str = [...str]

    for (let [type, a, b] of instructions) {
        if (type == 'move') {
            let [x] = str.splice(a, 1)
            str.splice(b, 0, x)
        } else if (type == 'reverse') {
            let xs = str.splice(a, b - a + 1)
            xs.reverse()
            str.splice(a, 0, ...xs)
        } else if (type == 'swap') {
            if (!isNaN(a)) {
                [str[a], str[b]] = [str[b], str[a]]
            } else {
                str = str.map(x => x == a ? b : x == b ? a : x)
            }
        } else if (type == 'rotate') {
            if (b != null) {
                str = rotate[a](str, b)
            } else {
                let i = str.indexOf(a)
                str = rotate.right(str, 1 + i)
                if (i >= 4) str = rotate.right(str, 1)
            }
        } else {
            return null
        }
    }

    return str.join('')
}

console.log('Part 1:\t' + scramble('abcdefgh'))

function unscramble(str) {
    str = [...str]

    for (let [type, a, b] of [...instructions].reverse()) {
        if (type == 'move') {
            let [x] = str.splice(b, 1)
            str.splice(a, 0, x)
        } else if (type == 'reverse') {
            let xs = str.splice(a, b - a + 1)
            xs.reverse()
            str.splice(a, 0, ...xs)
        } else if (type == 'swap') {
            if (!isNaN(a)) {
                [str[a], str[b]] = [str[b], str[a]]
            } else {
                str = str.map(x => x == a ? b : x == b ? a : x)
            }
        } else if (type == 'rotate') {
            if (b != null) {
                str = rotate[a == 'left' ? 'right' : 'left'](str, b)
            } else {
                for (let i = 0; i < str.length; i++) {
                    str = rotate.left(str, i == 4 ? 2 : 1)
                    if (i == str.indexOf(a)) break
                }
            }
        } else {
            return null
        }
    }

    return str.join('')
}

console.log('Part 2:\t' + unscramble('fbgdceah'))
