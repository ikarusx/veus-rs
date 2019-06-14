
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use winit::{WindowBuilder, dpi::LogicalSize, Event, EventsLoop, WindowEvent};
use std::sync::Arc;
use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[allow(unused)]
struct HelloTriangleApplication
{
    instance: Arc<Instance>,
    events_loop: EventsLoop,
}

impl HelloTriangleApplication
{
    pub fn initialize() -> Self
    {
        let instance = Self::create_instance();
        let events_loop = Self::init_window();

        Self
        {
            instance,
            events_loop,
        }
    }

    fn create_instance() -> Arc<Instance>
    {
        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("Failed to retrieve supported extensions!");
        println!("Supported extensions: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some("Hello Triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &required_extensions, None)
            .expect("Failed to create Vulkan instance!")
    }

    fn init_window() -> EventsLoop
    {
        let events_loop = EventsLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop);
        events_loop
    }

    fn main_loop(&mut self)
    {
        println!("Main Loop ...");
        loop
        {
            let mut done = false;
            self.events_loop.poll_events(|ev| {
                if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev {
                    done = true
                }
            });
            if done
            {
                return;
            }
        }
    }
}

fn main()
{
    let mut app = HelloTriangleApplication::initialize();
    app.main_loop();
}
