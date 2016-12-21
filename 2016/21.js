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
       left: (arr, n) => (n = n % arr.length, [...arr.slice(n), ...arr.slice(0, n)]),
      right: (arr, n) => rotate.left(arr, -n)
}

let doInstruction = (str, [type, a, b]) => ({
       move: () => (str = [...str], [a] = str.splice(a, 1), [...str.slice(0, b), a, ...str.slice(b)]),
    reverse: () => [...str.slice(0, a), ...str.slice(a, b + 1).reverse(), ...str.slice(b + 1)],
       swap: () => !isNaN(a) ? (str = [...str], [str[a], str[b]] = [str[b], str[a]], str)
                   : str.map(x => x == a ? b : x == b ? a : x),
     rotate: () => b != null ? rotate[a](str, b)
                   : (b = str.indexOf(a), rotate.right(str, +(b >= 4) + 1 + b))
})[type]()

let scramble = str => instructions.reduce(doInstruction, [...str]).join('')

console.log('Part 1:\t' + scramble('abcdefgh'))

let reverseInstruction = (str, [type, a, b]) => ({
       move: () => doInstruction(str, [type, b, a]),
    reverse: () => doInstruction(str, [type, a, b]),
       swap: () => doInstruction(str, [type, a, b]),
     rotate: () => b != null ? doInstruction(str, [type, a, -b])
                   : str.map((_, i) => rotate.left(str, +(i >= 4) + 1 + i)).find((x, i) => x.indexOf(a) == i)
})[type]()

let unscramble = str => [...instructions].reverse().reduce(reverseInstruction, [...str]).join('')

console.log('Part 2:\t' + unscramble('fbgdceah'))
