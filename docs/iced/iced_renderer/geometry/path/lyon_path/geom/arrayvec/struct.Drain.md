---
title: "Drain"
kind: struct
path: "iced_renderer::geometry::path::lyon_path::geom::arrayvec::Drain"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::arrayvec"
file: "iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.Drain.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/arrayvec/struct.Drain.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::arrayvec::Drain`

```rust
pub struct Drain<'a, T, const CAP: usize> where
    T: 'a, { /* private fields */ }
```

A draining iterator for `ArrayVec`.

**Source:** [arrayvec.rs.html#1004](https://docs.rs/arrayvec/0.7/src/arrayvec/arrayvec.rs.html#1004) (lines 1004)

## Trait Implementations

### `impl<'a, T, const CAP: usize> DoubleEndedIterator for Drain<'a, T, CAP> where T: 'a,`

#### `next_back`

```rust
fn next_back(&mut self) -> Option<<Drain<'a, T, CAP> as Iterator>::Item>
```

Removes and returns an element from the end of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html#tymethod.next_back)

#### `advance_back_by`

```rust
fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>>
```

*🔬This is a nightly-only experimental API. (iter_advance_by)*

Advances the iterator from the back by `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html#method.advance_back_by)

#### `nth_back`

```rust
fn nth_back(&mut self, n: usize) -> Option<Self::Item>
```

Returns the `n`th element from the end of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html#method.nth_back)

#### `try_rfold`

```rust
fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R where Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,
```

This is the reverse version of [`Iterator::try_fold()`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold): it takes elements starting from the back of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html#method.try_rfold)

#### `rfold`

```rust
fn rfold<B, F>(self, init: B, f: F) -> B where Self: Sized, F: FnMut(B, Self::Item) -> B,
```

An iterator method that reduces the iterator’s elements to a single, final value, starting from the back. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html#method.rfold)

#### `rfind`

```rust
fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Searches for an element of an iterator from the back that satisfies a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html#method.rfind)

### `impl<'a, T, const CAP: usize> Drop for Drain<'a, T, CAP> where T: 'a,`

#### `drop`

```rust
fn drop(&mut self)
```

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

### `impl<'a, T, const CAP: usize> ExactSizeIterator for Drain<'a, T, CAP> where T: 'a,`

#### `len`

```rust
fn len(&self) -> usize
```

Returns the exact remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html#method.len)

#### `is_empty`

```rust
fn is_empty(&self) -> bool
```

*🔬This is a nightly-only experimental API. (exact_size_is_empty)*

Returns `true` if the iterator is empty. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html#method.is_empty)

### `impl<'a, T, const CAP: usize> Iterator for Drain<'a, T, CAP> where T: 'a,`

#### `Item`

```rust
type Item = T
```

The type of the elements being iterated over.

#### `next`

```rust
fn next(&mut self) -> Option<<Drain<'a, T, CAP> as Iterator>::Item>
```

Advances the iterator and returns the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)

#### `size_hint`

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Returns the bounds on the remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)

#### `next_chunk`

```rust
fn next_chunk<const N: usize>( &mut self, ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>> where Self: Sized,
```

*🔬This is a nightly-only experimental API. (iter_next_chunk)*

Advances the iterator and returns an array containing the next `N` values. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)

#### `count`

```rust
fn count(self) -> usize where Self: Sized,
```

Consumes the iterator, counting the number of iterations and returning it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)

#### `last`

```rust
fn last(self) -> Option<Self::Item> where Self: Sized,
```

Consumes the iterator, returning the last element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)

#### `advance_by`

```rust
fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>>
```

*🔬This is a nightly-only experimental API. (iter_advance_by)*

Advances the iterator by `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)

#### `nth`

```rust
fn nth(&mut self, n: usize) -> Option<Self::Item>
```

Returns the `n`th element of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)

#### `step_by`

```rust
fn step_by(self, step: usize) -> StepBy<Self> where Self: Sized,
```

Creates an iterator starting at the same point, but stepping by the given amount at each iteration. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)

#### `chain`

```rust
fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> where Self: Sized, U: IntoIterator<Item = Self::Item>,
```

Takes two iterators and creates a new iterator over both in sequence. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)

#### `zip`

```rust
fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> where Self: Sized, U: IntoIterator,
```

‘Zips up’ two iterators into a single iterator of pairs. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)

#### `intersperse`

```rust
fn intersperse(self, separator: Self::Item) -> Intersperse<Self> where Self: Sized, Self::Item: Clone,
```

*🔬This is a nightly-only experimental API. (iter_intersperse)*

Creates a new iterator which places a copy of `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)

#### `intersperse_with`

```rust
fn intersperse_with<G>(self, separator: G) -> IntersperseWith<Self, G> where Self: Sized, G: FnMut() -> Self::Item,
```

*🔬This is a nightly-only experimental API. (iter_intersperse)*

Creates a new iterator which places an item generated by `separator` between adjacent items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)

#### `map`

```rust
fn map<B, F>(self, f: F) -> Map<Self, F> where Self: Sized, F: FnMut(Self::Item) -> B,
```

Takes a closure and creates an iterator which calls that closure on each element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)

#### `for_each`

```rust
fn for_each<F>(self, f: F) where Self: Sized, F: FnMut(Self::Item),
```

Calls a closure on each element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)

#### `filter`

```rust
fn filter<P>(self, predicate: P) -> Filter<Self, P> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Creates an iterator which uses a closure to determine if an element should be yielded. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)

#### `filter_map`

```rust
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> where Self: Sized, F: FnMut(Self::Item) -> Option<B>,
```

Creates an iterator that both filters and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)

#### `enumerate`

```rust
fn enumerate(self) -> Enumerate<Self> where Self: Sized,
```

Creates an iterator which gives the current iteration count as well as the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)

#### `peekable`

```rust
fn peekable(self) -> Peekable<Self> where Self: Sized,
```

Creates an iterator which can use the [`peek`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek) and [`peek_mut`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek_mut) methods to look at the next element of the iterator without consuming it. See their documentation for more information. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)

#### `skip_while`

```rust
fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Creates an iterator that [`skip`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)s elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)

#### `take_while`

```rust
fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Creates an iterator that yields elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)

#### `map_while`

```rust
fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P> where Self: Sized, P: FnMut(Self::Item) -> Option<B>,
```

Creates an iterator that both yields elements based on a predicate and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)

#### `skip`

```rust
fn skip(self, n: usize) -> Skip<Self> where Self: Sized,
```

Creates an iterator that skips the first `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)

#### `take`

```rust
fn take(self, n: usize) -> Take<Self> where Self: Sized,
```

Creates an iterator that yields the first `n` elements, or fewer if the underlying iterator ends sooner. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)

#### `scan`

```rust
fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F> where Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,
```

An iterator adapter which, like [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold), holds internal state, but unlike [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold), produces a new iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)

#### `flat_map`

```rust
fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F> where Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,
```

Creates an iterator that works like map, but flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)

#### `flatten`

```rust
fn flatten(self) -> Flatten<Self> where Self: Sized, Self::Item: IntoIterator,
```

Creates an iterator that flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten)

#### `map_windows`

```rust
fn map_windows<F, R, const N: usize>(self, f: F) -> MapWindows<Self, F, N> where Self: Sized, F: FnMut(&[Self::Item; N]) -> R,
```

*🔬This is a nightly-only experimental API. (iter_map_windows)*

