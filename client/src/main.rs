// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Opening and closing windows and using window and context menus.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::{
    commands as sys_cmds,
    widget::{
        prelude::*, Align, BackgroundBrush, Button, Controller, ControllerHost, Flex, Label,
        Padding,
    },
    AppDelegate, AppLauncher, Application, Color, Command, Data, DelegateCtx, Handled,
    LocalizedString, Menu, MenuItem,
    Target::{self, Global},
    WindowConfig, WindowDesc, WindowHandle, WindowId,
};
use tracing::info;

#[derive(Debug, Clone, Default, Data)]
struct State {
    menu_count: usize,
    selected: usize,
    glow_hot: bool,
    window_open: bool,
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder()).title(
        LocalizedString::new("multiwin-demo-window-title").with_placeholder("Many windows!"),
    );
    AppLauncher::with_window(main_window)
        .delegate(Delegate {
            stream_capture_window: None,
        })
        .log_to_console()
        .launch(State::default())
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<State> {
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &State, _env| data.menu_count.into());
    let label = Label::new(text);
    let inc_button =
        Button::<State>::new("Add menu item").on_click(|_ctx, data, _env| data.menu_count += 1);
    let dec_button = Button::<State>::new("Remove menu item")
        .on_click(|_ctx, data, _env| data.menu_count = data.menu_count.saturating_sub(1));
    let new_button = Button::<State>::new("New window").on_click(|ctx, _data, _env| {
        _data.window_open = true;

        ctx.new_window(
            WindowDesc::new(ui_builder())
                .title(
                    LocalizedString::new("multiwin-demo-window-title")
                        .with_placeholder("Many windows!"),
                )
                .transparent(true),
        )
    });
    let quit_button = Button::<State>::new("Quit app").on_click(|_ctx, _data, _env| {
        Application::global().quit();
    });

    let col = Flex::column()
        .add_flex_child(Align::centered(Padding::new(5.0, label)), 1.0)
        .add_flex_child(
            Align::centered(
                Flex::row()
                    .add_child(Padding::new(5.0, inc_button))
                    .add_child(Padding::new(5.0, dec_button)),
            ),
            1.0,
        )
        .add_flex_child(
            Align::centered(
                Flex::row()
                    .add_child(Padding::new(5.0, new_button))
                    .add_child(Padding::new(5.0, quit_button)),
            ),
            1.0,
        );

    Glow::new(col)
}

struct Glow<W> {
    inner: W,
}

impl<W> Glow<W> {
    pub fn new(inner: W) -> Glow<W> {
        Glow { inner }
    }
}

impl<W: Widget<State>> Widget<State> for Glow<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &State, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &State, data: &State, env: &Env) {
        if old_data.glow_hot != data.glow_hot {
            ctx.request_paint();
        }
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &State,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, env: &Env) {
        if data.glow_hot && ctx.is_hot() {
            BackgroundBrush::Color(Color::rgb8(200, 55, 55)).paint(ctx, data, env);
        }
        self.inner.paint(ctx, data, env);
    }
}

struct Delegate {
    stream_capture_window: Option<WindowId>,
}

impl AppDelegate<State> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        _cmd: &Command,
        _data: &mut State,
        _env: &Env,
    ) -> Handled {
        Handled::No
    }

    fn window_added(
        &mut self,
        id: WindowId,
        _handle: WindowHandle,
        _data: &mut State,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        info!("Window added, id: {:?}", id);
        self.stream_capture_window = Some(id);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        _data: &mut State,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        info!("Window removed, id: {:?}", id);
        if match self.stream_capture_window {
            Some(current_id) => current_id == id,
            None => false,
        } {
            self.stream_capture_window = None
        }
    }
}
