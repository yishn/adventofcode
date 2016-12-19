let input = 3014387
let state = Array(input).fill(true)
let participants = state.length

let printProgress = p => process.stdout.write(p < 1 ? '\r[' + [
    Array(Math.round(p * 20)).fill('='),
    Array(20 - Math.round(p * 20)).fill(' ')
].map(x => x.join('')).join('>') + `] ${Math.round(p * 100)}% ` : '\r' + Array(30).fill(' ').join('') + '\r')

function stealPresent(index, steal) {
    if (!state[index]) return

    state[steal] = false
    participants--
}

function leftNeighborElf(index) {
    let i = (index + 1) % state.length

    while (!state[i])
        i = (i + 1) % state.length

    return i == index ? null : i
}

function playGame(nextIndex) {
    let i = 0
    let lastPercent = 0

    printProgress(0)

    while (participants > 1) {
        let percent = 1 - participants / state.length

        if (percent - lastPercent >= 0.05) {
            printProgress(percent)
            lastPercent = percent
        }

        let n = nextIndex(i)
        stealPresent(i, n)
        i = leftNeighborElf(i)
    }

    printProgress(1)
    return state.findIndex(x => x)
}

console.log('Part 1:\t' + (playGame(leftNeighborElf) + 1))
