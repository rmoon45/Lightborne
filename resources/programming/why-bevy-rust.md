# Why Rust, and Why Bevy?

Back to [table of contents.](/resources/programming/index.md)

---

Rust, and more specifically Bevy, offer some advantages and some disadvantages over other game engines, in particular the ones that support more direct OOP concepts. Here are some key points that might interest you in Rust/Bevy:

## Advantages

### Modular and Performant Code with No Extra Effort

- Bevy's ECS development is truly refreshing and unique compared to my (limited) experience with other Engines.

- An ECS will store and manage all of your data for you, and give you access to it when appropriate.
    - This allows you to decouple systems in code that might be logically related, leading to easier-to-debug systems and a more modular codebase.
    - Want a poison status effect? Just add a `Poison` component to an entity with a `Health` component, write a system that queries for entities with both `Health` and `Poison`, and apply the effect!

- When you define `systems` that execute your game logic, you *inherently also define which data it accesses*!
    - This gives Bevy **crazy** flexibility in terms of actually scheduling your systems - it will use this information to run your systems in parallel, on multiple CPU threads, without race conditions/manual mutex management!
    - Bevy is *probably* one of the only engines that can use all cores on a modern CPU.

### Safe by Default - Focus on the Logic

- A big advantage of Rust is that it is **safe by default**. You have to explicitly go out of your way to write code that will cause undefined behavior, dereference a null pointer, or introduce race conditions.
    - Who wants to focus on these issues when you have a cool ass game to make?

- Ergonomic error handling. There are no exceptions in Rust, which means it is very apparent when reading/writing code where something might fail. Instead of dereferencing a null pointer, check if the value of an `Option` is `None`.
    - Easier to debug potential errors in logic.

- Strong type system. I personally hate any language that allows you to treat a `float` as an `unsigned 2005 honda accord`. No thanks. 
    - A strong type system reduces errors stemming from what should be easy-to-catch mistakes (using a variable with wrong name, etc.)

### Wide Platform Support

- Large platform support. WGPU compiles to any backend: OpenGL, Vulkan, Metal, DX12. Once we finish our game, you'll be able to play it in a browser, natively on Windows/Mac/Linux, or even on your mobile device.

- Cargo is a beast of a package manager. It is very unlikely you'll have dependency hell issues, machine-dependent build errors, or mismatched system libraries to be included. Any dev on any operating system should be able to work on Lightborne.

## Disadvantages

- Rust is *different*. There are several programming concepts and patterns you might have to "rewire" your brain to learn. Check out my [personal experience with migrating to Rust](/resources/programming/learning-rust.md) for a summary.

- Rust is *verbose*. You might find yourself writing a lot of code and refactoring quite a bit to achieve a certain task. This is the tradeoff of making sure that the code you write is safe, fast, and well designed.

- There is no GUI. This makes the change-test-change-test process rather cumbersome.
    - We will be using **LDTK** for Lightborne to design levels for that very reason - designers won't have make changes to (or even compile) code to test their levels.

- No shadergraph. Unfortunately, WGSL does not have fun drag and drop components with previews that allow you to preview your changes as you go. 
    - However, if you do have a shadergraph implementation of what you want, you can translate it to WGSL without much effort.

- Compile times. Yeah, it takes a while to compile the project from start to finish. 
    - There are ways to avoid this, including linking Bevy dynamically, using a dynamic or alternative linker, or using the nightly rust compiler. 
    - Compilation is cached, so any subsequent compilations will do just fine.
