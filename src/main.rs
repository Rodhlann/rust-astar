use grid_point::Point;
mod a_star;

use a_star::a_star;

fn main() {
    let start = Point { x: 8, y: 3 };
    let end = Point { x: 6, y: 3 };
    let walls = vec![
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

    a_star(10, 10, start, end, walls);
}
