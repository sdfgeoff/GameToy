use gametoy::config_file::ResolutionScalingMode;



#[derive(PartialEq)]
enum ResScalingModeUi {
    Fixed,
    ViewportScale
}
impl ResScalingModeUi {
    pub fn from_resolution_scaling_mode(mode: &ResolutionScalingMode) -> Self {
        match mode {
            ResolutionScalingMode::Fixed(_, _) => Self::Fixed,
            ResolutionScalingMode::ViewportScale(_, _) => Self::ViewportScale,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Fixed => "Fixed",
            Self::ViewportScale => "Viewport Scale"
        }
    }
    pub fn to_default(&self) -> ResolutionScalingMode {
        match self {
            Self::Fixed => ResolutionScalingMode::Fixed(512, 512),
            Self::ViewportScale => ResolutionScalingMode::ViewportScale(1.0, 1.0)
        }
    }
}



pub fn resolution_scaling_mode_widget(scaling_mode: &mut ResolutionScalingMode, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let mut current_scaling_mode = ResScalingModeUi::from_resolution_scaling_mode(&scaling_mode);
        egui::ComboBox::from_id_source(12345)
            .selected_text(current_scaling_mode.to_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut current_scaling_mode, ResScalingModeUi::Fixed, ResScalingModeUi::Fixed.to_str());
                ui.selectable_value(&mut current_scaling_mode, ResScalingModeUi::ViewportScale, ResScalingModeUi::ViewportScale.to_str());
            });

        if current_scaling_mode != ResScalingModeUi::from_resolution_scaling_mode(&scaling_mode) {
            *scaling_mode = current_scaling_mode.to_default()
        }
        match scaling_mode {
            ResolutionScalingMode::Fixed(mut x, mut y) => {
                ui.add(egui::widgets::DragValue::new(&mut x).suffix("px"));
                ui.add(egui::widgets::DragValue::new(&mut y).suffix("px"));
                x = x.max(1);
                y = y.max(1);
                *scaling_mode = ResolutionScalingMode::Fixed(x, y);
            },
            ResolutionScalingMode::ViewportScale(mut x, mut y) => {
                x = x * 100.0;
                y = y * 100.0;
                ui.add(egui::widgets::DragValue::new(&mut x).suffix("%"));
                ui.add(egui::widgets::DragValue::new(&mut y).suffix("%"));
                x = x / 100.0;
                y = y / 100.0;
                x = x.max(0.0);
                y = y.max(0.0);
                *scaling_mode = ResolutionScalingMode::ViewportScale(x, y);
            }
        }
    });
}