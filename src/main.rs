use std::{f32::{consts::PI, NEG_INFINITY}, thread, time::Duration};

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {

    let chars = ".,-~:;=!*#$@";

    let or: f32 = 10.0;
    let r: f32 = 4.0;
    let n = 100;

    let cx = 40;
    let cy = 20;


    let mut tick = 0;
    loop {
        clear();

        let mut zbuffer = [
            [NEG_INFINITY; 80]; 40
        ];
        let mut canvas = [
            [' '; 80]; 40
        ];

        let rax = tick as f32 * PI / 128.0;
        let ray = tick as f32 * PI / 128.0;
        for i in 0..n {
            for j in 0..n {
                let theta = 2.0 * PI * i as f32 / n as f32;
                let phi = 2.0 * PI * j as f32 / n as f32;

                let x1 = (or + r * theta.cos()) * phi.cos();
                let y1 = (or + r * theta.cos()) * phi.sin();
                let z1 = r * theta.sin();

                // rot x
                let y2 = y1 * rax.cos() - z1 * rax.sin();
                let z2 = y1 * rax.sin() + z1 * rax.cos();

                // rot y
                let x3 = x1 * ray.cos() + z2 * ray.sin();
                let z3 = -x1 * ray.sin() + z2 * ray.cos();
                
                let mut l = phi.cos() * theta.cos() * theta.sin() - phi.sin() * theta.cos();
                l = 6.0 + (l * 5.5);

                let x: usize = (x3 as i32 + cx).try_into().unwrap();
                let y: usize = (y2 as i32 + cy).try_into().unwrap();
                let z = z3;
                let l: usize = l as usize;

                if z > zbuffer[y][x] {
                    zbuffer[y][x] = z;
                    canvas[y][x] = chars.chars().nth(l).unwrap();
                }

            }
        }

        // print
        println!("{}", tick);
        for i in 0..canvas.len() {
            for j in 0..canvas[0].len() {
                let c = canvas[i][j];
                print!("{}", c);
            }
            print!("\n");
        }

        thread::sleep(Duration::from_millis(1000/30));
        tick += 1;
    };

}
