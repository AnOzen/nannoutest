use nannou::{prelude::*, rand};

fn main() {
    let build = nannou::sketch(view).size(800, 800);
    build.run();
}

fn view(app: &App, frame: Frame){
    app.main_window().set_title("Random Rectangles");
    app.main_window().set_resizable(false);
    let draw = app.draw();

    draw.rect().x_y(random_f32() * 800. - 400., random_f32() * 800. - 400.).w_h(random_range::<f32>(10., 200.), random_range::<f32>(10., 200.)).color(rgb8(random::<u8>(), rand::random::<u8>(), rand::random::<u8>()));

    draw.to_frame(app, &frame).unwrap();
}
