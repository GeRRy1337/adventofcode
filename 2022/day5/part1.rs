use std::{fs};

fn main() {

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/2022/day5/day5.txt").expect("");
    let mut matrix=[[' '; 9]; 30];
    let mut db=0;
    for line in contents.lines(){
        if db<=7 {            
            let mut ind=0;
            for c in line.chars() {
                matrix[7-db][ind]=c;
                ind+=1;
            }
            db+=1;
            write(&matrix);
        }else{
            let curr:String=line.split("move").collect();
            let curr:String=curr.split("from").collect();
            let curr:String=curr.split("to").collect();
            let mut curr=curr.split(" ");
            curr.next();
            let mut moves:i32=curr.next().unwrap().parse().unwrap();
            curr.next();
            let from:usize=curr.next().unwrap().parse().unwrap();
            curr.next();
            let to:usize=curr.next().unwrap().parse().unwrap();
            loop{
                if moves==0 {
                    break;
                }
                let mut ind=29;
                loop{
                    if matrix[ind][from-1] != ' ' {
                        break;
                    }
                    if ind==0 {
                        break;
                    }
                    
                    ind-=1;
                }
                let mc:char=matrix[ind][from-1];
                matrix[ind][from-1]=' ';
                if mc != ' ' {
                    for i in (0..30).rev(){
                        if matrix[i][to-1] != ' ' {
                            write(&matrix);
                            println!("ind:{}",ind);
                            matrix[i+1][to-1]=mc;
                            break;
                        }else if i==0 {
                            matrix[0][to-1]=mc;
                        }
                    }
                }
                moves-=1;
            }
        }
    }
    write(&matrix);
    for y in 0..9 {
            for i in 0..30 {
            if matrix[i][y]==' ' {
                print!("{}",matrix[i-1][y]);
                break;
            }
        }
    }
}

fn write(matrix:&[[char;9];30]){
    println!("");
    for col in (0..30).rev() {
        if col < 9 {
            print!("col( {}): ",col+1);
        }else{
            print!("col({}): ",col+1);
        }
        for row in 0..9 {
            print!("[{}] ", matrix[col][row]);
        }
        println!("");
    }
    println!("          1   2   3   4   5   6   7   8   9\n");
}