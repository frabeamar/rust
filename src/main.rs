use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    // Create a new WGPU instance:
    // This represents the connection between your program and the graphics backends
    // (Vulkan, Metal, DX12, GL, depending on the system).
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor{
        backends: wgpu::Backends::all(),     // Allow all GPU backends
        dx12_shader_compiler: Default::default(),
    });

    // Enumerate all available GPU adapters (graphics cards)
    // and print their information.
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    // Create the Winit event loop.
    // This loop drives the entire application and processes OS events.
    let event_loop = EventLoop::new();

    // Create a window that belongs to the event loop.
    let window = Window::new(&event_loop).unwrap();
    window.set_title("my window"); // Set the window title

    // Initialize env_logger so WGPU and libraries can print debug info.
    env_logger::init();

    // Start handling events.
    // The `move` keyword means the closure takes ownership of any captured values.
    // otherwise the variable don't live long enough
    event_loop.run(move |event, _, control_flow| {
        // The window should wait for events instead of constantly polling.
        *control_flow = ControlFlow::Wait;

        match event {
            // Handle window-specific events.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // If the user clicks the close button, exit the application.
                *control_flow = ControlFlow::Exit;
            }

            // Ignore all other events for now.
            _ => {}
        }
    });
}
