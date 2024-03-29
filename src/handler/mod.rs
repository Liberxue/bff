use serde_json::{Value}; 


pub fn update_json(json: &mut Value, update:&Value) {
    if let Some(update) = update.as_object(){
        for (key,value) in update {
            json[key] = value.clone();
        }
    }
}

pub fn add_json(json: &mut Value,add:&Value) {
    if let Some(add) = add.as_object() {
     for (key, value )  in add {
        json[key].as_object_mut().unwrap().
                extend(value.as_object().unwrap().
                    clone());
        }
    }
}

pub fn del_json(json:&mut Value, delete: &str) {
    let keys: Vec<&str> = delete.split(".").collect();
    let mut current = json;
    for key in keys[..keys.len()-1].iter(){
        current=current.get_mut(&key).unwrap();
    }
    current.as_object_mut().unwrap().
    remove(keys[keys.len()-1]);
}


pub fn action(json_str:&str,config_str:&str) -> String{
    let mut  json_value: Value = serde_json::from_str(json_str).unwrap();

    let conf: Value = serde_json::from_str(config_str).unwrap();


    println!("source json: \n{}\n",json_value);

    println!("conf json:\n{}\n",conf);

    
    update_json(&mut json_value, &conf["update"]);

    let new_json_str = serde_json::to_string(&json_value).unwrap();

    println!("update:\n{}\n",new_json_str);

    add_json(&mut json_value, &conf["add"]);

    let new_json_str = serde_json::to_string(&json_value).unwrap();

    println!("add:\n{}\n",new_json_str);

    for del in conf["delete"].as_array().unwrap(){
        del_json(&mut json_value,del.as_str().unwrap())
    }
    let new_json_str = serde_json::to_string(&json_value).unwrap();

    println!("del:\n{}\n",new_json_str);

    return new_json_str;
}