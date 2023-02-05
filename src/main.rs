use nannou::prelude::*;

fn main() {
    nannou::app(model)
    .update(update)
    .view(view)
    .run()
}

struct Model {
    _window: window::Id
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .size(1024, 768)
        .build()
        .unwrap();

    return Model{
        _window
    }
}

fn update(_app:&App, model:&mut Model, _update:Update) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw:Draw = app.draw();
    // draw.background().color(rgba(0.0, 0.0, 0.0, 0.5));

    // let circle = Circle::new(pt2(100,100),50.0);
    draw.ellipse(geom::rect(10,10,100,100));

    draw.to_frame(app, &frame).unwrap()
}