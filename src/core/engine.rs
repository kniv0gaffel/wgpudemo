#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;





// pub fn lock_mouse_tab() -> bool
// {
//     if self.mouse_locked == false
//     {
//         self.window_state.window.set_cursor_grab(CursorGrabMode::Confined).or_else(|_| 
//         self.window_state.window.set_cursor_grab(CursorGrabMode::Locked)).unwrap();
//         self.window_state.window.set_cursor_visible(false);
//         self.mouse_locked = true; 
//     }
//     else
//     {
//         self.window_state.window.set_cursor_grab(CursorGrabMode::None).unwrap();
//         self.window_state.window.set_cursor_visible(true);
//         self.mouse_locked = false;
//     }
//     true
// }

// match event
// {
//     DeviceEvent::MouseMotion{ delta, } if self.mouse_locked == true => 
//     {
//         self.camera.controller.process_mouse(delta.0, delta.1);
//         true
//     }
//     _ => false,
// }


#[macro_export]
macro_rules! event_loop
{
    () => 
    {
        event_loop.run(
            move |event, _, control_flow| 
            match event 
            {
                /****************************************************************
                 * Window Events
                 ***************************************************************/
                Event::WindowEvent { ref event, window_id, } 
                if window_id == self.window.id() =>
                {
                    match event 
                    {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput 
                        {
                            input : KeyboardInput 
                            {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit, 
                        WindowEvent::Resized(physical_size) =>  self.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>  self.resize(**new_inner_size),
                        WindowEvent::KeyboardInput 
                        {
                            input: KeyboardInput 
                            {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Tab),
                                ..
                            },
                            ..
                        } => lock_mouse_tab(),
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(key),
                                    state,
                                    ..
                                },
                            ..
                        } => self.camera.controller.process_keyboard(*key, *state),
                        WindowEvent::MouseWheel { delta, .. } => self.camera.controller.process_scroll(delta);
                        _ => {}
                    }
                }
                /****************************************************************
                 * Device Events
                 ***************************************************************/ 
                Event::DeviceEvent { ref event, .. } => self.device_input(event),
                /****************************************************************
                 * Redraw Requested
                 ***************************************************************/
                Event::RedrawRequested(window_id) if window_id == self.window.id() => 
                {
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    self.update(dt, last_render_time);
                    match self.render()
                    {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => self.resize(self.window_state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                /****************************************************************
                 * Main Events Cleared
                 ***************************************************************/
                Event::MainEventsCleared => self.window.request_redraw(),
                /****************************************************************
                 * Default
                 ***************************************************************/
                _ => {}
            }
        );
    };
}




#[macro_export]
macro_rules! rezize
{
    ($new_size:expr) => 
    {
        if $new_size.width > 0 && $new_size.height > 0 
        {
            self.size = $new_size;
            renderer.config.width = $new_size.width;
            renderer.config.height = $new_size.height;
            renderer.renderer.surface.configure(&renderer.device, &renderer.config);
            renderer.camera.projection.resize($new_size.width, $new_size.height);
        }
    };
}




#[macro_export]
macro_rules! new_device 
{
    () => 
    {
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
                ..Default::default()
            }
        );
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();
        (device, queue)
    };
    ($window:expr) => 
    {{
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
                ..Default::default()
            }
        );
        let surface = instance.create_surface($window)
            .expect("Failed to create surface");
        
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();
        let size = $window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration 
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 1,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let _modes = &surface_caps.present_modes;
        (device, queue, size, config, surface)
    }};
}




#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn new_window(title: &str) -> (winit::event_loop::EventLoop<()>, winit::window::Window)
{
    cfg_if::cfg_if! 
    {
        if #[cfg(target_arch = "wasm32")] 
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } 
        else { env_logger::init(); }
    }

    let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    window.request_inner_size(winit::dpi::PhysicalSize::new(1500, 1500));

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;

        window.set_inner_size(PhysicalSize::new(1500, 1500));
        

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| 
            {
                let dst = doc.get_element_by_id("f_stop")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append canvas to document body.");
    }
    (event_loop, window)
}
