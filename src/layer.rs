use crate::widgets::Widget;

#[derive(Default)]
pub struct Layer {
	widgets: Vec<Box<&'static Widget>>
}

impl Layer {
	pub fn add_widget<T>(&mut self, widget: &'static T) where T: Widget {
		self.widgets.push(Box::new(widget));
	}
}