# Radix sort

- 非比較式排序法（Non-comparative sort）
- 整數排序法（Integer sort）：以整數作為排序的鍵值
- 是穩定排序（採用 LSD），也可以是不穩定排序（採用 MSD）

## Algorithm

Radix sort 是比較整數的每個位數來排序，依照位數排序的先後順序不同，可分為兩種：

- Least significant digit (LSD)
- Most significant digit (MSE)


### LSD Radix sort

Each key is first figuratively dropped into one level of buckets corresponding to the value of the rightmost digit. Each bucket preserves the original order of the keys as the keys are dropped into the bucket. There is a one-to-one correspondence between the buckets and the values that can be represented by the rightmost digit. Then, the process repeats with the next neighbouring more significant digit until there are no more digits to process. In other words:

    Take the least significant digit (or group of bits, both being examples of radices) of each key.
    Group the keys based on that digit, but otherwise keep the original order of keys. (This is what makes the LSD radix sort a stable sort.)
    Repeat the grouping process with each more significant digit.


### MSD Radix sort

## Explanation

## Performance

|              | Complexity |
| :----------- | :--------- |
| Worst case   |            |
| Best case    |            |
| Average case |            |
| Worst space  |  auxiliary |

## Implementation

## Reference


https://en.wikipedia.org/wiki/Integer_sorting
