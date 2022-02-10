# priq

Array implementation of the min/max heap.

`PriorityQueue` is build on top of raw array for efficient performance.

There are two major reasons what makes this `PriorityQueue` different from
other binary heap implementations currently available:

1 - Allows data ordering to scores with `PartialOrd`.
    - Every other min-max heap requires 
    [total ordering](https://bit.ly/3GCWvYL) of scores (e.g. should 
    implement `Ord` trait). This can be an issue, for example, when you 
    want to order items based on a float scores, which doesn't implement
    `Ord` trait.
    - You can read about Rust's implementation or `Ord`, `PartialOrd` and 
    what's the different [here](https://bit.ly/3J7NwQI)

2 - Separation of score and item you wish to store.
    - This frees enforcement for associated items to implement any ordering.
    - Makes easier to evaluate items' order.

3 - Equal scoring items are stored at first available free space.
    - This gives performance boost for large number of entries.

4 - Easy to use!

## Implementation

You can read more about this crate on [my blog](https://www.bexxmodd.com)

A Min-Max Heap with designated arguments for `score` and associated `item`!

A `Default` implementation is a Min-Heap where the top node (root) is the 
lowest scoring element:

<center><p>10<p></center>
<center><p>/&emsp;&ensp;\</p></center>
<center><p>58&emsp;&emsp;70</p></center>
<center><p>/&emsp;\&emsp;&emsp;/&emsp;\</p></center>
<center><p>80&emsp;&ensp;92&emsp;97&emsp;&ensp;99</p></center>

> The value of Parent Node is smaller than Child Node.

Every parent node, including the top (root) node, is less than or equal to 
the value of its children nodes. And the left child is always less than or 
equal to the right child.
                                                                             
`PriorityQueue ` allows duplicate score/item values. When you [`put`]the 
item with a similar score that’s already in the queue new entry will be 
stored at the first empty location in memory. This gives an incremental 
performance boost (instead of resolving by using the associated item as a 
secondary tool to priority evaluation). Also, this form of implementation 
doesn’t enforce for the element `T` to have any implemented ordering. This
guarantees that the top node will always be of minimum value.
                                                                             
You can initialize an empty `PriorityQueue` and later add items:
                                                                             
```Rust
use priq::priq::PriorityQueue;
                                                                             
let pq: PriorityQueue<usize, String> = PriorityQueue::new();
```
                                                                             
Or you can _heapify_ a `Vec` and/or a `slice`:
                                                                             
```Rust
use priq::priq::PriorityQueue;
                                                                             
let pq_from_vec = PriorityQueue::from(vec![(5, 55), (1, 11), (4, 44)]);
let pq_from_slice = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
```

The standard usage of this data structure is to [`put`] an element to the 
queue and [`pop`] to remove the top element and peek to check what’s the 
top element in the queue. The stored structure of the elements is a balanced
tree realized using an array with a contiguous memory location. This allows
maintaining a proper parent-child relationship between put-ed items.
                                                                             
Runtime complexity with Big-O Notation:

| method    | Time Complexity |
|-----------|-----------------|
| [`put`]   | _O(log(n))_     |
| [`pop`]   | _O(log(n))_     |
| [`peek`]  | _O(1)_          |
                                                                             
You can also iterate over elements using for loop but the returned slice 
will not be properly order as the heap is re-balanced after each insertion 
and deletion. If you want to grab items in a proper priority call [`pop`] 
in a loop until it returns `None`.
                                                                             
What if you want to custom `struct ` without having a separate and 
specific score? You can pass the `struct`’s clone as a `score` and as an 
associated value, but if in this kind of scenario I’d recommend using
[`std::collections::binary_heap`] as it better fits the purpose.
                                                                             
If instead of Min-Heap you want to have Max-Heap, where the highest-scoring 
element is on top you can pass score using [`std::cmp::Reverse`] or a custom
[`Ord`] implementation can be used to have custom prioritization logic.
                                                                             
# Example
                                                                             
```
use std::cmp::Reverse;
use priq::priq::PriorityQueue;
                                                                             
let mut pq: PriorityQueue<Reverse<u8>, String> = PriorityQueue::new();
pq.put(Reverse(26), "Z".to_string());
pq.put(Reverse(1), "A".to_string());
assert_eq!(pq.pop().unwrap().1, "Z");
```

## Performance

This are the benchmark results for `priq::PriorityQueue`:


| `priq` benches | median | nanosecs | std.dev |
|-----|-------:|:----------:|:--------|
| pq_pop_100      |        146 | ns/iter | (+/- 1)
| pq_pop_100k     |    291,818 | ns/iter | (+/- 5,686)
| pq_pop_10k      |     14,129 | ns/iter | (+/- 39)
| pq_pop_1k       |      1,646 | ns/iter | (+/- 32)
| pq_pop_1mil     | 16,517,047 | ns/iter | (+/- 569,128|
| pq_put_100      |        488 | ns/iter | (+/- 21)
| pq_put_100k     |    758,422 | ns/iter | (+/- 13,961)
| pq_put_100k_wcap|    748,824 | ns/iter | (+/- 7,926)
| pq_put_10k      |     80,668 | ns/iter | (+/- 1,324)
| pq_put_1k       |      8,769 | ns/iter | (+/- 78)
| pq_put_1mil     |  6,728,203 | ns/iter | (+/- 76,416)
| pq_put_1mil_wcap|  6,622,341 | ns/iter | (+/- 77,162)


How it compares to `std::collections::BinaryHeap`:

| `BinaryHeap` benches | median | nanosecs | std.dev |
|-----|-------:|:----------:|:--------|
| bh_pop_100  |         272 | ns/iter | (+/- 90)
| bh_pop_100k |     171,071 | ns/iter | (+/- 6,131)
| bh_pop_10k  |      13,904 | ns/iter | (+/- 130)
| bh_pop_1k   |       1,847 | ns/iter | (+/- 6)
| bh_pop_1mil |   8,772,066 | ns/iter | (+/- 611,613)
| bh_push_100 |         857 | ns/iter | (+/- 50)
| bh_push_100k|     943,465 | ns/iter | (+/- 108,698)
| bh_push_10k |      92,807 | ns/iter | (+/- 7,930)
| bh_push_1k  |       8,606 | ns/iter | (+/- 639)
| bh_push_1mil|  12,946,815 | ns/iter | (+/- 900,347)
