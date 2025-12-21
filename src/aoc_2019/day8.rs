const INPUT: &str = include_str!("./inputs/day8.txt");

struct Layer {
    pixels: Vec<u8>,
    zero_count: u64,
}
type Image = Vec<Layer>;

fn parse_layers(input: &str, width: usize, height: usize) -> Image {
    let input = input.trim();
    let img_size = width * height;
    let layer_count = input.len() / img_size;
    let mut layers = Vec::with_capacity(layer_count);
    for cur_layer_idx in 0..layer_count {
        let mut layer = Vec::new();
        let start = img_size * cur_layer_idx;
        let end = start + img_size;
        let mut zero_count = 0;
        for ch in input[start..end].chars() {
            let cur_digit = ch.to_digit(10).unwrap() as u8;
            if cur_digit == 0 {
                zero_count += 1;
            }
            layer.push(cur_digit);
        }
        layers.push(Layer {
            pixels: layer,
            zero_count,
        });
    }
    layers
}

fn checksum(layers: &Image) -> u64 {
    let lowest_zero_layer = layers
        .iter()
        .enumerate()
        .fold((0, u64::MAX), |acc, (i, layer)| {
            if layer.zero_count < acc.1 {
                (i, layer.zero_count)
            } else {
                acc
            }
        });

    let mut ones = 0;
    let mut twos = 0;
    for value in layers[lowest_zero_layer.0].pixels.iter() {
        match *value {
            1 => {
                ones += 1;
            }
            2 => {
                twos += 1;
            }
            _ => {}
        }
    }

    ones * twos
}

fn _print_image(layers: &Image, width: usize, height: usize) {
    for y in 0..height {
        let y_offset = y * width;
        for x in 0..width {
            let idx = y_offset + x;
            let mut pixel = "  ";
            for l in layers {
                match l.pixels[idx] {
                    0 => {
                        break;
                    }
                    1 => {
                        pixel = "##";
                        break;
                    }
                    _ => {}
                }
            }
            print!("{pixel}");
        }
        println!("");
    }
}

#[test]
fn solutions() {
    let layers = parse_layers(INPUT, 25, 6);
    assert_eq!(2210, checksum(&layers));
    // Prints image to console, not trivial to write assertion for
    // print_image(&layers, 25, 6);
}
