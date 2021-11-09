# Border adder
This image adds a black border at the top of an image, replacing pixels under it. By default it is 1/30 of the image height.

## Why?
I'm using GNOME and a theme that forces the top bar to be transparent. I decided that the easiest solution is to add a black border to the top of the image, and for slideshows - this would be a pain to do manually, so I decided to automate it. Maybe someone will find it useful.

## Usage
1. Adjust height where it says `for y in 0..(image.height() / 30)` in `src/main.rs`
1. Run `cargo run --release "./image_path.png"`

> This will overwrite your image!

## License
This project is licensed under [MIT](./LICENSE)