Calls the given function `f` for each contiguous window of size `N` over `self` and returns an iterator over the outputs of `f`. Like [`slice::windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows), the windows during mapping overlap as well. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)

#### `fuse`

```rust
fn fuse(self) -> Fuse<Self> where Self: Sized,
```

Creates an iterator which ends after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None). [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)

#### `inspect`

```rust
fn inspect<F>(self, f: F) -> Inspect<Self, F> where Self: Sized, F: FnMut(&Self::Item),
```

Does something with each element of an iterator, passing the value on. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)

#### `by_ref`

```rust
fn by_ref(&mut self) -> &mut Self where Self: Sized,
```

Creates a “by reference” adapter for this instance of `Iterator`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)

#### `collect`

```rust
fn collect<B>(self) -> B where B: FromIterator<Self::Item>, Self: Sized,
```

Transforms an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)

#### `try_collect`

```rust
fn try_collect<B>( &mut self, ) -> <<Self::Item as Try>::Residual as Residual<B>>::TryType where Self: Sized, Self::Item: Try, <Self::Item as Try>::Residual: Residual<B>, B: FromIterator<<Self::Item as Try>::Output>,
```

*🔬This is a nightly-only experimental API. (iterator_try_collect)*

Fallibly transforms an iterator into a collection, short circuiting if a failure is encountered. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect)

#### `collect_into`

```rust
fn collect_into<E>(self, collection: &mut E) -> &mut E where E: Extend<Self::Item>, Self: Sized,
```

*🔬This is a nightly-only experimental API. (iter_collect_into)*

Collects all the items from an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)

#### `partition`

```rust
fn partition<B, F>(self, f: F) -> (B, B) where Self: Sized, B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool,
```

Consumes an iterator, creating two collections from it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)

#### `partition_in_place`

```rust
fn partition_in_place<'a, T, P>(self, predicate: P) -> usize where T: 'a, Self: Sized + DoubleEndedIterator<Item = &'a mut T>, P: FnMut(&T) -> bool,
```

*🔬This is a nightly-only experimental API. (iter_partition_in_place)*

Reorders the elements of this iterator *in-place* according to the given predicate, such that all those that return `true` precede all those that return `false`. Returns the number of `true` elements found. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition_in_place)

#### `is_partitioned`

```rust
fn is_partitioned<P>(self, predicate: P) -> bool where Self: Sized, P: FnMut(Self::Item) -> bool,
```

*🔬This is a nightly-only experimental API. (iter_is_partitioned)*

Checks if the elements of this iterator are partitioned according to the given predicate, such that all those that return `true` precede all those that return `false`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)

#### `try_fold`

```rust
fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R where Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Output = B>,
```

An iterator method that applies a function as long as it returns successfully, producing a single, final value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)

#### `try_for_each`

```rust
fn try_for_each<F, R>(&mut self, f: F) -> R where Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Output = ()>,
```

An iterator method that applies a fallible function to each item in the iterator, stopping at the first error and returning that error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)

#### `fold`

```rust
fn fold<B, F>(self, init: B, f: F) -> B where Self: Sized, F: FnMut(B, Self::Item) -> B,
```

Folds every element into an accumulator by applying an operation, returning the final result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)

#### `reduce`

```rust
fn reduce<F>(self, f: F) -> Option<Self::Item> where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Self::Item,
```

Reduces the elements to a single one, by repeatedly applying a reducing operation. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)

#### `try_reduce`

```rust
fn try_reduce<R>( &mut self, f: impl FnMut(Self::Item, Self::Item) -> R, ) -> <<R as Try>::Residual as Residual<Option<<R as Try>::Output>>>::TryType where Self: Sized, R: Try<Output = Self::Item>, <R as Try>::Residual: Residual<Option<Self::Item>>,
```

*🔬This is a nightly-only experimental API. (iterator_try_reduce)*

Reduces the elements to a single one by repeatedly applying a reducing operation. If the closure returns a failure, the failure is propagated back to the caller immediately. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)

#### `all`

```rust
fn all<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool,
```

Tests if every element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)

#### `any`

```rust
fn any<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool,
```

Tests if any element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)

#### `find`

```rust
fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Searches for an element of an iterator that satisfies a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)

#### `find_map`

```rust
fn find_map<B, F>(&mut self, f: F) -> Option<B> where Self: Sized, F: FnMut(Self::Item) -> Option<B>,
```

Applies function to the elements of iterator and returns the first non-none result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)

#### `try_find`

```rust
fn try_find<R>( &mut self, f: impl FnMut(&Self::Item) -> R, ) -> <<R as Try>::Residual as Residual<Option<Self::Item>>>::TryType where Self: Sized, R: Try<Output = bool>, <R as Try>::Residual: Residual<Option<Self::Item>>,
```

*🔬This is a nightly-only experimental API. (try_find)*

Applies function to the elements of iterator and returns the first true result or the first error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)

#### `position`

```rust
fn position<P>(&mut self, predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool,
```

Searches for an element in an iterator, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)

#### `rposition`

```rust
fn rposition<P>(&mut self, predicate: P) -> Option<usize> where P: FnMut(Self::Item) -> bool, Self: Sized + ExactSizeIterator + DoubleEndedIterator,
```

Searches for an element in an iterator from the right, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rposition)

#### `max`

```rust
fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord,
```

Returns the maximum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max)

#### `min`

```rust
fn min(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord,
```

Returns the minimum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min)

#### `max_by_key`

```rust
fn max_by_key<B, F>(self, f: F) -> Option<Self::Item> where B: Ord, Self: Sized, F: FnMut(&Self::Item) -> B,
```

Returns the element that gives the maximum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)

#### `max_by`

```rust
fn max_by<F>(self, compare: F) -> Option<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

Returns the element that gives the maximum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)

#### `min_by_key`

```rust
fn min_by_key<B, F>(self, f: F) -> Option<Self::Item> where B: Ord, Self: Sized, F: FnMut(&Self::Item) -> B,
```

Returns the element that gives the minimum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)

#### `min_by`

```rust
fn min_by<F>(self, compare: F) -> Option<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

Returns the element that gives the minimum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)

#### `rev`

```rust
fn rev(self) -> Rev<Self> where Self: Sized + DoubleEndedIterator,
```

Reverses an iterator’s direction. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rev)

#### `unzip`

```rust
fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB) where FromA: Default + Extend<A>, FromB: Default + Extend<B>, Self: Sized + Iterator<Item = (A, B)>,
```

Converts an iterator of pairs into a pair of containers. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)

#### `copied`

```rust
fn copied<'a, T>(self) -> Copied<Self> where T: Copy + 'a, Self: Sized + Iterator<Item = &'a T>,
```

Creates an iterator which copies all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)

#### `cloned`

```rust
fn cloned<'a, T>(self) -> Cloned<Self> where T: Clone + 'a, Self: Sized + Iterator<Item = &'a T>,
```

Creates an iterator which [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)s all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)

#### `cycle`

```rust
fn cycle(self) -> Cycle<Self> where Self: Sized + Clone,
```

Repeats an iterator endlessly. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)

#### `array_chunks`

```rust
fn array_chunks<const N: usize>(self) -> ArrayChunks<Self, N> where Self: Sized,
```

*🔬This is a nightly-only experimental API. (iter_array_chunks)*

Returns an iterator over `N` elements of the iterator at a time. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)

#### `sum`

```rust
fn sum<S>(self) -> S where Self: Sized, S: Sum<Self::Item>,
```

Sums the elements of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)

#### `product`

```rust
fn product<P>(self) -> P where Self: Sized, P: Product<Self::Item>,
```

Iterates over the entire iterator, multiplying all the elements [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)

#### `cmp`

```rust
fn cmp<I>(self, other: I) -> Ordering where I: IntoIterator<Item = Self::Item>, Self::Item: Ord, Self: Sized,
```

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) with those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp)

#### `cmp_by`

```rust
fn cmp_by<I, F>(self, other: I, cmp: F) -> Ordering where Self: Sized, I: IntoIterator, F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Ordering,
```

*🔬This is a nightly-only experimental API. (iter_order_by)*

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)

#### `partial_cmp`

```rust
fn partial_cmp<I>(self, other: I) -> Option<Ordering> where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized,
```

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) compares the [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html) elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) with those of another. The comparison works like short-circuit evaluation, returning a result without comparing the remaining elements. As soon as an order can be determined, the evaluation stops and a result is returned. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)

