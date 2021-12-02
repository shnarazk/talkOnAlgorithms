function bsort<T> (v: Array<T>): Array<T> {
  for (let i = 0; i < v.length - 1; i++) {
    for (let j = v.length - 1; i < j; j--) {
      if v[j - 1] > v[j] {
        const temp= v[j - 1]
        v[j - 1] = v[j]
        v[j] = temp
      }
    }
  return v
}

const vec1 = [1, 4, 8, 9, -1, -2, 8, 10]
console.log({ vec1, sorted: bsort(vec1) })
