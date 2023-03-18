use eframe::emath::RectTransform;
use egui::{Sense, Pos2, Rect, vec2, Stroke, Color32, Painter, Vec2, pos2};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct SchematicApp {
    resistors: Vec<Resistor>,
}

impl Default for SchematicApp {
    fn default() -> Self {
        Self {
	    resistors: Vec::new(),
	}
    }
}

impl SchematicApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for SchematicApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { resistors, } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("AC/DC analysis of linear circuits")
        });

	egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                // ui.text_edit_singleline(label);
            });

            //ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
		self.resistors.push(Resistor::new());
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
	
        egui::CentralPanel::default().show(ctx, |ui| {
	    // set up the drawing canvas with normalized coordinates:
            let (response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

	    // normalize painter coordinates to +-1 units in each direction with
	    // [0,0] in the center:
            let painter_proportions = response.rect.square_proportions();
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO - painter_proportions, 2. * painter_proportions),
                response.rect,
            );

            let color = if ui.visuals().dark_mode {
		Color32::WHITE
            } else {
		Color32::BLACK
            };
	    
	    for (i, r) in self.resistors.iter_mut().enumerate() {
		let size = Vec2::splat(2.0 * 8.0);
		let resistor_position = r.location();
		let resistor_rect = Rect::from_center_size(resistor_position, size);

		let resistor_id = response.id.with(i);
                let resistor_response = ui.interact(resistor_rect, resistor_id, Sense::drag());

		r.set_location(resistor_position + resistor_response.drag_delta());
		
		r.draw(&painter);
	    }
	});
    }
}

/// Resistor symbol
///
/// This is the zig-zag resistor symbol. It comprises
/// 6 equilateral triangles, alternating up and down (with
/// a common base horizontal line), with a terminal segment
/// at each end
struct Resistor {
    origin: Pos2,
    /// Length of base of equilateral triangle
    x: f32,
    /// Length of segment at each end of resistor
    d: f32,
    /// The line color and width
    stroke: Stroke,
    selected: bool,
}

impl Resistor {

    fn new() -> Self {	
	Self {
	    origin: pos2(256.0, 256.0),
	    x: 5.0,
	    d: 10.0,
	    stroke: Stroke {
		width: 2.0,
		color: Color32::WHITE,
	    },
	    selected: false,
	}
    }

    fn location(&self) -> Pos2 {
	self.origin
    }

    fn set_location(&mut self, new_location: Pos2) {
	self.origin = new_location;
    }
    
    /// Draw either the right (reflect == 1.0) or the right
    /// (reflect == -1.0) portion of the symbol
    fn draw_half(&self, reflect: f32, painter: &Painter) {
	let half_x = reflect * self.x/2.0;
	let y = - reflect * self.x * f32::sqrt(3.0) / 2.0;
	let start = self.origin;
	let end = self.origin + vec2(half_x, -y);
	painter.line_segment([start, end], self.stroke);
	let start = end;
	let end = end + vec2(2.0 * half_x, 2.0 * y);
	painter.line_segment([start, end], self.stroke);
	let start = end;
	let end = end + vec2(2.0 * half_x, - 2.0 * y);
	painter.line_segment([start, end], self.stroke);
	let start = end;
	let end = end + vec2(half_x, y);
	painter.line_segment([start, end], self.stroke);
	let start = end;
	let end = end + vec2(reflect * self.d, 0.0);
	painter.line_segment([start, end], self.stroke);
	
    }
    
    fn draw(&self, painter: &Painter) {
	self.draw_half(1.0, painter);
	self.draw_half(-1.0, painter);

    }
}
