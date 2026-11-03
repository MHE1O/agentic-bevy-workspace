# Agent Directives

## Operational Loop

1. Read `games/<target>/SCOPE.md` and relevant guides before editing.
2. Write or update a headless test first; confirm it fails for the intended reason.
3. Make the smallest change inside the owning crate.
4. Run `cargo xtask verify --game <name>` before completion.
5. Update `SCOPE.md`: objective, blockers, milestone checkboxes, latest update, and next step.
6. Run `cargo xtask status` to confirm metadata remains valid.

## Golden Rules

- **Physics:** Never mutate `Transform` to move physical entities. Apply Avian `LinearVelocity`, `AngularVelocity`, `ExternalImpulse`, or `ExternalForce` in `FixedUpdate`.
- **Input:** Never read `ButtonInput<KeyCode>` in gameplay. Query `ActionState<PlayerAction>` from Leafwing.
- **Assets:** Never load ad-hoc paths in gameplay. Register handles in an `#[derive(AssetCollection, Resource)]` collection in `loading.rs`.
- **States:** Use `Loading -> Menu -> Playing`; do not invent implicit lifecycle flags.
- **Verification:** Every feature needs headless coverage and a successful scoped `cargo xtask verify`.

## Boundaries

- Keep shared bootstrap and `GameState` in `engine_core`.
- Keep window-only diagnostics in `dev_diagnostics`; never require them headlessly.
- Keep game behavior in the target game crate and `xtask` free of Bevy dependencies.
- Pin all Bevy ecosystem upgrades together; see `COMPATIBILITY.md`.
- Do not commit `target/`, generated smoke games, local environment files, or machine-specific linker settings.

## Definition of Done

Formatting, Clippy with warnings denied, unit tests, and headless integration tests pass. Documentation and scope metadata match actual behavior. Any remaining limitation is recorded as a blocker, not hidden.
