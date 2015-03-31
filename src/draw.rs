
use form::Form;
use transform_2d::{self, Transform2D};

/// A type that can be used to render Forms.
pub trait Renderer {

    /// Render the given form with some given transform to the graphics device.
    /// This is the only method that should be implemented for the Renderer type.
    fn draw_form(&mut self, transform: Transform2D, form: Form);

}

/// Draw the given form with the given renderer.
pub fn draw<R: Renderer>(renderer: &mut R, form: Form) {
    renderer.draw_form(transform_2d::identity(), form);
}

/// Draw the given forms with the given renderer.
pub fn draw_forms<R: Renderer>(renderer: &mut R, forms: Vec<Form>) {
    for form in forms.into_iter() {
        draw(renderer, form);
    }
}

