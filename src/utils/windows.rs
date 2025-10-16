use dioxus::desktop::{DesktopService, LogicalPosition, LogicalSize};
use dioxus::logger::tracing::error;


pub struct WindowPercentSize {
    width: f64,
    height: f64
}

impl WindowPercentSize {
    pub fn new(width: f64, height: f64) -> WindowPercentSize {
        return WindowPercentSize { height, width }
    }
}

#[derive(Debug)]
pub struct WindowLogicalData {
    pub size: LogicalSize<f64>,
    pub centered_position: LogicalPosition<f64>
}

impl WindowLogicalData {
    pub fn new(window: &DesktopService, target_size: WindowPercentSize) -> Option<WindowLogicalData> {
        let monitor_size: LogicalSize<f64> = match window.current_monitor() {
            Some(cm) => {
                cm.size().to_logical(cm.scale_factor())
            },
            None => {
                error!("Failed get monitor size");
                return Option::None;
            }
        };

        let window_size = LogicalSize {
            width: monitor_size.width * target_size.width / 100.0,
            height: monitor_size.height * target_size.height / 100.0
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