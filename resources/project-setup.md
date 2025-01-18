# Project Setup

Hi! If you're here, that means you want to start working on Lightborne. This guide will get Lightborne set up on your computer so you can start casting some light beams!

1. Installation
    1. [Code Editor](#code-editor)
    2. [Rust](#rust)
    3. [Bevy](#bevy)
    4. [Git](#git)
    5. [Ldtk (Optional)](#ldtk-optional)
2. Setup
    1. [Repository](#repository)
    2. [Project](#project)
3. [Conclusion](#conclusion)

## Code Editor

Any code editor will do, but if you don't know what to use, then [VSCode](https://code.visualstudio.com/) is a good place to start.

## Rust 

For most use cases, you should install `rustup`, which will then install Rust for you. To do this, you can follow the installation guide below.

### [Installation Guide](https://www.rust-lang.org/tools/install)

To see if you have successfully installed Rust, run `rustc --version` in a terminal. If there are no errors, move onto setting up Bevy.

### Notes

- If you have a _package manager_ on your system (like `pacman`, `Chocolatey`, etc) then feel free to install `rustup` from there.
- You can also install a standalone version of Rust, but note that if you work with other projects that use different versions of Rust, you'll be out of luck.

## Bevy

From the [official quickstart guide](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies):

> ### Linux
>
> - Follow the instructions at [Linux Dependencies](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md).
>
> ### Windows
>
> - Run the Visual Studio 2019 build tools installer
> - For easy setup, select the `Desktop development with C++` workload in the installer.
> - For a minimal setup, follow these steps:
>     1. In the installer, navigate to `Individual components`
>     2. Select the latest `MSVC` for your architecture and version of Windows
>     3. Select the latest `Windows SDK` for your version of Windows
>     4. Select the `C++ CMake tools` for Windows component
>     5. Install the components
>
> ### MacOS
> 
> - Install the Xcode command line tools with `xcode-select --install` or the Xcode app

### Notes

- You might already have these dependencies installed on your system already.
- Not sure? Move on for now, and come back if you have issues running the Lightborne program in the [project setup](#project) section.

## Git

We will be using Git as version control for this project. There are a couple options for you to install Git:

- [Github Desktop](https://github.com/apps/desktop)
- [Git](https://git-scm.com/downloads)

Before moving on, make sure that you can run `git -v` from the terminal, without errors.

### Notes

- If you're a beginner, Github Desktop provides a great way to get started with Git. Not only does it handle authentication for you, it allows you to visualize some of Git's initially confusing moving parts.
- I (Raymond Bian) do not know how to use the Desktop GUI :pensive:! 
    - The remainder of this tutorial will use CLI instructions
    - If you have any git issues and need some help, feel free to reach out (but note that I will probably help you resolve it through the CLI).

## Ldtk (Optional)

**Note**: You only need to do this step if you are a programmer/level designer.

[LDTK](https://ldtk.io/) is a level design toolkit from the director of Dead Cells. Install it using [this link.](https://ldtk.io/download/)

Once you launch the application, it should look something like this:

![Image](https://github.com/user-attachments/assets/518a002b-c4c8-46f3-b188-3e841b5609d5)

## Repository

1. First, head over to the [Github repository](https://github.com/raybbian/Lightborne)
2. Make a **fork** of the repository
    ![Image](https://github.com/user-attachments/assets/553b0c45-bf3b-4521-a49a-06ea18bd7d7b)
    ### If using the CLI:
3. Copy the repository URL
    ![Image](https://github.com/user-attachments/assets/6b4b0f7e-552b-453d-a748-bc28d2ab32c8)
4. Navigate to folder where you'd like to store the project, and type `git clone <url you just copied>`
    ### If using Github Desktop:
5. Follow [these instructions](https://docs.github.com/en/desktop/adding-and-cloning-repositories/cloning-a-repository-from-github-to-github-desktop).

### Notes

- If the folder you navigated to is called `Foo`, then the project will be cloned into `Foo/Lightborne/<project files>`
- If you'd rather have your folder structure like `Foo/<project files>` then run `git clone <url you just copied> .` (note the period).
- Here is a [nice tutorial with GUI instructions](https://docs.google.com/document/d/1_OLH8WOER0-sgenzXkye7k3H6un_LsuDimNk283oSnU/edit?usp=sharing) on the basics of Git.

## Project

Once you have installed and cloned the repository, all that is left to do is run Lightborne!

1. In a terminal, navigate to where you cloned your repository. Double check that there is a file called `Cargo.toml` in this directory.
2. Type `cargo r` (short for `cargo run`). This will compile and run the project!

### Notes

- This will probably take a while (only for the first time!).
- If you're interested, you can research [some ways to optimize for compile time](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional). We have already enabled dynamic linking for the project, but feel free to try anything else.

## Conclusion

If you've made it this far, then Lightborne is (probably) working on your machine.

You can play the demo (as of 1-18-2025) by 
- navigating with WASD
- jumping with SPACE
- changing lightbeam colors with 1, 2, and 3 (green, red, and white)
- holding left click to aim, and releasing to shoot
