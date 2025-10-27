use dioxus::desktop::{DesktopService, LogicalPosition, LogicalSize};
use dioxus::logger::tracing::error;

pub enum WindowSize {
    // Percent {
    //     width: f64,
    //     height: f64
    // },
    Pixel {
        width: f64,
        height: f64
    }
}

#[derive(Debug)]
pub struct WindowLogicalData {
    pub size: LogicalSize<f64>,
    pub centered_position: LogicalPosition<f64>
}

impl WindowLogicalData {
    pub fn new(window: &DesktopService, target_size: WindowSize) -> Option<WindowLogicalData> {
        let monitor_size: LogicalSize<f64> = match window.current_monitor() {
            Some(cm) => {
                cm.size().to_logical(cm.scale_factor())
            },
            None => {
                error!("Failed get monitor size");
                return Option::None;
            }
        };

        match target_size {
            // WindowSize::Percent { width, height } => {
            //     let window_size = LogicalSize {
            //         width: monitor_size.width * width / 100.0,
            //         height: monitor_size.height * height / 100.0
            //     };

            //     Option::Some(WindowLogicalData {
            //         size: window_size,
            //         centered_position: LogicalPosition {
            //             x: (monitor_size.width - window_size.width) / 2.0,
            //             y: (monitor_size.height - window_size.height) / 2.0
            //         }
            //     })
            // },
            WindowSize::Pixel { width, height } => {
                let window_size = LogicalSize {
                    width,
                    height
                };
                Option::Some(WindowLogicalData {
                    size: window_size,
                    centered_position: LogicalPosition {
                        x: (monitor_size.width - window_size.width) / 2.0,
                        y: (monitor_size.height - window_size.height) / 2.0
                    }
                })
            }
        }
    }
}