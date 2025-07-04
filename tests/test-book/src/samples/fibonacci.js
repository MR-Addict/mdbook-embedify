function fibonacci(n) {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

function generateSequence(length) {
  const sequence = [];
  for (let i = 0; i < length; i++) {
    sequence.push(fibonacci(i));
  }
  return sequence;
}

console.log(generateSequence(10));
