### Binary state transition model

```math
state_A[i] := \begin{cases}
    0 & \\
    1 & \\
    state_A[x] & x = f(...) \\
    state_B[y] & y = g(...) \\
    h(state_A, state_B) & \\
\end{cases}
```

#### Constant zero

```python
for i in range(len(state_A)):
    state_A[i] = 0
```

#### Constant one

```python
for i in range(len(state_A)):
    state_A[i] = 1
```

#### Identity

```python
for i in range(len(state_A)):
    state_A[i] = state_A[i]
```

#### Full copy

```python
for i in range(len(state_A)):
    state_A[i] = state_B[i]
```

#### Most general transformation

```python
for (i, j) in enumerate(targets):
    state_A[j] = state_B[mapping[i]]
```

### Other

Requirements are defined by test cases.

|Requirement|Status|
|---|---|
|Indexing finite collection with out-of-bounds index must fail on compile time.||

|Type|`IndexU8`|`IndexU32`|
|---|---|---|
|`I::<0>`-`I::<7>`|:heavy_check_mark:|:heavy_check_mark:|
|`I::<8>`-`I::<31>`|:x:|:heavy_check_mark:|