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

> The value of Parent Node is small than Child Node.

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
                                                                             
```
use priq::priq::PriorityQueue;
                                                                             
let pq: PriorityQueue<usize, String> = PriorityQueue::new();
```
                                                                             
Or you can _heapify_ a `Vec` and/or a `slice`:
                                                                             
```
use priq::priq::PriorityQueue;
                                                                             
let pq_from_vec = PriorityQueue::from(vec![(5, 55), (1, 11), (4, 44)]);
let pq_from_slice = PriorityQueue::from([(5, 55), (1, 11), (4, 44)]);
```

The standard usage of this data structure is to [`put`] an element to the 
queue and [`pop`] to remove the top element and peek to check what’s the 
top element in the queue. The stored structure of the elements is a balanced
tree realized using an array with a contiguous memory location. This allows
maintaining a proper parent-child relationship between put-ed items.
                                                                             
[`put`]: PriorityQueue::put
[`peek`]: PriorityQueue::peek
[`pop`]: PriorityQueue::pop
                                                                             
                                                                             
Runtime complexity with Big-O Notation:

| method    | Time Complexity |
|-----------|-----------------|
| [`put`]   | _O(log(n))_     |
| [`pop`]   | _O(long(n))_    |
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
