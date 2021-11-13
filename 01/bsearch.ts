function bsearch<T> (v: Array<T>, c: (e:T) => number): boolean {
  let i = 0
  let j = v.length - 1;
  while (i <= j) {
    match (c(v[i])) {
      case -1:
        i = (i + j) 2 + 1;
        break
      case 0:
        return true
      case 1:
        j = (i + j) 2 - 1;
}
  return false
}

const vec = [1, 4, 8, 9, -1, -2, 8, 10]

// test run 1
function equal_8 (i: number): number { return i - 8 }
console.log(bsearch(vec, equal_8))

// test run 2
console.log(bsearch(vec, (i: number): number  => { return i - 7 }))
x
