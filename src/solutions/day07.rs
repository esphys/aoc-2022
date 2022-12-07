use std::collections::HashMap;
pub fn part_one() -> usize{

  let mut path: String = "".to_string();
  let mut paths: HashMap<String, usize> = HashMap::new();

  include_str!("inputs/day07.txt")
    .lines().for_each(|line| {
      let mut args = line.split(' ');
      let arg_1 = args.next().unwrap();
      match arg_1 {
        "$" => {
          match args.next() {
            Some("cd") => {
              let dir_name = args.next().unwrap();
              match dir_name {
                "/" => {
                  path = dir_name.to_string()
                }
                ".." => {
                  // println!("moving up");
                  
                  
                  // if path == "" {path = "/".to_string()};
                  path.pop();
                  if path != "/" && path != "" {
                    path = path.rsplit_once('/').unwrap().0.to_string();
                  }

                  if path == "" {
                    path = "/".to_string();
                  } else {
                    path.push('/');
                  }

                  
                  
                  
                },
                _ => {
                  // println!("delving further");
                  path.push_str(&(dir_name.to_owned()+"/"))
                },
              }
            },
            Some("ls") => {
              // println!("listing contents");
            },
            _ => println!("unknown command")
          }
        },
        "dir" => {
          // println!("found directory");
        },
        _ => {
          // println!("file found");

          let mut tmp_path = path.clone();
// /a/
          while tmp_path != "" {
            let mut current_size = 0;

            if tmp_path != "/"{
              tmp_path = tmp_path.rsplit_once('/').unwrap().0.to_string();
            // cascade the sizes up the hashmap indexes
            }

            if paths.contains_key(&tmp_path){
              current_size = *paths.get(&tmp_path).unwrap();
            }

            paths.insert(tmp_path.clone(), arg_1.parse::<usize>().unwrap() + current_size);

            tmp_path.pop();
          }
        },
      }
      
      // println!("path: {}", path);
        
    });
    // println!("{:#?}", paths.iter());

    paths.retain(|_, v| *v <= 100000);
    paths.values().sum()
           
}


pub fn part_two() -> u64 {

  let mut path: String = "".to_string();
  let mut paths: HashMap<String, u64> = HashMap::new();

  include_str!("inputs/day07.txt")
    .lines().for_each(|line| {
      let mut args = line.split(' ');
      let arg_1 = args.next().unwrap();
      match arg_1 {
        "$" => {
          match args.next() {
            Some("cd") => {
              let dir_name = args.next().unwrap();
              match dir_name {
                "/" => {
                  path = dir_name.to_string()
                }
                ".." => {
                  // println!("moving up");
                  
                  
                  // if path == "" {path = "/".to_string()};
                  path.pop();
                  if path != "/" && path != "" {
                    path = path.rsplit_once('/').unwrap().0.to_string();
                  }

                  if path == "" {
                    path = "/".to_string();
                  } else {
                    path.push('/');
                  }

                  
                  
                  
                },
                _ => {
                  // println!("delving further");
                  path.push_str(&(dir_name.to_owned()+"/"))
                },
              }
            },
            Some("ls") => {
              // println!("listing contents");
            },
            _ => println!("unknown command")
          }
        },
        "dir" => {
          // println!("found directory");
        },
        _ => {
          // println!("file found");

          let mut tmp_path = path.clone();
          // /a/
          while tmp_path != "" {
            let mut current_size = 0;

            if tmp_path != "/"{
              tmp_path = tmp_path.rsplit_once('/').unwrap().0.to_string();
            // cascade the sizes up the hashmap indexes
            }
            if tmp_path != "" {
              if paths.contains_key(&tmp_path){
                current_size = *paths.get(&tmp_path).unwrap();
              }
            } else {
              if paths.contains_key("/"){
                current_size = *paths.get("/").unwrap();
              }
            }
            
            if tmp_path != "" {
              paths.insert(tmp_path.clone(), arg_1.parse::<u64>().unwrap() + current_size);
            } else {
              paths.insert("/".to_string(), arg_1.parse::<u64>().unwrap() + current_size);
            }
            

            tmp_path.pop();
          }
        },
      }
    });

    let root_size = paths.get("/").unwrap();
    let unused_space = 70000000 - root_size;
    let required_space = 30000000 - unused_space;

    paths.values().fold( u64::MAX, |a, b| {
      if b < &a && *b >= required_space {
        *b
      } else {
        a
      }
    })
}