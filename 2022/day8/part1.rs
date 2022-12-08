use std::{fs};

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day8/day8.txt").unwrap();
    let mut db=0;
    let mut arr:Vec<Vec<i32>>=Vec::new();
    let mut up:bool;
    let mut down:bool;
    let mut left:bool;
    let mut right:bool;
    for line in contents.lines(){
        let mut temp:Vec<i32>=Vec::new();
        for c in line.chars() {
            temp.push(c as i32);
        }
        arr.push(temp);
    }

    for x in 0..arr.len(){
        for y in 0..arr[x].len() {
            up=false;down=false;left=false;right=false;
            if x>0 && x<arr.len()-1 && y>0 && y < arr[x].len()-1{

                for i in (x+1)..arr.len() {
                    if arr[x][y] <= arr[i][y] {
                        down=true;
                        break;
                    }
                }

                for i in (0..=(x-1) as usize).rev(){
                    if arr[x][y] <= arr[i][y] {
                        up=true;
                        break;
                    }
                }

                for i in (y+1)..arr[x].len(){
                    if arr[x][y] <= arr[x][i] {
                        right=true;
                        break;
                    } 
                }

                for i in (0..=(y-1)).rev() {
                    if arr[x][y] <= arr[x][i] {
                        left=true;
                        break;
                    } 
                }
            }
            if !up || !down || !left || !right {
                db+=1;
            }
        }
    }
    
    println!("db: {}", db);
}