#### `partial_cmp_by`

```rust
fn partial_cmp_by<I, F>(self, other: I, partial_cmp: F) -> Option<Ordering> where Self: Sized, I: IntoIterator, F: FnMut(Self::Item, <I as IntoIterator>::Item) -> Option<Ordering>,
```

*🔬This is a nightly-only experimental API. (iter_order_by)*

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)

#### `eq`

```rust
fn eq<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<<I as IntoIterator>::Item>, Self: Sized,
```

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)

#### `eq_by`

```rust
fn eq_by<I, F>(self, other: I, eq: F) -> bool where Self: Sized, I: IntoIterator, F: FnMut(Self::Item, <I as IntoIterator>::Item) -> bool,
```

*🔬This is a nightly-only experimental API. (iter_order_by)*

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are equal to those of another with respect to the specified equality function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)

#### `ne`

```rust
fn ne<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<<I as IntoIterator>::Item>, Self: Sized,
```

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are not equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)

#### `lt`

```rust
fn lt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized,
```

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) less than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)

#### `le`

```rust
fn le<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized,
```

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) less or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)

#### `gt`

```rust
fn gt<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized,
```

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) greater than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)

#### `ge`

```rust
fn ge<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialOrd<<I as IntoIterator>::Item>, Self: Sized,
```

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html) are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison) greater than or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)

#### `is_sorted`

```rust
fn is_sorted(self) -> bool where Self: Sized, Self::Item: PartialOrd,
```

Checks if the elements of this iterator are sorted. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted)

#### `is_sorted_by`

```rust
fn is_sorted_by<F>(self, compare: F) -> bool where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> bool,
```

Checks if the elements of this iterator are sorted using the given comparator function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)

#### `is_sorted_by_key`

```rust
fn is_sorted_by_key<F, K>(self, f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> K, K: PartialOrd,
```

Checks if the elements of this iterator are sorted using the given key extraction function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)

### `impl<'a, T, const CAP: usize> Send for Drain<'a, T, CAP> where T: Send,`

### `impl<'a, T, const CAP: usize> Sync for Drain<'a, T, CAP> where T: Sync,`

## Auto Trait Implementations

- `impl<'a, T, const CAP: usize> Freeze for Drain<'a, T, CAP>`
- `impl<'a, T, const CAP: usize> RefUnwindSafe for Drain<'a, T, CAP> where T: RefUnwindSafe,`
- `impl<'a, T, const CAP: usize> Unpin for Drain<'a, T, CAP>`
- `impl<'a, T, const CAP: usize> UnsafeUnpin for Drain<'a, T, CAP>`
- `impl<'a, T, const CAP: usize> UnwindSafe for Drain<'a, T, CAP> where T: RefUnwindSafe,`
## Blanket Implementations

### `impl<T> Any for T where T: 'static + ?Sized,`

#### `type_id`

```rust
fn type_id(&self) -> TypeId
```

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

### `impl<T> Borrow<T> for T where T: ?Sized,`

#### `borrow`

```rust
fn borrow(&self) -> &T
```

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

### `impl<T> BorrowMut<T> for T where T: ?Sized,`

#### `borrow_mut`

```rust
fn borrow_mut(&mut self) -> &mut T
```

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

### `impl<T> Downcast<T> for T`

#### `downcast`

```rust
fn downcast(&self) -> &T
```

### `impl<T> Downcast for T where T: Any,`

#### `into_any`

```rust
fn into_any(self: Box<T>) -> Box<dyn Any>
```

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `into_any_rc`

```rust
fn into_any_rc(self: Rc<T>) -> Rc<dyn Any>
```

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

#### `as_any`

```rust
fn as_any(&self) -> &(dyn Any + 'static)
```

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

#### `as_any_mut`

```rust
fn as_any_mut(&mut self) -> &mut (dyn Any + 'static)
```

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

### `impl<T> DowncastSync for T where T: Any + Send + Sync,`

#### `into_any_arc`

```rust
fn into_any_arc(self: Arc<T>) -> Arc<dyn Any + Sync + Send>
```

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

### `impl<T> From<T> for T`

#### `from`

```rust
fn from(t: T) -> T
```

Returns the argument unchanged.

### `impl<T> Instrument for T`

#### `instrument`

```rust
fn instrument(self, span: Span) -> Instrumented<Self>
```

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

#### `in_current_span`

```rust
fn in_current_span(self) -> Instrumented<Self>
```

Instruments this type with the current `Span`, returning an `Instrumented` wrapper. Read more

### `impl<T, U> Into<U> for T where U: From<T>,`

#### `into`

```rust
fn into(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

### `impl<T> IntoEither for T`

#### `into_either`

```rust
fn into_either(self, into_left: bool) -> Either<Self, Self>
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)

#### `into_either_with`

```rust
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self> where F: FnOnce(&Self) -> bool,
```

Converts `self` into a [`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right) variant of [`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html) otherwise. [Read more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)

### `impl<I> IntoIterator for I where I: Iterator,`

#### `Item`

```rust
type Item = <I as Iterator>::Item
```

The type of the elements being iterated over.

#### `IntoIter`

```rust
type IntoIter = I
```

Which kind of iterator are we turning this into?

#### `into_iter`

```rust
fn into_iter(self) -> I
```

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

### `impl<T> Itertools for T where T: Iterator + ?Sized,`

#### `interleave`

```rust
fn interleave<J>( self, other: J, ) -> Interleave<Self, <J as IntoIterator>::IntoIter> where J: IntoIterator<Item = Self::Item>, Self: Sized,
```

Alternate elements from two iterators until both have run out. Read more

#### `interleave_shortest`

```rust
fn interleave_shortest<J>( self, other: J, ) -> InterleaveShortest<Self, <J as IntoIterator>::IntoIter> where J: IntoIterator<Item = Self::Item>, Self: Sized,
```

Alternate elements from two iterators until at least one of them has run out. Read more

#### `intersperse`

```rust
fn intersperse( self, element: Self::Item, ) -> IntersperseWith<Self, IntersperseElementSimple<Self::Item>> where Self: Sized, Self::Item: Clone,
```

An iterator adaptor to insert a particular value between each element of the adapted iterator. Read more

#### `intersperse_with`

```rust
fn intersperse_with<F>(self, element: F) -> IntersperseWith<Self, F> where Self: Sized, F: FnMut() -> Self::Item,
```

An iterator adaptor to insert a particular value created by a function between each element of the adapted iterator. Read more

#### `get`

```rust
fn get<R>(self, index: R) -> <R as IteratorIndex<Self>>::Output where Self: Sized, R: IteratorIndex<Self>,
```

Returns an iterator over a subsection of the iterator. Read more

#### `zip_longest`

```rust
fn zip_longest<J>( self, other: J, ) -> ZipLongest<Self, <J as IntoIterator>::IntoIter> where J: IntoIterator, Self: Sized,
```

Create an iterator which iterates over both this and the specified iterator simultaneously, yielding pairs of two optional elements. Read more

#### `zip_eq`

```rust
fn zip_eq<J>(self, other: J) -> ZipEq<Self, <J as IntoIterator>::IntoIter> where J: IntoIterator, Self: Sized,
```

Create an iterator which iterates over both this and the specified iterator simultaneously, yielding pairs of elements. Read more

#### `batching`

```rust
fn batching<B, F>(self, f: F) -> Batching<Self, F> where F: FnMut(&mut Self) -> Option<B>, Self: Sized,
```

A “meta iterator adaptor”. Its closure receives a reference to the iterator and may pick off as many elements as it likes, to produce the next iterator element. Read more

#### `chunk_by`

```rust
fn chunk_by<K, F>(self, key: F) -> ChunkBy<K, Self, F> where Self: Sized, F: FnMut(&Self::Item) -> K, K: PartialEq,
```

*Available on crate feature use_alloc only.*

Return an *iterable* that can group iterator elements. Consecutive elements that map to the same key (“runs”), are assigned to the same group. Read more

