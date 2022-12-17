# Vectored reads and writes with scatter and gather

```plantuml
title ReadBit

map Value {
    0 => x0
    1 => x1
    2 => x2
    3 => x3
    4 => x4
    5 => x5
    6 => x6
    7 => x7
}

object X {
    index: 5
}

object Y {
    value: x5
}

X -> Value::5
Value::5 -> Y
```
