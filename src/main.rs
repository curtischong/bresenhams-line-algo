use std::{thread, time};

fn render(arr: [[i32; 30]; 20]){
    for row in arr.iter().rev(){
        for val in row.iter(){
            if *val == 0 {
                print!(" ");
            }else{
                print!(".");
            }
        }
        println!("")
    }
}

struct Point{
    x: i32,
    y: i32 
}

// from https://www.geeksforgeeks.org/bresenhams-line-generation-algorithm/
fn main() {
    println!("Hello, world!");
    let mut arr: [[i32; 30]; 20] = [[0;30];20];
    let p1 = Point{x:1, y:5};
    let p2 = Point{x:25, y:0};
    let dx = (p1.x - p2.x).abs();
    let dy = (p1.y - p2.y).abs();

    let mut pk = (2 * dy) - dx;
    let mut x1 = p1.x;
    let mut x2 = p2.x;
    let mut y1 = p1.y;
    let mut y2 = p2.y;

    let decide = if dx > dy {0} else {1};

    for i in 0..dx{
        print! ("\x1B[2J\x1B[1;1H");
        x1 = if x1 < x2 { x1+1} else {x1-1};
        if pk < 0{
            if decide == 0{
                arr[y1 as usize][x1 as usize] = 1;
            }else{
                arr[x1 as usize][y1 as usize] = 1;
            }
            pk = pk + 2 *dy;
        }else{
            y1 = if y1 < y2 {y1 + 1} else{y1 - 1};
            if decide == 0{
                arr[y1 as usize][x1 as usize] = 1;
            }else{
                arr[x1 as usize][y1 as usize] = 1;
            }
            pk = pk + (2 *dy) - (2*dx);
        }
        render(arr);
        thread::sleep(time::Duration::from_millis(300));
    }
}