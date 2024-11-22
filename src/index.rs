use anathema::{
    component::Component,
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{State, Value},
};

pub struct Index {}

#[derive(State)]
pub struct IndexState {
    pub active: Value<u64>,
}

impl Component for Index {
    type State = IndexState;

    type Message = ();

    fn on_key(
        &mut self,
        _key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        if State::to_number(&state.active).unwrap().as_uint() == 0 {
            *state.active.to_mut() = 1;
        } else {
            *state.active.to_mut() = 0;
        }
    }
}

pub fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) {
    let _ = runtime_builder.register_component(
        "index",
        "src/templates/index.aml",
        Index {},
        IndexState {
            active: Value::new(0),
        },
    );
}
