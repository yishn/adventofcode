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
        if (participants % 100000 == 0) {
            let percent = 1 - participants / state.length
            printProgress(percent)
        }

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
    let skip = participants / 2
    let i = index

    while (skip-- >= 1) {
        i = leftNeighborElf(i)
    }

    return i
}

function playGame2() {
    let i = 0

    printProgress(0)

    while (participants > 1) {
        let index = i
        let across = acrossElf(i)
        let j = across

        let counter = 1
        let limit = participants / 2

        while (true) {
            if (participants % 100000 == 0) {
                let percent = 1 - participants / state.length
                printProgress(percent)
            }

            state[j] = false

            i = leftNeighborElf(i)

            if (participants % 2 == 1) {
                j = leftNeighborElf(leftNeighborElf(j))
            } else {
                j = leftNeighborElf(j)
            }

            participants--

            if (counter++ >= limit) break
        }
    }

    printProgress(1)
    return state.findIndex(x => x)
}

console.log('Part 2:\t' + (playGame2() + 1))
