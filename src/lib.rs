use std::time::Instant;

use state::State;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
    window::WindowBuilder,
};

pub mod camera;
pub mod instance;
// pub mod mesh;
pub mod light;
pub mod model;
pub mod resources;
pub mod state;
pub mod texture;
pub mod vertex;
pub mod mesh;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
    let window = WindowBuilder::new()
        .with_title("skygen")
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window).await;
    let mut last_render_time = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let now = Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;
            state.update(dt);

            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        },
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion{ delta, },
            .. // We're not using device_id currently
        } => if state.mouse_pressed {
            state.camera_controller.process_mouse(delta.0, delta.1)
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
        }
        _ => {}
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn run() {
        pollster::block_on(super::run());
    }
}
