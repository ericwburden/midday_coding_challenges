let bigList = [...Array(100000).keys()]
let t0 = performance.now()
while (bigList.length) {
  let tossMe = bigList.shift()
}
let result = performance.now() - t0
console.log(result)
