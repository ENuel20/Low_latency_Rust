fn move_between(from: &mut Vec<u32>, to: &mut Vec<u32>, from_name: char, to_name: char) {
    match (from.last(), to.last()) {
        (None, None) => return,
        // from empty → move from 'to' → 'from'
        (None, Some(_)) => {
            let disk = to.pop().unwrap();
            from.push(disk);
            println!("Move disk {} from {} to {}", disk, to_name, from_name);
        }

        // to empty → move from 'from' → 'to'
        (Some(_), None) => {
            let disk = from.pop().unwrap();
            to.push(disk);
            println!("Move disk {} from {} to {}", disk, from_name, to_name);
        }

        // both have disks → move smaller on top of larger
        (Some(&f), Some(&t)) => {
            if f < t {
                let disk = from.pop().unwrap();
                to.push(disk);
                println!("Move disk {} from {} to {}", disk, from_name, to_name);
            } else {
                let disk = to.pop().unwrap();
                from.push(disk);
                println!("Move disk {} from {} to {}", disk, to_name, from_name);
            }
        }
    }
}

fn tower_of_hanoi_iterative(n: usize) {
    let total_moves = (1 << n) - 1;

    let mut src: Vec<u32> = (1..=n as u32).rev().collect();
    let mut aux: Vec<u32> = Vec::new();
    let mut dest: Vec<u32> = Vec::new();

    let (mut s_name, mut a_name, mut d_name) = ('A', 'B', 'C');

    // Even n → swap destination and auxiliary
    if n % 2 == 0 {
        std::mem::swap(&mut aux, &mut dest);
        std::mem::swap(&mut a_name, &mut d_name);
    }

    for i in 1..=total_moves {
        match i % 3 {
            1 => move_between(&mut src, &mut dest, s_name, d_name),
            2 => move_between(&mut src, &mut aux, s_name, a_name),
            _ => move_between(&mut aux, &mut dest, a_name, d_name),
        }
    }
}

fn main() {
    let n = 3;
    println!("Iterative Tower of Hanoi with {} disks:\n", n);
    tower_of_hanoi_iterative(n);
}
