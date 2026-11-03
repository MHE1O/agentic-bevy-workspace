# Leafwing Input Manager 0.22

## Semantic Actions

```rust
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
enum PlayerAction {
    #[actionlike(Axis)]
    Move,
    Jump,
    Start,
}
```

`InputMap<A>` and `ActionState<A>` are entity components. Add `InputManagerPlugin::<A>::default()` and spawn an entity with the map; required components supply its action state.

```rust
fn input_map() -> InputMap<PlayerAction> {
    let mut map = InputMap::default();
    map.insert_axis(PlayerAction::Move, VirtualAxis::ad());
    map.insert_axis(PlayerAction::Move, VirtualAxis::horizontal_arrow_keys());
    map.insert(PlayerAction::Jump, KeyCode::Space);
    map.insert(PlayerAction::Start, KeyCode::Enter);
    map
}
```

With the template's `gamepad` feature, add `VirtualAxis::dpad_x()`, `GamepadControlAxis::LEFT_X`, `GamepadButton::South`, and `GamepadButton::Start`. That optional feature requires Linux `libudev` development files.

## Reading Gameplay Input

```rust
fn control(actions: Single<&ActionState<PlayerAction>, With<crate::actions::PlayerInput>>) {
    let horizontal = actions.value(&PlayerAction::Move);
    if actions.just_pressed(&PlayerAction::Jump) {
        info!("jump: {horizontal}");
    }
}
```

Use `value` for axes and `pressed`/`just_pressed`/`released` for buttons. Gameplay never reads `ButtonInput<KeyCode>` directly.

## Headless Mocking

For full input-map integration tests, mock physical input through `Buttonlike`; the plugin then produces semantic actions:

```rust
use bevy::prelude::*;
use leafwing_input_manager::user_input::Buttonlike;

fn press_d(app: &mut App) {
    KeyCode::KeyD.press(app.world_mut());
    app.update();
}
```

For an isolated controller test with no input-map update pass, direct mutation is concise:

```rust
use leafwing_input_manager::prelude::*;

fn mock(actions: &mut ActionState<PlayerAction>) {
    actions.press(&PlayerAction::Jump);
    actions.set_value(&PlayerAction::Move, 1.0);
    actions.release(&PlayerAction::Jump);
}
```

Do not combine direct mutation with a normal `app.update()` input-map pass: collected physical input overwrites the action state. Complete plugin lifecycle with `finish()`/`cleanup()` before manually updating the app.
