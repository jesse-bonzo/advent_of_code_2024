use std::fs;

pub fn solve() {
    let example_input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day4_example.txt",
    )
    .expect("Unable to read input");
    if part1::solve(&example_input) != 18 {
        println!("Example solution not valid for part1");
        return;
    }

    if part2::solve(&example_input) != 9 {
        println!("Example solution not valid for part2");
        return;
    }

    let input = fs::read_to_string(
        "C:\\Users\\jbonz\\RustroverProjects\\adventofcode2024\\input\\day4.txt",
    )
    .expect("Unable to read input");
    println!("Solution Part1: {}", part1::solve(&input));
    println!("Solution Part2: {}", part2::solve(&input));
}

mod part1 {
    pub(crate) fn solve(input: &String) -> i32 {
        // Looking for XMAS horizontally, vertically, diagonally, backwards
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut count = 0;
        for row_index in 0..grid.len() {
            let row = &grid[row_index];
            for index in 0..row.len() {
                if row[index] == 'X' {
                    if index + 3 < row.len() {
                        // check horizontal to the right
                        if row[index + 1] == 'M' && row[index + 2] == 'A' && row[index + 3] == 'S' {
                            count = count + 1;
                        }
                    }

                    if index >= 3 {
                        // check horizontal to the left
                        if row[index - 1] == 'M' && row[index - 2] == 'A' && row[index - 3] == 'S' {
                            count = count + 1;
                        }
                    }

                    if row_index + 3 < grid.len() {
                        // check vertical down
                        if grid[row_index + 1][index] == 'M'
                            && grid[row_index + 2][index] == 'A'
                            && grid[row_index + 3][index] == 'S'
                        {
                            count = count + 1;
                        }
                    }

                    if row_index >= 3 {
                        // check vertical up
                        if grid[row_index - 1][index] == 'M'
                            && grid[row_index - 2][index] == 'A'
                            && grid[row_index - 3][index] == 'S'
                        {
                            count = count + 1;
                        }
                    }

                    if index + 3 < row.len() && row_index + 3 < grid.len() {
                        // check diagonal to the right and down
                        if grid[row_index + 1][index + 1] == 'M'
                            && grid[row_index + 2][index + 2] == 'A'
                            && grid[row_index + 3][index + 3] == 'S'
                        {
                            count = count + 1;
                        }
                    }

                    if index >= 3 && row_index + 3 < grid.len() {
                        // check diagonal to the left and down
                        if grid[row_index + 1][index - 1] == 'M'
                            && grid[row_index + 2][index - 2] == 'A'
                            && grid[row_index + 3][index - 3] == 'S'
                        {
                            count = count + 1;
                        }
                    }

                    if index + 3 < row.len() && row_index >= 3 {
                        // check diagonal to the right and up
                        if grid[row_index - 1][index + 1] == 'M'
                            && grid[row_index - 2][index + 2] == 'A'
                            && grid[row_index - 3][index + 3] == 'S'
                        {
                            count = count + 1;
                        }
                    }

                    if index >= 3 && row_index >= 3 {
                        // check diagonal to the left and up
                        if grid[row_index - 1][index - 1] == 'M'
                            && grid[row_index - 2][index - 2] == 'A'
                            && grid[row_index - 3][index - 3] == 'S'
                        {
                            count = count + 1;
                        }
                    }
                }
            }
        }
        count
    }
}

mod part2 {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    #[derive(Hash, PartialEq, Eq, Debug, Ord, PartialOrd, Clone)]
    struct Point {
        row_index: usize,
        col_index: usize,
    }

    #[derive(Eq, Debug)]
    struct X {
        points: [Point; 4],
    }

