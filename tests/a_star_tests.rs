#[path = "../src/a_star.rs"]
mod a_star;

#[cfg(test)]
mod tests {
    use crate::a_star::a_star;
    use grid_point::Point;
    use std::time::Instant;

    const START: Point = Point { x: 8, y: 3 };
    const END: Point = Point { x: 6, y: 3 };
    const WALLS: [Point; 13] = [
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

    #[test]
    fn a_star_test_10() {
        let expected = vec![
            vec![0, 0, 0, 6, 5, 4, 3, 4, 5, 0],
            vec![0, 0, 0, 5, 9, 9, 9, 9, 4, 0],
            vec![0, 0, 0, 4, 9, 0, 0, 9, 3, 0],
            vec![0, 0, 0, 3, 2, 1, 0, 9, 2, 0],
            vec![0, 0, 0, 0, 9, 0, 0, 9, 0, 0],
            vec![0, 0, 0, 0, 9, 9, 9, 9, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let timer = Instant::now();
        let result = a_star(10, 10, START, END, WALLS.to_vec());
        let duration = timer.elapsed();

        println!("Test duration: {:?}", duration);
        assert_eq!(expected, result)
    }

    #[test]
    fn a_star_test_20() {
        let timer = Instant::now();
        a_star(20, 20, START, END, WALLS.to_vec());
        let duration = timer.elapsed();

        println!("Test duration: {:?}", duration);
    }

    #[test]
    fn a_star_test_100() {
        let timer = Instant::now();
        a_star(100, 100, START, END, WALLS.to_vec());
        let duration = timer.elapsed();

        println!("Test duration: {:?}", duration);
    }
}
