import time

def eval_direct(a, b):
    c = a + b
    if c > 10:
        return c * 2
    else:
        return c - 1

a = 9
b = 10
rounds = 100_000

start = time.perf_counter()
result = 0
for _ in range(rounds):
    result = eval_direct(a, b)
end = time.perf_counter()

duration = end - start
duration_ns = duration * 1e9  # convert to nanoseconds

print(f"Result: {result}")
print(f"Total time: {duration * 1000:.4f} ms")
print(f"Average per iteration: {duration_ns / rounds:.2f} ns")