#### `group_by`

```rust
fn group_by<K, F>(self, key: F) -> ChunkBy<K, Self, F> where Self: Sized, F: FnMut(&Self::Item) -> K, K: PartialEq,
```

*👎Deprecated since 0.13.0: Use .chunk_by() insteadAvailable on crate feature use_alloc only.*

See `.chunk_by()`.

#### `chunks`

```rust
fn chunks(self, size: usize) -> IntoChunks<Self> where Self: Sized,
```

*Available on crate feature use_alloc only.*

Return an *iterable* that can chunk the iterator. Read more

#### `tuple_windows`

```rust
fn tuple_windows<T>(self) -> TupleWindows<Self, T> where Self: Sized + Iterator<Item = <T as TupleCollect>::Item>, T: HomogeneousTuple, <T as TupleCollect>::Item: Clone,
```

Return an iterator over all contiguous windows producing tuples of a specific size (up to 12). Read more

#### `circular_tuple_windows`

```rust
fn circular_tuple_windows<T>(self) -> CircularTupleWindows<Self, T> where Self: Sized + Clone + Iterator<Item = <T as TupleCollect>::Item> + ExactSizeIterator, T: TupleCollect + Clone, <T as TupleCollect>::Item: Clone,
```

Return an iterator over all windows, wrapping back to the first elements when the window would otherwise exceed the length of the iterator, producing tuples of a specific size (up to 12). Read more

#### `tuples`

```rust
fn tuples<T>(self) -> Tuples<Self, T> where Self: Sized + Iterator<Item = <T as TupleCollect>::Item>, T: HomogeneousTuple,
```

Return an iterator that groups the items in tuples of a specific size (up to 12). Read more

#### `tee`

```rust
fn tee(self) -> (Tee<Self>, Tee<Self>) where Self: Sized, Self::Item: Clone,
```

*Available on crate feature use_alloc only.*

Split into an iterator pair that both yield all elements from the original iterator. Read more

#### `map_into`

```rust
fn map_into<R>(self) -> MapSpecialCase<Self, MapSpecialCaseFnInto<R>> where Self: Sized, Self::Item: Into<R>,
```

Convert each item of the iterator using the [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html) trait. Read more

#### `map_ok`

```rust
fn map_ok<F, T, U, E>(self, f: F) -> MapSpecialCase<Self, MapSpecialCaseFnOk<F>> where Self: Sized + Iterator<Item = Result<T, E>>, F: FnMut(T) -> U,
```

Return an iterator adaptor that applies the provided closure to every `Result::Ok` value. `Result::Err` values are unchanged. Read more

#### `filter_ok`

```rust
fn filter_ok<F, T, E>(self, f: F) -> FilterOk<Self, F> where Self: Sized + Iterator<Item = Result<T, E>>, F: FnMut(&T) -> bool,
```

Return an iterator adaptor that filters every `Result::Ok` value with the provided closure. `Result::Err` values are unchanged. Read more

#### `filter_map_ok`

```rust
fn filter_map_ok<F, T, U, E>(self, f: F) -> FilterMapOk<Self, F> where Self: Sized + Iterator<Item = Result<T, E>>, F: FnMut(T) -> Option<U>,
```

Return an iterator adaptor that filters and transforms every `Result::Ok` value with the provided closure. `Result::Err` values are unchanged. Read more

#### `flatten_ok`

```rust
fn flatten_ok<T, E>(self) -> FlattenOk<Self, T, E> where Self: Sized + Iterator<Item = Result<T, E>>, T: IntoIterator,
```

Return an iterator adaptor that flattens every `Result::Ok` value into a series of `Result::Ok` values. `Result::Err` values are unchanged. Read more

#### `process_results`

```rust
fn process_results<F, T, E, R>(self, processor: F) -> Result<R, E> where Self: Sized + Iterator<Item = Result<T, E>>, F: FnOnce(ProcessResults<'_, Self, E>) -> R,
```

“Lift” a function of the values of the current iterator so as to process an iterator of `Result` values instead. Read more

#### `merge`

```rust
fn merge<J>( self, other: J, ) -> MergeBy<Self, <J as IntoIterator>::IntoIter, MergeLte> where Self: Sized, Self::Item: PartialOrd, J: IntoIterator<Item = Self::Item>,
```

Return an iterator adaptor that merges the two base iterators in ascending order. If both base iterators are sorted (ascending), the result is sorted. Read more

#### `merge_by`

```rust
fn merge_by<J, F>( self, other: J, is_first: F, ) -> MergeBy<Self, <J as IntoIterator>::IntoIter, F> where Self: Sized, J: IntoIterator<Item = Self::Item>, F: FnMut(&Self::Item, &Self::Item) -> bool,
```

Return an iterator adaptor that merges the two base iterators in order. This is much like `.merge()` but allows for a custom ordering. Read more

#### `merge_join_by`

```rust
fn merge_join_by<J, F, T>( self, other: J, cmp_fn: F, ) -> MergeBy<Self, <J as IntoIterator>::IntoIter, MergeFuncLR<F, <F as FuncLR<Self::Item, <<J as IntoIterator>::IntoIter as Iterator>::Item>>::T>> where J: IntoIterator, F: FnMut(&Self::Item, &<J as IntoIterator>::Item) -> T, Self: Sized,
```

Create an iterator that merges items from both this and the specified iterator in ascending order. Read more

#### `kmerge`

```rust
fn kmerge(self) -> KMergeBy<<Self::Item as IntoIterator>::IntoIter, KMergeByLt> where Self: Sized, Self::Item: IntoIterator, <Self::Item as IntoIterator>::Item: PartialOrd,
```

*Available on crate feature use_alloc only.*

Return an iterator adaptor that flattens an iterator of iterators by merging them in ascending order. Read more

#### `kmerge_by`

```rust
fn kmerge_by<F>( self, first: F, ) -> KMergeBy<<Self::Item as IntoIterator>::IntoIter, F> where Self: Sized, Self::Item: IntoIterator, F: FnMut(&<Self::Item as IntoIterator>::Item, &<Self::Item as IntoIterator>::Item) -> bool,
```

*Available on crate feature use_alloc only.*

Return an iterator adaptor that flattens an iterator of iterators by merging them according to the given closure. Read more

#### `cartesian_product`

```rust
fn cartesian_product<J>( self, other: J, ) -> Product<Self, <J as IntoIterator>::IntoIter> where Self: Sized, Self::Item: Clone, J: IntoIterator, <J as IntoIterator>::IntoIter: Clone,
```

Return an iterator adaptor that iterates over the cartesian product of the element sets of two iterators `self` and `J`. Read more

#### `multi_cartesian_product`

```rust
fn multi_cartesian_product( self, ) -> MultiProduct<<Self::Item as IntoIterator>::IntoIter> where Self: Sized, Self::Item: IntoIterator, <Self::Item as IntoIterator>::IntoIter: Clone, <Self::Item as IntoIterator>::Item: Clone,
```

*Available on crate feature use_alloc only.*

Return an iterator adaptor that iterates over the cartesian product of all subiterators returned by meta-iterator `self`. Read more

#### `coalesce`

```rust
fn coalesce<F>(self, f: F) -> CoalesceBy<Self, F, NoCount> where Self: Sized, F: FnMut(Self::Item, Self::Item) -> Result<Self::Item, (Self::Item, Self::Item)>,
```

Return an iterator adaptor that uses the passed-in closure to optionally merge together consecutive elements. Read more

#### `dedup`

```rust
fn dedup(self) -> CoalesceBy<Self, DedupPred2CoalescePred<DedupEq>, NoCount> where Self: Sized, Self::Item: PartialEq,
```

Remove duplicates from sections of consecutive identical elements. If the iterator is sorted, all elements will be unique. Read more

#### `dedup_by`

