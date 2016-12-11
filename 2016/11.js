const fs = require('fs')

let input = fs.readFileSync('./input11.txt', 'utf8').trim()

function parse(str) {
    let floors = str.split('\n').map(line => {
        return line.split('contains ')[1].split(/, and a |, a |and a |a /)
            .map(x => x.replace(/(-compatible|\.)/g, '').trim())
            .filter(x => !['', 'nothing relevant'].includes(x))
            .map(x => x.split(' '))
    })

    return [floors, 0]
}

function copy([floors, elevator]) {
    return [floors.map(floor => floor.slice(0)), elevator]
}

function isEndState([floors, ]) {
    return floors.slice(0, -1).every(floor => floor.length == 0)
}

function isValid([floors, elevator]) {
    return 0 <= elevator && elevator < floors.length && floors.every(floor => {
        let generators = floor.filter(([, type]) => type == 'generator')

        return generators.length == 0
            || floor.every(([el, type]) => type == 'generator' || generators.some(([x, ]) => x == el))
    })
}

function* powerSet(array, n) {
    if (n == 0 || array.length == 0 || n > array.length) {
        yield []
        return
    }

    for (let i = 0; i < array.length - n + 1; i++) {
        for (let rest of powerSet(array.slice(i + 1), n - 1)) {
            rest.unshift(array[i])
            yield rest
        }
    }
}

function* listSteps(state) {
    let [floors, elevator] = state

    for (let n = 1; n <= 2; n++) {
        for (let objects of powerSet(floors[elevator], n)) {
            if (objects.length == 0) continue

            if (objects.length == 2) {
                let [[a, b], [c, d]] = objects
                if (a != c && b != d) continue
            }

            for (let newElevator = elevator - 1; newElevator <= elevator + 1; newElevator += 2) {
                if (newElevator < 0 || newElevator >= floors.length)
                    continue
                if (newElevator < elevator && floors.slice(0, elevator).every(floor => floor.length == 0))
                    continue

                let newState = copy(state)
                let [newFloors, ] = newState
                newState[1] = newElevator

                for (let [el, type] of objects) {
                    let i = newFloors[elevator].findIndex(([x, y]) => x == el && y == type)
                    newFloors[newElevator].push(...newFloors[elevator].splice(i, 1))
                }

                newFloors[newElevator].sort()

                if (isValid(newState)) yield newState
            }
        }
    }
}

function eqClass([floors, elevator]) {
    let n = floors.reduce((sum, floor) => sum + floor.length, 0) / 2
    let objects = [...Array(n)].map(_ => Array(2).fill(null))
    let names = []

    for (let i = 0; i < floors.length; i++) {
        for (let [el, type] of floors[i]) {
            let j = names.indexOf(el)

            if (j < 0) {
                names.push(el)
                j = names.length - 1
            }

            objects[j][type == 'generator' ? 0 : 1] = i
        }
    }

    return objects.sort().join(';') + ';' + elevator
}

function bfs(state) {
    let queue = [state]
    let key = eqClass(state)
    let parents = {[key]: null}

    let getPath = function(end) {
        let path = [end]
        let key = eqClass(end)

        while (parents[key] != null) {
            path.push(parents[key])
            key = eqClass(parents[key])
        }

        return path.reverse()
    }

    while (queue.length > 0) {
        let current = queue.shift()

        if (isEndState(current)) return getPath(current)

        for (let neighbor of listSteps(current)) {
            let key = eqClass(neighbor)
            if (key in parents) continue
            parents[key] = current

            queue.push(neighbor)
        }
    }

    return null
}

let state = parse(input)
let path = bfs(state)

console.log('Part 1:\t' + (path.length - 1)) // ~2 sec

state[0][0].push(
    ['elerium', 'generator'],
    ['elerium', 'microchip'],
    ['dilithium', 'generator'],
    ['dilithium', 'microchip']
)

path = bfs(state)

console.log('Part 2:\t' + (path.length - 1)) // ~9 sec
