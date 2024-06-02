use std::{
    collections::{HashMap, VecDeque},
    usize,
};

struct Node {
    x: usize,
    y: usize,
    points: Vec<[usize; 3]>,
}

pub fn diff(a: &Vec<&str>, b: &Vec<&str>) {
    // A = abcabba and B = cbabac.
    // let a: Vec<&str> = vec!["a", "b", "c", "a", "b", "b", "a"];
    // let b: Vec<&str> = vec!["c", "b", "a", "b", "a", "c"];

    // [x:0, y:2, pos:0]
    // new
    let mut queue: VecDeque<Node> = VecDeque::new(); //vec![[0, 0]];
    queue.push_back(Node {
        x: 0,
        y: 0,
        points: vec![[0, 0, 0]],
    });

    let cols = a.len();
    let rows = b.len();
    let mut seen: HashMap<usize, bool> = HashMap::new();

    let dirs = [[1, 1, 0], [0, 1, 1], [1, 0, 2]];

    // let mut i = 0;
    let mut list: Vec<[usize; 3]> = Vec::new();

    while queue.len() > 0 {
        let mut pos = queue.pop_front().unwrap();
        let index = pos.x + pos.y + (pos.x * rows);

        if seen.get(&index) != None {
            continue;
        }

        if pos.x == 0 && pos.y == 0 {
            if b[pos.x] == a[pos.y] {
                pos.points[0][2] = 0;
            } else {
                pos.points[0][2] = 2;
            }
        }

        // if b[pos.x] == a[pos.y] {
        //     println!("item:{} at x:{} y:{}", b[pos.x], pos.x, pos.y);
        // } else {
        //     println!("no item at x:{} y:{}", pos.x, pos.y);
        // }

        for dir in dirs {
            let x = pos.x + dir[0];
            let y = pos.y + dir[1];

            if dir[0] == 1 && dir[1] == 1 && x < rows && y < cols && b[x] != a[y] {
                continue;
            }

            if x < rows && y < cols {
                let mut points = Clone::clone(&pos.points);
                // let mut old_points = points.pop().unwrap();
                // old_points[2] = dir[2];
                // points.push(old_points);
                points.push([x, y, dir[2]]);

                queue.push_back(Node { x, y, points });
            }
        }

        seen.insert(index, true);

        // if 10 == i {
        //     break;
        // }

        if pos.x == rows - 1 && pos.y == cols - 1 {
            list.clone_from(&pos.points);
            println!("STOP, {:?}", pos.points);
            break;
        }

        // i += 1;
    }

    // for point in list.clone() {
    //     println!("{} {} {}", a[point[1]], b[point[0]], point[2])
    // }

    let top = list.remove(0);

    if a[top[1]] == b[top[0]] {
    } else {
        println!("{}-  {}{}", Tag::DEL.get_color(), a[top[1]], END);
        println!("{}+  {}{}", Tag::INS.get_color(), b[top[0]], END);
    }

    for point in list {
        let d_a = a[point[1]];
        let d_b = b[point[0]];
        let d_p = point[2];

        if d_p == 0 {
            println!("{}   {}{}", Tag::EQL.get_color(), d_a, END);
        } else if d_p == 1 {
            println!("{}-  {}{}", Tag::DEL.get_color(), d_a, END);
        } else {
            println!("{}+  {}{}", Tag::INS.get_color(), d_b, END);
            // println!("+{}", d_b);
        }

        // println!("{} {} {}", a[point[1]], b[point[0]], point[2])
    }
}

const END: &str = "\u{001b}[0m";

#[derive(Debug)]
enum Tag {
    INS,
    DEL,
    EQL,
}

impl Tag {
    fn to_str(&self) -> &str {
        match self {
            Tag::INS => "+",
            Tag::DEL => "-",
            Tag::EQL => " ",
        }
    }

    fn get_color(&self) -> &str {
        return match self {
            Tag::INS => "\u{001b}[38;5;158m\u{001b}[48;5;22m",
            Tag::DEL => "\u{001b}[38;5;217m\u{001b}[48;5;52m",
            Tag::EQL => "\u{001b}[39m",
        };
    }
}