```rust
fn dedup_by<Cmp>( self, cmp: Cmp, ) -> CoalesceBy<Self, DedupPred2CoalescePred<Cmp>, NoCount> where Self: Sized, Cmp: FnMut(&Self::Item, &Self::Item) -> bool,
```

Remove duplicates from sections of consecutive identical elements, determining equality using a comparison function. If the iterator is sorted, all elements will be unique. Read more

#### `dedup_with_count`

```rust
fn dedup_with_count( self, ) -> CoalesceBy<Self, DedupPredWithCount2CoalescePred<DedupEq>, WithCount> where Self: Sized,
```

Remove duplicates from sections of consecutive identical elements, while keeping a count of how many repeated elements were present. If the iterator is sorted, all elements will be unique. Read more

#### `dedup_by_with_count`

```rust
fn dedup_by_with_count<Cmp>( self, cmp: Cmp, ) -> CoalesceBy<Self, DedupPredWithCount2CoalescePred<Cmp>, WithCount> where Self: Sized, Cmp: FnMut(&Self::Item, &Self::Item) -> bool,
```

Remove duplicates from sections of consecutive identical elements, while keeping a count of how many repeated elements were present. This will determine equality using a comparison function. If the iterator is sorted, all elements will be unique. Read more

#### `duplicates`

```rust
fn duplicates(self) -> DuplicatesBy<Self, Self::Item, ById> where Self: Sized, Self::Item: Eq + Hash,
```

*Available on crate feature use_std only.*

Return an iterator adaptor that produces elements that appear more than once during the iteration. Duplicates are detected using hash and equality. Read more

#### `duplicates_by`

```rust
fn duplicates_by<V, F>(self, f: F) -> DuplicatesBy<Self, V, ByFn<F>> where Self: Sized, V: Eq + Hash, F: FnMut(&Self::Item) -> V,
```

*Available on crate feature use_std only.*

Return an iterator adaptor that produces elements that appear more than once during the iteration. Duplicates are detected using hash and equality. Read more

#### `unique`

```rust
fn unique(self) -> Unique<Self> where Self: Sized, Self::Item: Clone + Eq + Hash,
```

*Available on crate feature use_std only.*

Return an iterator adaptor that filters out elements that have already been produced once during the iteration. Duplicates are detected using hash and equality. Read more

#### `unique_by`

```rust
fn unique_by<V, F>(self, f: F) -> UniqueBy<Self, V, F> where Self: Sized, V: Eq + Hash, F: FnMut(&Self::Item) -> V,
```

*Available on crate feature use_std only.*

Return an iterator adaptor that filters out elements that have already been produced once during the iteration. Read more

#### `peeking_take_while`

```rust
fn peeking_take_while<F>(&mut self, accept: F) -> PeekingTakeWhile<'_, Self, F> where Self: Sized + PeekingNext, F: FnMut(&Self::Item) -> bool,
```

Return an iterator adaptor that borrows from this iterator and takes items while the closure `accept` returns `true`. Read more

#### `take_while_ref`

```rust
fn take_while_ref<F>(&mut self, accept: F) -> TakeWhileRef<'_, Self, F> where Self: Clone, F: FnMut(&Self::Item) -> bool,
```

Return an iterator adaptor that borrows from a `Clone`-able iterator to only pick off elements while the predicate `accept` returns `true`. Read more

#### `take_while_inclusive`

```rust
fn take_while_inclusive<F>(self, accept: F) -> TakeWhileInclusive<Self, F> where Self: Sized, F: FnMut(&Self::Item) -> bool,
```

Returns an iterator adaptor that consumes elements while the given predicate is `true`, *including* the element for which the predicate first returned `false`. Read more

#### `while_some`

```rust
fn while_some<A>(self) -> WhileSome<Self> where Self: Sized + Iterator<Item = Option<A>>,
```

Return an iterator adaptor that filters `Option<A>` iterator elements and produces `A`. Stops on the first `None` encountered. Read more

#### `tuple_combinations`

```rust
fn tuple_combinations<T>(self) -> TupleCombinations<Self, T> where Self: Sized + Clone, Self::Item: Clone, T: HasCombination<Self>,
```

Return an iterator adaptor that iterates over the combinations of the elements from an iterator. Read more

#### `array_combinations`

```rust
fn array_combinations<const K: usize>( self, ) -> CombinationsGeneric<Self, [usize; K]> where Self: Sized + Clone, Self::Item: Clone,
```

*Available on crate feature use_alloc only.*

Return an iterator adaptor that iterates over the combinations of the elements from an iterator. Read more

#### `combinations`

```rust
fn combinations(self, k: usize) -> CombinationsGeneric<Self, Vec<usize>> where Self: Sized, Self::Item: Clone,
```

*Available on crate feature use_alloc only.*

Return an iterator adaptor that iterates over the `k`-length combinations of the elements from an iterator. Read more

#### `combinations_with_replacement`

```rust
fn combinations_with_replacement( self, k: usize, ) -> CombinationsWithReplacement<Self> where Self: Sized, Self::Item: Clone,
```

*Available on crate feature use_alloc only.*

Return an iterator that iterates over the `k`-length combinations of the elements from an iterator, with replacement. Read more

#### `permutations`

```rust
fn permutations(self, k: usize) -> Permutations<Self> where Self: Sized, Self::Item: Clone,
```

*Available on crate feature use_alloc only.*

Return an iterator adaptor that iterates over all k-permutations of the elements from an iterator. Read more

#### `powerset`

```rust
fn powerset(self) -> Powerset<Self> where Self: Sized, Self::Item: Clone,
```

*Available on crate feature use_alloc only.*

Return an iterator that iterates through the powerset of the elements from an iterator. Read more

#### `pad_using`

```rust
fn pad_using<F>(self, min: usize, f: F) -> PadUsing<Self, F> where Self: Sized, F: FnMut(usize) -> Self::Item,
```

Return an iterator adaptor that pads the sequence to a minimum length of `min` by filling missing elements using a closure `f`. Read more

#### `with_position`

```rust
fn with_position(self) -> WithPosition<Self> where Self: Sized,
```

Return an iterator adaptor that combines each element with a `Position` to ease special-case handling of the first or last elements. Read more

#### `positions`

```rust
fn positions<P>(self, predicate: P) -> Positions<Self, P> where Self: Sized, P: FnMut(Self::Item) -> bool,
```

Return an iterator adaptor that yields the indices of all elements satisfying a predicate, counted from the start of the iterator. Read more

#### `update`

```rust
fn update<F>(self, updater: F) -> Update<Self, F> where Self: Sized, F: FnMut(&mut Self::Item),
```

Return an iterator adaptor that applies a mutating function to each element before yielding it. Read more

#### `next_array`

```rust
fn next_array<const N: usize>(&mut self) -> Option<[Self::Item; N]> where Self: Sized,
```

Advances the iterator and returns the next items grouped in an array of a specific size. Read more

#### `collect_array`

```rust
fn collect_array<const N: usize>(self) -> Option<[Self::Item; N]> where Self: Sized,
```

Collects all items from the iterator into an array of a specific size. Read more

#### `next_tuple`

```rust
fn next_tuple<T>(&mut self) -> Option<T> where Self: Sized + Iterator<Item = <T as TupleCollect>::Item>, T: HomogeneousTuple,
```

Advances the iterator and returns the next items grouped in a tuple of a specific size (up to 12). Read more

#### `collect_tuple`

```rust
fn collect_tuple<T>(self) -> Option<T> where Self: Sized + Iterator<Item = <T as TupleCollect>::Item>, T: HomogeneousTuple,
```

Collects all items from the iterator into a tuple of a specific size (up to 12). Read more

#### `find_position`

```rust
fn find_position<P>(&mut self, pred: P) -> Option<(usize, Self::Item)> where P: FnMut(&Self::Item) -> bool,
```

Find the position and value of the first element satisfying a predicate. Read more

#### `find_or_last`

