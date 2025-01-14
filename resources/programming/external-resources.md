# External Resources

Back to [table of contents.](/resources/programming/index.md)

---

## Rust

- **The Book**, if you want a solid understanding of everything Rust: https://doc.rust-lang.org/book/
    - Lots of information, recommended if you want to learn Rust outside of this project
    - Here's why you might want to read:  
        - Chapter 3, for basic syntax and structure
        - Chapter 4, to learn about Rust's concept of ownership (**important**).
        - Chapter 6 and 18, to learn about pattern matching (`match`, `if let`, `let else`)
        - Chapter 7, to catch yourself up on modules (think `import` in python)
        - Chapter 8, to learn the standard library data structures (`Vec`, `String`, `Hashmap`)
        - Chapter 9, to see how Rust approaches error handling with (`Option`, `Result`)
        - Chapter 17, to see how common OOP patterns are handled in Rust with (`Traits`)
    - Other chapters would be good for you to know, but it is unlikely that we will be using e.g. custom lifetimes in our project.

- **Rust By Example**, if you prefer seeing example code snippets: https://doc.rust-lang.org/rust-by-example/index.html

## Bevy

- The official **Bevy Quick Start Guide**: https://bevyengine.org/learn/quick-start/getting-started/

- The unofficial **Bevy Cheat Book**: https://bevy-cheatbook.github.io/
    - This is a great resource that goes more in depth on how to actually do things in Bevy, including common patterns/things you would see in a game.
    - Chapter 14 is a must read if you'd like to learn about what **ECS** can do.
    - Some pages are **outdated**, so if you're not sure if what the book describes is actually the best way to do things, feel free to message @raybbian on Discord!

- Bevy's official **Examples**: https://bevyengine.org/examples/ and https://bevyengine.org/examples-webgpu/
    - This is a collection of minimal examples that give you an understanding of how something can be done.
    - These are great resources if you're trying to accomplish something more advanced, but you're not sure how to get started

### LDTK

- The **Bevy ECS LDTK Book**: https://trouv.github.io/bevy_ecs_ldtk/v0.9.0/
    - Gives you a basic understanding on how the level from LDTK will be imported into Bevy
    - Pay attention to the [`anatomy of the world`](https://trouv.github.io/bevy_ecs_ldtk/v0.9.0/explanation/anatomy-of-the-world.html)

- Example **Platformer Game**: https://github.com/Trouv/bevy_ecs_ldtk/tree/main/examples/platformer
    - Really good resource to see the common patterns used when importing e.g. entities from LDTK to Bevy (with `from_entity_instance`, `with(fn)`, etc.)

- **Bevy ECS LDTK Docs**: https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/index.html
    - Take note of the traits [`LdtkEntity`](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/app/trait.LdtkEntity.html) and [`LdtkIntCell`](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/app/trait.LdtkIntCell.html)

## WGSL

- The **function reference**: https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl-function-reference.html 

- **Fundamentals tutorial**: https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl.html
