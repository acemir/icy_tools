pub mod about;
pub mod appearance;
pub mod dialog;
pub mod export;
pub mod monitor;
pub mod screen;
pub mod shortcuts;
pub mod zoom;

pub fn wgpu_configuration() -> egui_wgpu::WgpuConfiguration {
    let mut setup = egui_wgpu::WgpuSetupCreateNew::default();
    let device_descriptor = setup.device_descriptor;
    setup.device_descriptor = std::sync::Arc::new(move |adapter| {
        let mut descriptor = device_descriptor(adapter);
        descriptor.required_limits = descriptor.required_limits.using_resolution(adapter.limits());
        descriptor
    });
    egui_wgpu::WgpuConfiguration {
        present_mode: egui_wgpu::wgpu::PresentMode::AutoNoVsync,
        wgpu_setup: setup.into(),
        ..Default::default()
    }
}
