# Contributing

## Making changes to the level

- If you are a level designer or programmer making changes to the level, you need to **make a copy** of the level file. Copy the `lightborne.ldtk` file from `assets/lightborne.ldtk` to `assets/levels/<firstname>-<lastname>.ldtk`. This way, each one of you can make and test changes to the level without causing tons of merge conflicts. See my example already in the folder!
- In `src/level/setup.rs`, you will need to make two changes (search for 'CHANGEME'):
    ```rust
    impl Plugin for LevelSetupPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(LevelSelection::index(3))
                // CHANGEME:                          ^
                // Change this if you want to start in a different level. Note that the "Lyra" entity
                // should be present in this level.
                .add_systems(Startup, setup_level);
        }
    }
    ```
    ```rust
    fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load("lightborne.ldtk").into(),
            // CHANGEME:                           ^
            // Change this to the name of your own level file (likely
            // levels/<firstname>-<lastname>.ldtk)
            ..Default::default()
        });
    }
    ```
- Once you have done these things, then you can make edits to your own LDTK file. Try and paint some new tiles, **save**, and run again with `cargo r`.
- **IMPORTANT**: I have added the `src/level/setup.rs` file to `.gitignore`. This means that you should not include this file when you make a commit! This will prevent us from getting merge conflicts in this file.

## Commit messages and Branch names

- Try to use your best judgement for these: there are no rules!
- It's best have your commit messages describe the changes that you have made
- And your branch names to describe the feature you are implementing
- (It's not that deep)

## Where to put code

- If you're not sure where to put code, check out [the structure of the project](/resources/programming/lightborne-structure.md).
- If it's still not clear, feel free to make your best judgement, or ask on Discord!
- Moving code around is a lot easier than writing it.

## Making a pull request

- Once you have committed and pushed all of your changes to your fork, make a Pull Request on the main repository.
- It is somewhat likely that there will be merge conflicts. If you're not sure how to resolve the conflict,
    1. Follow this guide to [allow maintainers (me!) to edit the merge request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/allowing-changes-to-a-pull-request-branch-created-from-a-fork#enabling-repository-maintainer-permissions-on-existing-pull-requests)
    2. Ping me on Discord: I'll hop in a call and merge your commit with you!