```rust
fn find_or_last<P>(self, predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Find the value of the first element satisfying a predicate or return the last element, if any. Read more

#### `find_or_first`

```rust
fn find_or_first<P>(self, predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool,
```

Find the value of the first element satisfying a predicate or return the first element, if any. Read more

#### `contains`

```rust
fn contains<Q>(&mut self, query: &Q) -> bool where Self: Sized, Self::Item: Borrow<Q>, Q: PartialEq + ?Sized,
```

Returns `true` if the given item is present in this iterator. Read more

#### `all_equal`

```rust
fn all_equal(&mut self) -> bool where Self: Sized, Self::Item: PartialEq,
```

Check whether all elements compare equal. Read more

#### `all_equal_value`

```rust
fn all_equal_value( &mut self, ) -> Result<Self::Item, Option<(Self::Item, Self::Item)>> where Self: Sized, Self::Item: PartialEq,
```

If there are elements and they are all equal, return a single copy of that element. If there are no elements, return an Error containing None. If there are elements and they are not all equal, return a tuple containing the first two non-equal elements found. Read more

#### `all_unique`

```rust
fn all_unique(&mut self) -> bool where Self: Sized, Self::Item: Eq + Hash,
```

*Available on crate feature use_std only.*

Check whether all elements are unique (non equal). Read more

#### `dropping`

```rust
fn dropping(self, n: usize) -> Self where Self: Sized,
```

Consume the first `n` elements from the iterator eagerly, and return the same iterator again. Read more

#### `dropping_back`

```rust
fn dropping_back(self, n: usize) -> Self where Self: Sized + DoubleEndedIterator,
```

Consume the last `n` elements from the iterator eagerly, and return the same iterator again. Read more

#### `concat`

```rust
fn concat(self) -> Self::Item where Self: Sized, Self::Item: Extend<<Self::Item as IntoIterator>::Item> + IntoIterator + Default,
```

Combine all an iterator’s elements into one element by using [`Extend`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html). Read more

#### `collect_vec`

```rust
fn collect_vec(self) -> Vec<Self::Item> where Self: Sized,
```

*Available on crate feature use_alloc only.*

`.collect_vec()` is simply a type specialization of [`Iterator::collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect), for convenience.

#### `try_collect`

```rust
fn try_collect<T, U, E>(self) -> Result<U, E> where Self: Sized + Iterator<Item = Result<T, E>>, Result<U, E>: FromIterator<Result<T, E>>,
```

`.try_collect()` is more convenient way of writing `.collect::<Result<_, _>>()` Read more

#### `set_from`

```rust
fn set_from<'a, A, J>(&mut self, from: J) -> usize where A: 'a, Self: Iterator<Item = &'a mut A>, J: IntoIterator<Item = A>,
```

Assign to each reference in `self` from the `from` iterator, stopping at the shortest of the two iterators. Read more

#### `join`

```rust
fn join(&mut self, sep: &str) -> String where Self::Item: Display,
```

*Available on crate feature use_alloc only.*

Combine all iterator elements into one `String`, separated by `sep`. Read more

#### `format`

```rust
fn format(self, sep: &str) -> Format<'_, Self> where Self: Sized,
```

Format all iterator elements, separated by `sep`. Read more

#### `format_with`

```rust
fn format_with<F>(self, sep: &str, format: F) -> FormatWith<'_, Self, F> where Self: Sized, F: FnMut(Self::Item, &mut dyn FnMut(&dyn Display) -> Result<(), Error>) -> Result<(), Error>,
```

Format all iterator elements, separated by `sep`. Read more

#### `fold_ok`

```rust
fn fold_ok<A, E, B, F>(&mut self, start: B, f: F) -> Result<B, E> where Self: Iterator<Item = Result<A, E>>, F: FnMut(B, A) -> B,
```

Fold `Result` values from an iterator. Read more

#### `fold_options`

```rust
fn fold_options<A, B, F>(&mut self, start: B, f: F) -> Option<B> where Self: Iterator<Item = Option<A>>, F: FnMut(B, A) -> B,
```

Fold `Option` values from an iterator. Read more

#### `fold1`

```rust
fn fold1<F>(self, f: F) -> Option<Self::Item> where F: FnMut(Self::Item, Self::Item) -> Self::Item, Self: Sized,
```

*👎Deprecated since 0.10.2: Use Iterator::reduce instead*

Accumulator of the elements in the iterator. Read more

#### `tree_reduce`

```rust
fn tree_reduce<F>(self, f: F) -> Option<Self::Item> where F: FnMut(Self::Item, Self::Item) -> Self::Item, Self: Sized,
```

Accumulate the elements in the iterator in a tree-like manner. Read more

#### `tree_fold1`

```rust
fn tree_fold1<F>(self, f: F) -> Option<Self::Item> where F: FnMut(Self::Item, Self::Item) -> Self::Item, Self: Sized,
```

*👎Deprecated since 0.13.0: Use .tree_reduce() instead*

See `.tree_reduce()`.

#### `fold_while`

```rust
fn fold_while<B, F>(&mut self, init: B, f: F) -> FoldWhile<B> where Self: Sized, F: FnMut(B, Self::Item) -> FoldWhile<B>,
```

An iterator method that applies a function, producing a single, final value. Read more

#### `sum1`

```rust
fn sum1<S>(self) -> Option<S> where Self: Sized, S: Sum<Self::Item>,
```

Iterate over the entire iterator and add all the elements. Read more

#### `product1`

```rust
fn product1<P>(self) -> Option<P> where Self: Sized, P: Product<Self::Item>,
```

Iterate over the entire iterator and multiply all the elements. Read more

#### `sorted_unstable`

```rust
fn sorted_unstable(self) -> IntoIter<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. Read more

#### `sorted_unstable_by`

```rust
fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. Read more

#### `sorted_unstable_by_key`

```rust
fn sorted_unstable_by_key<K, F>(self, f: F) -> IntoIter<Self::Item> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. Read more

#### `sorted`

```rust
fn sorted(self) -> IntoIter<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. Read more

#### `sorted_by`

```rust
fn sorted_by<F>(self, cmp: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. Read more

#### `sorted_by_key`

```rust
fn sorted_by_key<K, F>(self, f: F) -> IntoIter<Self::Item> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. Read more

#### `sorted_by_cached_key`

```rust
fn sorted_by_cached_key<K, F>(self, f: F) -> IntoIter<Self::Item> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

*Available on crate feature use_alloc only.*

Sort all iterator elements into a new iterator in ascending order. The key function is called exactly once per key. Read more

#### `k_smallest`

```rust
fn k_smallest(self, k: usize) -> IntoIter<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Sort the k smallest elements into a new iterator, in ascending order. Read more

#### `k_smallest_by`

```rust
fn k_smallest_by<F>(self, k: usize, cmp: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Sort the k smallest elements into a new iterator using the provided comparison. Read more

#### `k_smallest_by_key`

```rust
fn k_smallest_by_key<F, K>(self, k: usize, key: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item) -> K, K: Ord,
```

*Available on crate feature use_alloc only.*

Return the elements producing the k smallest outputs of the provided function. Read more

#### `k_smallest_relaxed`

```rust
fn k_smallest_relaxed(self, k: usize) -> IntoIter<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Sort the k smallest elements into a new iterator, in ascending order, relaxing the amount of memory required. Read more

#### `k_smallest_relaxed_by`

```rust
fn k_smallest_relaxed_by<F>(self, k: usize, cmp: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Sort the k smallest elements into a new iterator using the provided comparison, relaxing the amount of memory required. Read more

#### `k_smallest_relaxed_by_key`

```rust
fn k_smallest_relaxed_by_key<F, K>( self, k: usize, key: F, ) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item) -> K, K: Ord,
```

*Available on crate feature use_alloc only.*

Return the elements producing the k smallest outputs of the provided function, relaxing the amount of memory required. Read more

#### `k_largest`

```rust
fn k_largest(self, k: usize) -> IntoIter<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Sort the k largest elements into a new iterator, in descending order. Read more

