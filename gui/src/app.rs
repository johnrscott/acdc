use eframe::emath::RectTransform;
use egui::{Sense, Pos2, Rect, vec2, Stroke, Color32, Painter};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("AC/DC analysis of linear circuits")
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
	    
	    let resistor = Resistor::new();
	    resistor.draw(&painter, to_screen * Pos2::new(0.0, 0.0));
	});
    }
}

/// The resistor symbol
///
/// This is the zig-zag resistor symbol. It comprises
/// 6 equilateral triangles, alternating up and down (with
/// a common base horizontal line), with a terminal segment
/// at each end
struct Resistor {
    /// Length of base of equilateral triangle
    x: f32,
    /// Length of segment at each end of resistor
    d: f32,
    /// The line color and width
    stroke: Stroke,
}

impl Resistor {

    fn new() -> Self {
	Self {
	    x: 30.0,
	    d: 60.0,
	    stroke: Stroke {
		width: 2.0,
		color: Color32::WHITE,
	    }
	}
    }

    /// Draw either the right (reflect == 1.0) or the right
    /// (reflect == -1.0) portion of the symbol
    fn draw_half(&self, reflect: f32, painter: &Painter, origin: Pos2) {
	let half_x = reflect * self.x/2.0;
	let y = - reflect * self.x * f32::sqrt(3.0) / 2.0;
	let start = origin;
	let end = origin + vec2(half_x, -y);
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
    
    fn draw(&self, painter: &Painter, at: Pos2) {
	self.draw_half(1.0, painter, at);
	self.draw_half(-1.0, painter, at);

    }
}
