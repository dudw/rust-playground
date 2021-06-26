use std::collections::HashMap;

fn plus(args:Vec<String>) -> String {
    let mut ivals:Vec<i32> = Vec::new();
    for val in args{
        let i_val= val.parse().unwrap();
        ivals.push(i_val);
    }
    ivals.iter().fold(0,|acc,&x|acc+x).to_string()
}

fn tokenize(lisp_code:&String) -> Vec<&str> {
    lisp_code.split_whitespace().collect()
}

fn main() {
    let lisp_code = "(+ 1 2 (+ 3 4))".to_string();
    let tokens = tokenize(&lisp_code);
    println!("tokens {:?}",tokens);

    let mut ops = HashMap::new();
    ops.insert(String::from("+"),plus);

    let mut stack:Vec<String> = Vec::new();
    for token_ref in tokens{
        let token = token_ref.to_string();
        if token == "("{
          // do nothing by design
        }else if token==")"{
            let mut sub_stack:Vec<String> = Vec::new();
            loop{
                let t:String = match stack.pop(){
                    None => break,
                    Some(t) => t
                };
                if t=="("{
                    continue;
                }
                sub_stack.push(t.clone());
                if ops.contains_key(&t){
                    break;
                }
            }

            let op = sub_stack.pop().unwrap();
            let mut ivals:Vec<i32> = Vec::new();
            for val in &sub_stack{
                let i_val= val.parse().unwrap();
                ivals.push(i_val);
            }

            ops.get(&op).unwrap()(sub_stack);

        } else{
            stack.push(token);
        }
    }
}
