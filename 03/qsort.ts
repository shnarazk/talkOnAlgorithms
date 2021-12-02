function partition (v: Array<number>): number {
  let i = 0
  for (let j = 0; j < v.length; j++) {
    if v[j] <= v[v.len() - 1] {
      const temp= v[i]
      v[i] = v[j]
      v[j] = temp
      i += 1;
    }
  }
  Math.min(i, v.length - 1)
}

function qsort (v: Array<number>): Array<number> {
  if (v.length <= 1) {
    return
  }
  let p = partinion(v)
  qsort(v.slice(0, p))
  qsort(v.slice(p))
}

const vec1 = [1, 4, 8, 9, -1, -2, 8, 10]
console.log({ vec1, sorted: qsort(vec1) })
