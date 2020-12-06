use {
	crate::{structs::GameData, scenes::prelude::{Scene, SceneData}},
	net_ui::{Context, Layer},
	quicksilver::Graphics,
};

pub struct IntroScene {
	data: SceneData,
}

impl IntroScene {
	pub fn new() -> Self {
		IntroScene {
			data: SceneData {
				context: Context::new(), // can also init the layers/widgets here
			},
		}
	}
}

impl Scene for IntroScene {
	fn init(&mut self) {
		//init data
		//self.context
	}

	fn handle_data(&mut self, gd: &GameData) {
		
	}

	fn draw_scene(&self, gfx: &mut Graphics) {
		// okay... maybe not cacheing the widgets - load them up every time
		// why though?
	}

	fn trans_from(&mut self) {
		//free data
	}
}

/*
There's a few ways I could try to implement this
	> store data
*/
