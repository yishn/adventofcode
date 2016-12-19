let input = 3014387

let printProgress = p => process.stdout.write(p < 1 ? '\r[' + [
    Array(Math.round(p * 20)).fill('='),
    Array(20 - Math.round(p * 20)).fill(' ')
].map(x => x.join('')).join('>') + `] ${Math.round(p * 100)}% ` : '\r' + Array(30).fill(' ').join('') + '\r')

function leftNeighborElf(state, index) {
    let i = index + 1
    if (i == state.length) i = 0

    while (!state[i]) {
        i++
        if (i == state.length) i = 0
    }

    return i
}

function playGame1(n) {
    let state = Array(n).fill(true)
    let participants = state.length
    let i = 0

    printProgress(0)

    while (participants > 1) {
        if (participants % 100000 == 0)
            printProgress(1 - participants / state.length)

        state[leftNeighborElf(state, i)] = false
        participants--

        i = leftNeighborElf(state, i)
    }

    printProgress(1)
    return i
}

console.log('Part 1:\t' + (playGame1(input) + 1))

function playGame2(n) {
    let state = Array(n).fill(true)
    let participants = state.length
    let i = 0
    let j = Math.floor(participants / 2)

    printProgress(0)

    while (participants > 1) {
        if (participants % 100000 == 0)
            printProgress(1 - participants / state.length)

        state[j] = false
        participants--

        i = leftNeighborElf(state, i)
        j = leftNeighborElf(state, j)

        if (participants % 2 == 0)
            j = leftNeighborElf(state, j)
    }

    printProgress(1)
    return i
}

console.log('Part 2:\t' + (playGame2(input) + 1))
