function lsearch<T> (v: Array<T>, c: (e:T) => boolean): boolean {
  let i = 0
  while (i < v.length) {
    if (c(v[i])) { return true }
    i += 1
}
  return false
}

const vec = [1, 4, 8, 9, -1, -2, 8, 10]

// test run 1
function equal_8 (i: number): boolean { return i === 8 }
console.log(lsearch(vec, equal_8))

// test run 2
function equal_7 (i: number): boolean { return i === 7 }
console.log(lsearch(vec, equal_7))
