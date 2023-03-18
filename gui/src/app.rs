use eframe::emath::RectTransform;
use egui::{Sense, Pos2, Rect, vec2, Stroke, Color32, Painter, Vec2, pos2, Ui, Id, Shape};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct SchematicApp {
    resistors: Vec<Resistor>,
    // An item in this vector means a link between two resistors.
    // The two usizes are the indices of the resistors, and the f32 are +-1
    // depending which terminal are joined
    edges: Vec<(usize, f32, usize, f32)>,
}

impl Default for SchematicApp {
    fn default() -> Self {
        Self {
	    resistors: Vec::new(),
	    edges: Vec::new(),
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
        let Self { resistors, edges, } = self;

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
            if ui.button("Add resistor").clicked() {
		self.resistors.push(Resistor::new());
            }

            if ui.button("Add edge").clicked() {
		self.edges.push((1, 1.0, 0, 1.0));
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

	    // Draw resistors
	    for (i, r) in self.resistors.iter_mut().enumerate() {
		let resistor_id = response.id.with(i);
		r.update(ui, &painter, resistor_id)
	    }

	    // Draw edges
	    let stroke = ui.style().noninteractive().fg_stroke;
	    for edge in self.edges.iter() {
		let (resistor_1, term_1, resistor_2, term_2) = edge;
		let start = self.resistors[*resistor_1].term_location(*term_1);
		let end = self.resistors[*resistor_2].term_location(*term_2);
		let edge_line = Shape::line(vec![start, end], stroke);
		painter.add(edge_line);
		
	    }

	    
	});
    }
}

enum Orientation {
    Vertical,
    Horizontal,
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
    orientation: Orientation,
}

impl Resistor {

    fn new() -> Self {	
	Self {
	    origin: pos2(256.0, 256.0),
	    x: 8.0,
	    d: 15.0,
	    orientation: Orientation::Vertical,
	}
    }

    fn location(&self) -> Pos2 {
	self.origin
    }

    fn set_location(&mut self, new_location: Pos2) {
	self.origin = new_location;
    }

    fn rotate(&mut self) {
	match self.orientation {
	    Orientation::Horizontal => self.orientation = Orientation::Vertical,
	    Orientation::Vertical => self.orientation = Orientation::Horizontal,
	}
    }

    fn central_bounding_box(&self) -> Rect {
	let y = self.x * f32::sqrt(3.0) / 2.0;
	match self.orientation {
	    Orientation::Horizontal => {
		Rect::from_center_size(self.origin, vec2(6.0 * self.x, y))
	    }
	    Orientation::Vertical => {
		Rect::from_center_size(self.origin, vec2(y, 6.0 * self.x))		
	    }
	}
    }

    fn term_location(&self, side: f32) -> Pos2 {
	match self.orientation {
	    Orientation::Horizontal => self.origin + side * vec2(3.0 * self.x + self.d, 0.0),
	    Orientation::Vertical => self.origin + side * vec2( 0.0, 3.0 * self.x + self.d),
	}
    }
    
    /// side = +-1.0 depending on which terminal is required
    fn term_bounding_box(&self, side: f32) -> Rect {
	Rect::from_center_size(self.term_location(side), Vec2::splat(5.0))
    }
    
    fn update(&mut self, ui: &mut Ui, painter: &Painter, resistor_id: Id) {

	// Detect a drag of the main component
        let main_response = ui.interact(self.central_bounding_box(),
					resistor_id, Sense::drag());
	self.set_location(self.origin + main_response.drag_delta());

	// Check for rotations while the resistor is in focus
	if main_response.hovered() {
	    if ui.input(|i| i.key_pressed(egui::Key::R)) {
		self.rotate()
	    }
	}

	// Draw the resistor
	let stroke = ui.style().interact(&main_response).fg_stroke;
	self.draw(&painter, stroke);

	// Draw terminal 1 circle
	let term_1_response = ui.interact(self.term_bounding_box(1.0),
					  resistor_id.with(1), Sense::drag());
	let stroke = ui.style().interact(&term_1_response).fg_stroke;
	let term_1 = Shape::circle_stroke(self.term_location(1.0), 5.0, stroke);
	painter.add(term_1);

	// Draw terminal 2 circle
	let term_2_response = ui.interact(self.term_bounding_box(-1.0),
					  resistor_id.with(2), Sense::drag());
	let stroke = ui.style().interact(&term_2_response).fg_stroke;
	let term_2 = Shape::circle_stroke(self.term_location(-1.0), 5.0, stroke);
	painter.add(term_2);

	
	// This stroke depends on whether the mouse is hovering over the
	// region or not
    }
    
    /// Draw either the right (reflect == 1.0) or the right
    /// (reflect == -1.0) portion of the symbol
    fn draw_half(&self, reflect: f32, painter: &Painter, stroke: Stroke) {
	let half_x = reflect * self.x/2.0;
	let y = - reflect * self.x * f32::sqrt(3.0) / 2.0;
	match self.orientation {
	    Orientation::Horizontal => {
		let start = self.origin;
		let end = self.origin + vec2(half_x, -y);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(2.0 * half_x, 2.0 * y);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(2.0 * half_x, - 2.0 * y);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(half_x, y);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(reflect * self.d, 0.0);
		painter.line_segment([start, end], stroke);	
	    },
	    Orientation::Vertical => {
		let start = self.origin;
		let end = self.origin + vec2(-y, half_x);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(2.0 * y, 2.0 * half_x);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(- 2.0 * y, 2.0 * half_x);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(y, half_x);
		painter.line_segment([start, end], stroke);
		let start = end;
		let end = end + vec2(0.0, reflect * self.d);
		painter.line_segment([start, end], stroke);		
	    }
	}
    }
    
    fn draw(&self, painter: &Painter, stroke: Stroke) {
	self.draw_half(1.0, painter, stroke);
	self.draw_half(-1.0, painter, stroke);

    }
}
