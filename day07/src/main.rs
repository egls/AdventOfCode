use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

struct Node {
    name: String,
    children: Vec<Node>,
    files: Vec<String>,
}

fn main() {
    println!("Advent of code 2021, day 7!");

    let filename = "rsc/input2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;

   
    let mut currentNode = Node { name: "root".to_string(), children: vec![], files: vec![], };
    let mut root: Node;

    let mut ls_enabled = false;
    let mut dir_enabled = false;
    let mut cd_enabled = false;

    for (_idx, iter) in reader.lines().enumerate() {
        println!("{:?}", iter);

        let line = iter.unwrap();

        if line.contains("/") {
            println!("root found");
            // create root node                       
            ls_enabled = false;
            cd_enabled = false;
           
        } 
        
        if line.contains("$ ls") {
            print!("ls mode -> show");
            ls_enabled = true;
            cd_enabled = false;
        } 
        
        if line.contains("& cd") {
            println!("cd mode -> move");  
            cd_enabled = true;
            ls_enabled = false;
        }

        if ls_enabled {
            if line.contains("dir") {
                let mut split = line.split(" ").collect::<Vec<&str>>();
                let mut new_node = Node {
                    name: split[1].to_string(),
                    children: vec![],
                    files: vec![],
                };

                currentNode.children.push(new_node);
                
            }
        }


    }
    println!{"sum: {}", sum};
    println!{"current node: {:?}", currentNode.children[0].name};
}