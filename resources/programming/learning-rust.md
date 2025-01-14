# Learning Rust

Back to [table of contents.](/resources/programming/index.md)

---

## Raymond's Personal Pointers

Before 2024, I had never written a single line of Rust code. If your experience is anything like mine, you'll probably find it very frustrating. But once it clicks, you'll find that learning Rust is a very rewarding endeavor that takes less time than you might think.

### Mindset

Rust was designed for many purposes - fixing crashing elevator software was one of them. Writing languages like C and C++ can turn into a fiery mess if you're not constantly aware of where your data is in memory. Even if you're careful, one dereferenced null pointer, and the only thing half of corporate computers in America can do is frown `:(`.

Other languages like C# and Java use the heap, and autmatically garbage collect the memory. But even these languages aren't free from memory leaks: having a circular reference prevents the GC from actually freeing the memory. Not only that, the heap is a lot slower compared to the stack.

If you haven't written C or C++, in CS2110 for example, then you might find some of the core components of the Rust language confusing, and perhaps superflous. I personally don't think its "worth it" to learn a language like C before learning Rust, but just know that there is likely a reason for every "annoying" part of Rust, even if not immediately apparent.

### General Pointers

- Just get started on writing Rust. I strongly believe that the best way to learn a programming language is to just use it. Be prepared to struggle, but note that there are always documentation, examples, and other resources to fall back on.

- RTFM. Not only is Rust documentation itself really good, but documentaton for third party packages are the best in any language's ecosystem I have seen. Don't be afraid to dig deep to find the information that you need.

- When writing Rust, take a little more time to think how you should structure your system.
    - Rust can feel like a constant fight with the compiler, which can be especially frustrating when you can't even see your code run.
    - Take note of what the compiler tells you, and consider the effectiveness of your design.
        - Do you move a `Vec` to a function for it to be modified, only for it to be used later in your code? That extra function should borrow the `Vec` instead (more performant!)

- The easy path is likely the best path.
    - Do you think the only way to satisfy the compilers is to use custom lifetimes? Think again - this is likely not the case for a majority of use cases, and certainly not for the use cases we'll see on Lightborne

### Other Info

- Really dig deep on the concept of ownership and why it exists.
    - What happens if you mutably modify the same data from two different places? 
    - Why can't you mix mutable and immutable references to data? 
    - If some data doesn't have an "owner", then who is responsible for cleaning it up when it goes out of scope?

- Keep structs as small as possible. Data should only go together in the same struct if you're always going to use it at the same time.
    - You cannot borrow fields of the struct without borrowing the whole struct.
    - Bevy can only parallelize your systems if they access different components, so making them as small as reasonably possible is best.

- Consider index-based access into containers. Even though this may seen straightforward to some, indices are "references" to data in collections - you can keep a pointer to data without actually making one.
