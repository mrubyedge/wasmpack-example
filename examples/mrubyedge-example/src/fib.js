import { fib } from 'mrubyedge-wasmpack-example'

export function setupFib(element) {
  let counter = 0
  const setCounterWithFib = (count) => {
    counter = count
    element.innerHTML = `fib(${counter}) is ${fib(counter)}`
  }
  element.addEventListener('click', () => setCounterWithFib(counter + 1))
  setCounterWithFib(0)
}