    impl Hash for X {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let mut mpoint = self.points.clone();
            mpoint.sort();
            mpoint[0].hash(state);
            mpoint[1].hash(state);
            mpoint[2].hash(state);
            mpoint[3].hash(state);
        }
    }

    impl PartialEq for X {
        fn eq(&self, other: &Self) -> bool {
            let mut first = self.points.clone();
            let mut second = other.points.clone();
            first.sort();
            second.sort();
            first[0] == second[0]
                && first[1] == second[1]
                && first[2] == second[2]
                && first[3] == second[3]
        }
    }

    fn verify() {
        assert_eq!(
            Point {
                row_index: 1,
                col_index: 1
            },
            Point {
                row_index: 1,
                col_index: 1
            }
        );

        assert_ne!(
            Point {
                row_index: 1,
                col_index: 1
            },
            Point {
                row_index: 0,
                col_index: 1
            }
        );

        assert_eq!(
            X {
                points: [
                    Point {
                        row_index: 0,
                        col_index: 0,
                    },
                    Point {
                        row_index: 0,
                        col_index: 2,
                    },
                    Point {
                        row_index: 2,
                        col_index: 0,
                    },
                    Point {
                        row_index: 2,
                        col_index: 2,
                    },
                ],
            },
            X {
                points: [
                    Point {
                        row_index: 2,
                        col_index: 2,
                    },
                    Point {
                        row_index: 2,
                        col_index: 0,
                    },
                    Point {
                        row_index: 0,
                        col_index: 0,
                    },
                    Point {
                        row_index: 0,
                        col_index: 2,
                    },
                ],
            }
        );

        let mut set: HashSet<X> = HashSet::new();
        set.insert(X {
            points: [
                Point {
                    row_index: 0,
                    col_index: 0,
                },
                Point {
                    row_index: 0,
                    col_index: 2,
                },
                Point {
                    row_index: 2,
                    col_index: 0,
                },
                Point {
                    row_index: 2,
                    col_index: 2,
                },
            ],
        });
        set.insert(X {
            points: [
                Point {
                    row_index: 2,
                    col_index: 2,
                },
                Point {
                    row_index: 2,
                    col_index: 0,
                },
                Point {
                    row_index: 0,
                    col_index: 0,
                },
                Point {
                    row_index: 0,
                    col_index: 2,
                },
            ],
        });
        assert_eq!(set.len(), 1, "set should have 1 unique value");
        set.insert(X {
            points: [
                Point {
                    row_index: 2,
                    col_index: 2,
                },
                Point {
                    row_index: 6,
                    col_index: 7,
                },
                Point {
                    row_index: 6,
                    col_index: 6,
                },
                Point {
                    row_index: 5,
                    col_index: 6,
                },
            ],
        });
        set.insert(X {
            points: [
                Point {
                    row_index: 2,
                    col_index: 2,
                },
                Point {
                    row_index: 6,
                    col_index: 7,
                },
                Point {
                    row_index: 6,
                    col_index: 6,
                },
                Point {
                    row_index: 5,
                    col_index: 6,
                },
            ],
        });
        assert_eq!(set.len(), 2, "set should have 2 unique values");

        let points = [
            Point {
                row_index: 0,
                col_index: 0,
            },
            Point {
                row_index: 0,
                col_index: 10,
            },
            Point {
                row_index: 10,
                col_index: 10,
            },
            Point {
                row_index: 10,
                col_index: 1,
            },
            Point {
                row_index: 2,
                col_index: 0,
            },
        ];
        let mut points_sorted = points.clone();
        points_sorted.sort();
        assert_eq!(
            points_sorted[0],
            Point {
                row_index: 0,
                col_index: 0,
            }
        );
        assert_eq!(
            points_sorted[1],
            Point {
                row_index: 0,
                col_index: 10,
            }
        );
        assert_eq!(
            points_sorted[2],
            Point {
                row_index: 2,
                col_index: 0,
            }
        );
        assert_eq!(
            points_sorted[3],
            Point {
                row_index: 10,
                col_index: 1,
            }
        );
        assert_eq!(
            points_sorted[4],
            Point {
                row_index: 10,
                col_index: 10,
            }
        );
    }

    pub(crate) fn solve(input: &String) -> i32 {
        verify();

        // Looking for MAS or SAM in an X forwards and backwards
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut found: HashSet<X> = HashSet::new();
        for row_index in 0..grid.len() {
            for col_index in 0..grid[row_index].len() {
                let x1 = X {
                    points: [
                        Point {
                            row_index,
                            col_index,
                        },
                        Point {
                            row_index: row_index + 2,
                            col_index: col_index + 2,
                        },
                        Point {
                            row_index,
                            col_index: col_index + 2,
                        },
                        Point {
                            row_index: row_index + 2,
                            col_index,
                        },
                    ],
                };

                if is_valid(&grid, &x1) {
                    found.insert(x1);
                }

                if row_index >= 2 {
                    let x = X {
                        points: [
                            Point {
                                row_index,
                                col_index,
                            },
                            Point {
                                row_index: row_index - 2,
                                col_index: col_index + 2,
                            },
                            Point {
                                row_index,
                                col_index: col_index + 2,
                            },
                            Point {
                                row_index: row_index - 2,
                                col_index,
                            },
                        ],
                    };
                    if is_valid(&grid, &x) {
                        found.insert(x);
                    }
                }

                if col_index >= 2 {
                    let x = X {
                        points: [
                            Point {
                                row_index,
                                col_index,
                            },
                            Point {
                                row_index: row_index + 2,
                                col_index: col_index - 2,
                            },
                            Point {
                                row_index,
                                col_index: col_index - 2,
                            },
                            Point {
                                row_index: row_index + 2,
                                col_index,
                            },
                        ],
                    };
                    if is_valid(&grid, &x) {
                        found.insert(x);
                    }
                }

                if row_index >= 2 && col_index >= 2 {
                    let x = X {
                        points: [
                            Point {
                                row_index,
                                col_index,
                            },
                            Point {
                                row_index: row_index - 2,
                                col_index: col_index - 2,
                            },
                            Point {
                                row_index,
                                col_index: col_index - 2,
                            },
                            Point {
                                row_index: row_index - 2,
                                col_index,
                            },
                        ],
                    };
                    if is_valid(&grid, &x) {
                        found.insert(x);
                    }
                }
            }
        }
        println!("found: {}", found.len());
        // println!("found: {:?}", found);
        found.len() as i32
    }

    fn get(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> char {
        if row_index < grid.len() && col_index < grid[row_index].len() {
            grid[row_index][col_index]
        } else {
            ' '
        }
    }

    fn get_by_point(grid: &Vec<Vec<char>>, point: &Point) -> char {
        get(grid, point.row_index, point.col_index)
    }

    fn is_valid(grid: &Vec<Vec<char>>, x: &X) -> bool {
        if x.points
            .iter()
            .all(|p| get_by_point(&grid, &p) == 'M' || get_by_point(&grid, &p) == 'S')
        {
            let mut points = x.points.clone();
            points.sort();

            let top_left = get_by_point(grid, &points[0]);
            let top_right = get_by_point(grid, &points[1]);
            let bottom_left = get_by_point(grid, &points[2]);
            let bottom_right = get_by_point(grid, &points[3]);
            let middle = get_by_point(
                grid,
                &Point {
                    row_index: points[0].row_index + 1,
                    col_index: points[0].col_index + 1,
                },
            );

            return if middle == 'A' {
                if (top_left == 'M' && bottom_right == 'S')
                    && (top_right == 'M' && bottom_left == 'S')
                {
                    println!("points: {:?}", points);

                    true
                } else if (top_left == 'S' && bottom_right == 'M')
                    && (top_right == 'M' && bottom_left == 'S')
                {
                    println!("points: {:?}", points);

                    true
                } else if (top_left == 'S' && bottom_right == 'M')
                    && (top_right == 'S' && bottom_left == 'M')
                {
                    println!("points: {:?}", points);

                    true
                } else if (top_left == 'M' && bottom_right == 'S')
                    && (top_right == 'S' && bottom_left == 'M')
                {
                    println!("points: {:?}", points);

                    true
                } else {
                    false
                }
            } else {
                false
            };
        }
        false
    }
}
