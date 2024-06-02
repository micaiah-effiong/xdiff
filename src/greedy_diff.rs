use crate::utils::utils::wrap;
use std::{isize, str, usize};

fn shortest_edit(a: &Vec<&str>, b: &Vec<&str>) -> Vec<Vec<isize>> {
    let m = a.len();
    let n = b.len();
    let max = m + n;

    let mut trace: Vec<Vec<isize>> = Vec::new();
    let mut v: Vec<isize> = vec![0; max * 2 + 1];
    v[1] = 0;

    for dd in 0..max {
        let d = dd as isize;
        trace.push(v.clone());

        for kk in (-d..d + 1).step_by(2) {
            let k = kk;
            let mut x: isize;
            // println!("checking {} {}", d, k);

            let kplus = wrap(v.len(), k + 1);
            let kminus = wrap(v.len(), k - 1);

            if k == -d || (k != d && v[kminus] < v[kplus]) {
                x = v[kplus];
            } else {
                x = v[kminus] + 1;
            }

            // println!("checking {} {} {} of {}", k, kplus, kminus, v.len());

            let mut y = x - k;

            // println!(
            //     "d is {}\n k is {} | x is {} from {} | y is {} from {}\n===",
            //     d, k, x, m, y, n
            // );
            while (x as usize) < m && (y as usize) < n && a[x as usize] == b[y as usize] {
                x = x + 1;
                y = y + 1;
            }

            let unsigned_k = wrap(v.len(), k.clone());

            v[unsigned_k] = x.clone();

            if x as usize >= m && y as usize >= n {
                println!("end = (d is {})", d);
                // trace.push(v.clone());
                return trace;
            }
        }
    }

    return trace;
}

fn backtrack(a: &Vec<&str>, b: &Vec<&str>, trace: Vec<Vec<isize>>) -> Vec<[isize; 4]> {
    let mut x = a.len();
    let mut y = b.len();

    let mut path: Vec<[isize; 4]> = Vec::with_capacity(trace.len());
    // path.push([x as isize, y as isize]);
    for dd in (0..trace.len()).rev() {
        println!("dd = {}", dd);
        // println!(
        //     "range {:?}, len {}",
        //     (0..trace.len()).rev().collect::<Vec<_>>(),
        //     trace.len()
        // );
    }
    for dd in (0..trace.len()).rev() {
        let v = &trace[dd];

        let k = x as isize - y as isize;
        let kplus = wrap(v.len(), k + 1);
        let kminus = wrap(v.len(), k - 1);
        let d = dd as isize;

        let prev_k: isize;
        let prev_x: isize;
        let prev_y: isize;

        // println!(
        //     "dd = {}, trace_len = {}, k = {}, v_len = {}",
        //     dd,
        //     trace.len(),
        //     k,
        //     v.len()
        // );

        if k == -d || (k != d && v[kminus] < v[kplus]) {
            prev_k = k + 1;
        } else {
            prev_k = k - 1;
        }

        prev_x = v[wrap(v.len(), prev_k)];
        prev_y = prev_x - prev_k;

        // println!(
        //     "b_trace d = {} \nx = {}, prev_x = {}, \ny = {}, prev_y = {} \nprev_k = {}\n",
        //     dd, x, prev_x, y, prev_y, prev_k
        // );
        while x as isize > prev_x && y as isize > prev_y {
            path.push([x as isize - 1, y as isize - 1, x as isize, y as isize]);
            x = x - 1;
            y = y - 1;
        }

        if d > 0 {
            path.push([prev_x, prev_y, x as isize, y as isize]);
        }

        x = prev_x as usize;
        y = prev_y as usize;
    }

    return path;
}

fn diff_parser(a: &Vec<&str>, b: &Vec<&str>, path: Vec<[isize; 4]>) -> Vec<Edit> {
    let mut diff: Vec<Edit> = Vec::new();

    println!("path_len {}", path.len());

    for index in (0..path.len()).rev() {
        let item = path[index];
        //
        // if index == 0 {
        //     break;
        // }

        let x = item[2];
        let y = item[3];
        let prev_x = item[0];
        let prev_y = item[1];

        println!("index {} {:?}", index, item);

        if x == prev_x {
            let b_line = b[prev_y as usize];
            let new_line = Line::new(b_line.to_owned(), prev_y as u64);
            diff.push(Edit::new(Tag::INS, None, Some(new_line)))
        } else if y == prev_y {
            let a_line = a[prev_x as usize];
            let old_line = Line::new(a_line.to_owned(), prev_x as u64);
            diff.push(Edit::new(Tag::DEL, Some(old_line), None))
        } else {
            let a_line = a[prev_x as usize];
            let b_line = b[prev_y as usize];
            let old_line = Line::new(a_line.to_owned(), prev_x as u64);
            let new_line = Line::new(b_line.to_owned(), prev_y as u64);
            diff.push(Edit::new(Tag::EQL, Some(old_line), Some(new_line)));
        }
    }

    return diff;
}

fn diff_printer(diff_path: Vec<Edit>) {
    for item in diff_path {
        item.print();
    }
}

pub fn diff(a: &Vec<&str>, b: &Vec<&str>) {
    let trace_value = shortest_edit(&a, &b);

    let mut trace: Vec<String> = Vec::new();

    for i in 0..trace_value.len() {
        trace.push(
            trace_value[i]
                .iter()
                .map(|&id| id.to_string() + " ")
                .collect(),
        );
    }

    // println!("{}", trace.join("\n"));

    let path = backtrack(&a, &b, trace_value);
    println!("path \n {:?}", path);

    let path_diff = diff_parser(&a, &b, path);

    // println!("{} hi ", Tag::DEL.get_color());
    diff_printer(path_diff);
    // println!("path_diff \n {} {:#?}", path_diff.len(), path_diff);
    //
}

struct Line {
    number: u64,
    text: String,
}

impl Line {
    pub fn new(text: String, number: u64) -> Line {
        return Line {
            number: number + 1,
            text,
        };
    }
}

#[derive(Debug)]
struct Edit {
    old_line: String,
    new_line: String,
    text: String,
    tag: Tag,
}

impl Edit {
    pub fn new(operation: Tag, old_line: Option<Line>, new_line: Option<Line>) -> Edit {
        if old_line.is_some() && new_line.is_some() {
            let old_edit = old_line.unwrap();
            let new_edit = new_line.unwrap();

            return Edit {
                old_line: old_edit.number.to_string(),
                new_line: new_edit.number.to_string(),
                text: old_edit.text,
                tag: operation,
            };
        } else if old_line.is_some() {
            let old_edit = old_line.unwrap();

            return Edit {
                old_line: old_edit.number.to_string(),
                new_line: " ".to_owned(),
                text: old_edit.text,
                tag: operation,
            };
        } else {
            let new_edit = new_line.unwrap();

            return Edit {
                old_line: " ".to_owned(),
                new_line: new_edit.number.to_string(),
                text: new_edit.text,
                tag: operation,
            };
        };
    }

    fn print(&self) {
        println!(
            "{}  {}  {}  {}  {}\u{001b}[0m",
            self.tag.get_color(),
            self.tag.to_str(),
            self.old_line,
            self.new_line,
            self.text
        );
    }
}

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
