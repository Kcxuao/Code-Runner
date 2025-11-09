```
curl -X POST http://localhost:3000/run \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "def add(a, b):\n    return a + b",
    "function": "add",
    "test_cases": [
      {"input": [1, 2], "expected": 3},
      {"input": [5, 7], "expected": 12}
    ]
  }'

```