#### `k_largest_by`

```rust
fn k_largest_by<F>(self, k: usize, cmp: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Sort the k largest elements into a new iterator using the provided comparison. Read more

#### `k_largest_by_key`

```rust
fn k_largest_by_key<F, K>(self, k: usize, key: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item) -> K, K: Ord,
```

*Available on crate feature use_alloc only.*

Return the elements producing the k largest outputs of the provided function. Read more

#### `k_largest_relaxed`

```rust
fn k_largest_relaxed(self, k: usize) -> IntoIter<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Sort the k largest elements into a new iterator, in descending order, relaxing the amount of memory required. Read more

#### `k_largest_relaxed_by`

```rust
fn k_largest_relaxed_by<F>(self, k: usize, cmp: F) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Sort the k largest elements into a new iterator using the provided comparison, relaxing the amount of memory required. Read more

#### `k_largest_relaxed_by_key`

```rust
fn k_largest_relaxed_by_key<F, K>( self, k: usize, key: F, ) -> IntoIter<Self::Item> where Self: Sized, F: FnMut(&Self::Item) -> K, K: Ord,
```

*Available on crate feature use_alloc only.*

Return the elements producing the k largest outputs of the provided function, relaxing the amount of memory required. Read more

#### `tail`

```rust
fn tail(self, n: usize) -> IntoIter<Self::Item> where Self: Sized,
```

*Available on crate feature use_alloc only.*

Consumes the iterator and return an iterator of the last `n` elements. Read more

#### `partition_map`

```rust
fn partition_map<A, B, F, L, R>(self, predicate: F) -> (A, B) where Self: Sized, F: FnMut(Self::Item) -> Either<L, R>, A: Default + Extend<L>, B: Default + Extend<R>,
```

Collect all iterator elements into one of two partitions. Unlike [`Iterator::partition`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition), each partition may have a distinct type. Read more

#### `partition_result`

```rust
fn partition_result<A, B, T, E>(self) -> (A, B) where Self: Sized + Iterator<Item = Result<T, E>>, A: Default + Extend<T>, B: Default + Extend<E>,
```

Partition a sequence of `Result`s into one list of all the `Ok` elements and another list of all the `Err` elements. Read more

#### `into_group_map`

```rust
fn into_group_map<K, V>(self) -> HashMap<K, Vec<V>> where Self: Sized + Iterator<Item = (K, V)>, K: Hash + Eq,
```

*Available on crate feature use_std only.*

Return a `HashMap` of keys mapped to `Vec`s of values. Keys and values are taken from `(Key, Value)` tuple pairs yielded by the input iterator. Read more

#### `into_group_map_by`

```rust
fn into_group_map_by<K, V, F>(self, f: F) -> HashMap<K, Vec<V>> where Self: Sized + Iterator<Item = V>, K: Hash + Eq, F: FnMut(&V) -> K,
```

*Available on crate feature use_std only.*

Return a `HashMap` of keys mapped to `Vec`s of values. The key is specified in the closure. The values are taken from the input iterator. Read more

#### `into_grouping_map`

```rust
fn into_grouping_map<K, V>(self) -> GroupingMap<Self> where Self: Sized + Iterator<Item = (K, V)>, K: Hash + Eq,
```

*Available on crate feature use_std only.*

Constructs a `GroupingMap` to be used later with one of the efficient group-and-fold operations it allows to perform. Read more

#### `into_grouping_map_by`

```rust
fn into_grouping_map_by<K, V, F>( self, key_mapper: F, ) -> GroupingMap<MapSpecialCase<Self, GroupingMapFn<F>>> where Self: Sized + Iterator<Item = V>, K: Hash + Eq, F: FnMut(&V) -> K,
```

*Available on crate feature use_std only.*

Constructs a `GroupingMap` to be used later with one of the efficient group-and-fold operations it allows to perform. Read more

#### `min_set`

```rust
fn min_set(self) -> Vec<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Return all minimum elements of an iterator. Read more

#### `min_set_by`

```rust
fn min_set_by<F>(self, compare: F) -> Vec<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Return all minimum elements of an iterator, as determined by the specified function. Read more

#### `min_set_by_key`

```rust
fn min_set_by_key<K, F>(self, key: F) -> Vec<Self::Item> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

*Available on crate feature use_alloc only.*

Return all minimum elements of an iterator, as determined by the specified function. Read more

#### `max_set`

```rust
fn max_set(self) -> Vec<Self::Item> where Self: Sized, Self::Item: Ord,
```

*Available on crate feature use_alloc only.*

Return all maximum elements of an iterator. Read more

#### `max_set_by`

```rust
fn max_set_by<F>(self, compare: F) -> Vec<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

*Available on crate feature use_alloc only.*

Return all maximum elements of an iterator, as determined by the specified function. Read more

#### `max_set_by_key`

```rust
fn max_set_by_key<K, F>(self, key: F) -> Vec<Self::Item> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

*Available on crate feature use_alloc only.*

Return all maximum elements of an iterator, as determined by the specified function. Read more

#### `minmax`

```rust
fn minmax(self) -> MinMaxResult<Self::Item> where Self: Sized, Self::Item: PartialOrd,
```

Return the minimum and maximum elements in the iterator. Read more

#### `minmax_by_key`

```rust
fn minmax_by_key<K, F>(self, key: F) -> MinMaxResult<Self::Item> where Self: Sized, K: PartialOrd, F: FnMut(&Self::Item) -> K,
```

Return the minimum and maximum element of an iterator, as determined by the specified function. Read more

#### `minmax_by`

```rust
fn minmax_by<F>(self, compare: F) -> MinMaxResult<Self::Item> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

Return the minimum and maximum element of an iterator, as determined by the specified comparison function. Read more

#### `position_max`

```rust
fn position_max(self) -> Option<usize> where Self: Sized, Self::Item: Ord,
```

Return the position of the maximum element in the iterator. Read more

#### `position_max_by_key`

```rust
fn position_max_by_key<K, F>(self, key: F) -> Option<usize> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

Return the position of the maximum element in the iterator, as determined by the specified function. Read more

#### `position_max_by`

```rust
fn position_max_by<F>(self, compare: F) -> Option<usize> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

Return the position of the maximum element in the iterator, as determined by the specified comparison function. Read more

#### `position_min`

```rust
fn position_min(self) -> Option<usize> where Self: Sized, Self::Item: Ord,
```

Return the position of the minimum element in the iterator. Read more

#### `position_min_by_key`

```rust
fn position_min_by_key<K, F>(self, key: F) -> Option<usize> where Self: Sized, K: Ord, F: FnMut(&Self::Item) -> K,
```

Return the position of the minimum element in the iterator, as determined by the specified function. Read more

#### `position_min_by`

```rust
fn position_min_by<F>(self, compare: F) -> Option<usize> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

Return the position of the minimum element in the iterator, as determined by the specified comparison function. Read more

#### `position_minmax`

```rust
fn position_minmax(self) -> MinMaxResult<usize> where Self: Sized, Self::Item: PartialOrd,
```

Return the positions of the minimum and maximum elements in the iterator. Read more

#### `position_minmax_by_key`

```rust
fn position_minmax_by_key<K, F>(self, key: F) -> MinMaxResult<usize> where Self: Sized, K: PartialOrd, F: FnMut(&Self::Item) -> K,
```

Return the postions of the minimum and maximum elements of an iterator, as determined by the specified function. Read more

#### `position_minmax_by`

```rust
fn position_minmax_by<F>(self, compare: F) -> MinMaxResult<usize> where Self: Sized, F: FnMut(&Self::Item, &Self::Item) -> Ordering,
```

Return the postions of the minimum and maximum elements of an iterator, as determined by the specified comparison function. Read more

#### `exactly_one`

```rust
fn exactly_one(self) -> Result<Self::Item, ExactlyOneError<Self>> where Self: Sized,
```

