# xdiff

Given `A = abcabba` and `B = cbabac` expected result

```diff
- A
- B
  C
- A
  B
+ A
  B
  A
+ C

or

- A
+ C
  B
- C
  A
  B
- B
  A
+ C

or

+ C
- A
  B
- C
  A
  B
- B
  A
+ C
```
