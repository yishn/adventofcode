let input = 3014387
let state = Array(input).fill(true)
let participants = state.length

let printProgress = p => process.stdout.write(p < 1 ? '\r[' + [
    Array(Math.round(p * 20)).fill('='),
    Array(20 - Math.round(p * 20)).fill(' ')
].map(x => x.join('')).join('>') + `] ${Math.round(p * 100)}% ` : '\r' + Array(30).fill(' ').join('') + '\r')

function leftNeighborElf(index) {
    let i = index + 1
    if (i == state.length) i = 0

    while (!state[i]) {
        i++
        if (i == state.length) i = 0
    }

    return i
}

function playGame1() {
    let i = 0

    printProgress(0)

    while (participants > 1) {
        if (participants % 100000 == 0)
            printProgress(1 - participants / state.length)

        state[leftNeighborElf(i)] = false
        participants--

        i = leftNeighborElf(i)
    }

    printProgress(1)
    return state.findIndex(x => x)
}

console.log('Part 1:\t' + (playGame1() + 1))

state = Array(input).fill(1)
participants = state.length

function acrossElf(index) {
    let skip = Math.floor(participants / 2)
    let i = index

    while (skip-- > 0) {
        i = leftNeighborElf(i)
    }

    return i
}

function playGame2() {
    let i = 0
    let j = acrossElf(i)

    printProgress(0)

    while (participants > 1) {
        if (participants % 100000 == 0)
            printProgress(1 - participants / state.length)

        state[j] = false
        participants--

        i = leftNeighborElf(i)
        j = leftNeighborElf(j)

        if (participants % 2 == 0)
            j = leftNeighborElf(j)
    }

    printProgress(1)
    return state.findIndex(x => x)
}

console.log('Part 2:\t' + (playGame2() + 1))