If the iterator yields exactly one element, that element will be returned, otherwise an error will be returned containing an iterator that has the same output as the input iterator. Read more

#### `at_most_one`

```rust
fn at_most_one(self) -> Result<Option<Self::Item>, ExactlyOneError<Self>> where Self: Sized,
```

If the iterator yields no elements, `Ok(None)` will be returned. If the iterator yields exactly one element, that element will be returned, otherwise an error will be returned containing an iterator that has the same output as the input iterator. Read more

#### `multipeek`

```rust
fn multipeek(self) -> MultiPeek<Self> where Self: Sized,
```

*Available on crate feature use_alloc only.*

An iterator adaptor that allows the user to peek at multiple `.next()` values without advancing the base iterator. Read more

#### `counts`

```rust
fn counts(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Eq + Hash,
```

*Available on crate feature use_std only.*

Collect the items in this iterator and return a `HashMap` which contains each item that appears in the iterator and the number of times it appears. Read more

#### `counts_by`

```rust
fn counts_by<K, F>(self, f: F) -> HashMap<K, usize> where Self: Sized, K: Eq + Hash, F: FnMut(Self::Item) -> K,
```

*Available on crate feature use_std only.*

Collect the items in this iterator and return a `HashMap` which contains each item that appears in the iterator and the number of times it appears, determining identity using a keying function. Read more

#### `multiunzip`

```rust
fn multiunzip<FromI>(self) -> FromI where Self: Sized + MultiUnzip<FromI>,
```

Converts an iterator of tuples into a tuple of containers. Read more

#### `try_len`

```rust
fn try_len(&self) -> Result<usize, (usize, Option<usize>)>
```

Returns the length of the iterator if one exists. Otherwise return `self.size_hint()`. Read more

### `impl<IT> MultiUnzip<()> for IT where IT: Iterator<Item = ()>,`

#### `multiunzip`

```rust
fn multiunzip(self)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA> MultiUnzip<(FromA,)> for IT where IT: Iterator<Item = (A,)>, FromA: Default + Extend<A>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA,)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB> MultiUnzip<(FromA, FromB)> for IT where IT: Iterator<Item = (A, B)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC> MultiUnzip<(FromA, FromB, FromC)> for IT where IT: Iterator<Item = (A, B, C)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB, FromC)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD> MultiUnzip<(FromA, FromB, FromC, FromD)> for IT where IT: Iterator<Item = (A, B, C, D)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB, FromC, FromD)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE> MultiUnzip<(FromA, FromB, FromC, FromD, FromE)> for IT where IT: Iterator<Item = (A, B, C, D, E)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF)> for IT where IT: Iterator<Item = (A, B, C, D, E, F)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF, G, FromG> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF, FromG)> for IT where IT: Iterator<Item = (A, B, C, D, E, F, G)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>, FromG: Default + Extend<G>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF, G, FromG, H, FromH> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH)> for IT where IT: Iterator<Item = (A, B, C, D, E, F, G, H)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>, FromG: Default + Extend<G>, FromH: Default + Extend<H>,`

#### `multiunzip`

```rust
fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF, G, FromG, H, FromH, I, FromI> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI)> for IT where IT: Iterator<Item = (A, B, C, D, E, F, G, H, I)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>, FromG: Default + Extend<G>, FromH: Default + Extend<H>, FromI: Default + Extend<I>,`

#### `multiunzip`

```rust
fn multiunzip( self, ) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF, G, FromG, H, FromH, I, FromI, J, FromJ> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ)> for IT where IT: Iterator<Item = (A, B, C, D, E, F, G, H, I, J)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>, FromG: Default + Extend<G>, FromH: Default + Extend<H>, FromI: Default + Extend<I>, FromJ: Default + Extend<J>,`

#### `multiunzip`

```rust
fn multiunzip( self, ) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF, G, FromG, H, FromH, I, FromI, J, FromJ, K, FromK> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ, FromK)> for IT where IT: Iterator<Item = (A, B, C, D, E, F, G, H, I, J, K)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>, FromG: Default + Extend<G>, FromH: Default + Extend<H>, FromI: Default + Extend<I>, FromJ: Default + Extend<J>, FromK: Default + Extend<K>,`

#### `multiunzip`

```rust
fn multiunzip( self, ) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ, FromK)
```

Unzip this iterator into multiple collections.

### `impl<IT, A, FromA, B, FromB, C, FromC, D, FromD, E, FromE, F, FromF, G, FromG, H, FromH, I, FromI, J, FromJ, K, FromK, L, FromL> MultiUnzip<(FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ, FromK, FromL)> for IT where IT: Iterator<Item = (A, B, C, D, E, F, G, H, I, J, K, L)>, FromA: Default + Extend<A>, FromB: Default + Extend<B>, FromC: Default + Extend<C>, FromD: Default + Extend<D>, FromE: Default + Extend<E>, FromF: Default + Extend<F>, FromG: Default + Extend<G>, FromH: Default + Extend<H>, FromI: Default + Extend<I>, FromJ: Default + Extend<J>, FromK: Default + Extend<K>, FromL: Default + Extend<L>,`

#### `multiunzip`

```rust
fn multiunzip( self, ) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ, FromK, FromL)
```

Unzip this iterator into multiple collections.

### `impl<Iter> PathIterator for Iter where Iter: Iterator<Item = Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>>,`

#### `flattened`

```rust
fn flattened(self, tolerance: f32) -> Flattened<Self> ⓘ
```

Returns an iterator that turns curves into line segments.

#### `transformed`

```rust
fn transformed<T>(self, mat: &T) -> Transformed<'_, Self, T> ⓘ where T: Transformation<f32>,
```

Returns an iterator applying a 2D transform to all of its events.

### `impl<T> Pointable for T`

#### `ALIGN`

```rust
const ALIGN: usize
```

The alignment of pointer.

#### `Init`

```rust
type Init = T
```

The type for initializers.

#### `init`

```rust
unsafe fn init(init: <T as Pointable>::Init) -> usize
```

Initializes a with the given initializer. Read more

#### `deref`

```rust
unsafe fn deref<'a>(ptr: usize) -> &'a T
```

Dereferences the given pointer. Read more

#### `deref_mut`

```rust
unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T
```

Mutably dereferences the given pointer. Read more

#### `drop`

```rust
unsafe fn drop(ptr: usize)
```

Drops the object pointed to by the given pointer. Read more

### `impl<T, U> TryFrom<U> for T where U: Into<T>,`

#### `Error`

```rust
type Error = Infallible
```

The type returned in the event of a conversion error.

#### `try_from`

```rust
fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error>
```

Performs the conversion.

### `impl<T, U> TryInto<U> for T where U: TryFrom<T>,`

#### `Error`

```rust
type Error = <U as TryFrom<T>>::Error
```

The type returned in the event of a conversion error.

#### `try_into`

```rust
fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>
```

Performs the conversion.

### `impl<T> Upcast<T> for T`

#### `upcast`

```rust
fn upcast(&self) -> Option<&T>
```

### `impl<T> WithSubscriber for T`

#### `with_subscriber`

```rust
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> where S: Into<Dispatch>,
```

Attaches the provided `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

#### `with_current_subscriber`

```rust
fn with_current_subscriber(self) -> WithDispatch<Self>
```

Attaches the current default `Subscriber` to this type, returning a [`WithDispatch`] wrapper. Read more

### `impl<ST, DT> CastableFrom<ST, Initialized, Initialized> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<ST, DT> CastableFrom<ST, Uninit, Uninit> for DT where ST: ?Sized, DT: ?Sized,`

### `impl<T> MaybeSend for T where T: Send,`

### `impl<T> MaybeSync for T where T: Sync,`

### `impl<T> Read<Exclusive, BecauseExclusive> for T where T: ?Sized,`

### `impl<T> WasmNotSend for T where T: Send,`

### `impl<T> WasmNotSendSync for T where T: WasmNotSend + WasmNotSync,`

### `impl<T> WasmNotSync for T where T: Sync,`
