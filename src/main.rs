use shapes::sphere::Sphere;
use shapes::Shapes::Spheres;
use vector::Vec3;

mod create_image;
mod shapes;
mod vector;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Need an output location.");
        std::process::exit(1);
    }

    let filepath = &args[1];

    let mut image = create_image::Image::new(1000, 1000);

    let circle = Sphere::new(Vec3::new(0.0, 3.0, -10.0), 0.2);
    let mut circle2 = Sphere::new(Vec3::new(0.0, -5.0, -10.0), 2.0);

    circle2.change_colour(image::Rgb([255, 0, 200]));

    image.add_shape(Spheres(circle));
    image.add_shape(Spheres(circle2));

    image.draw_image();

    image.create_image(filepath);
}
