use grid_point::Point;
mod a_star;

use a_star::a_star;

fn main() {
    const START: Point = Point { x: 8, y: 3 };
    const END: Point = Point { x: 6, y: 3 };
    const WALLS: [Point;13] = [
        Point { x: 4, y: 1 },
        Point { x: 4, y: 2 },
        Point { x: 4, y: 4 },
        Point { x: 4, y: 5 },
        Point { x: 5, y: 1 },
        Point { x: 5, y: 5 },
        Point { x: 6, y: 1 },
        Point { x: 6, y: 5 },
        Point { x: 7, y: 1 },
        Point { x: 7, y: 2 },
        Point { x: 7, y: 3 },
        Point { x: 7, y: 4 },
        Point { x: 7, y: 5 },
    ];
    const HEIGHT: usize = 10;
    const WIDTH: usize = 10;

    let best_node = a_star(HEIGHT, WIDTH, START, END, WALLS.to_vec());

    // Fill matrix with 0s, we have to use a Vec matrix here because height and width are not constant
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(HEIGHT);
    for _ in 0..HEIGHT {
        let mut row = Vec::with_capacity(WIDTH);
        for _ in 0..WIDTH {
            row.push(0);
        }
        matrix.push(row);
    }

    let mut out = best_node.clone();
    loop {
        matrix[out.pos.y as usize][out.pos.x as usize] = out.h;
        if let Some(parent) = *out.parent {
            out = parent;
        } else {
            break;
        }
    }
    for wall in WALLS {
        matrix[wall.y as usize][wall.x as usize] = 9;
    }

    println!("{:?}", matrix)
}
