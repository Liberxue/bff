extern  crate serde_json;
extern  crate hyper;
use serde_json::{Value};
fn main() {
    let json_str = r#"
    {
        "name": "Test BFF",
        "age": 30,
        "liber": "000",
        "sex":18,
        "address": {
            "street": "123 Main St",
            "city": "Shanghai",
            "data1":{
                "Liber":"1234",
                "Liber123":"1234"
            }
        }
    }
    "#;

    let config_str = r#"
    {
        "update": {
            "age": 80,
            "liber": "111",
            "data3":{
                "key1":"value1"
                }
        },
        "add": {
            "address": {
                "zipcode": "94107",
                "liber": "111",
                "liber222222": "111",
                "data":[
                    {
                        "math_1":"1111",
                            "englist_1":"5555"
                        },
                        {
                            "math_2":"123",
                            "englist_2":"456"
                        }
                    ]
                }
        },
        "delete":[
            "address.data1.Liber123",
            "address.data1.Liber",
            "address.data1"
        ]
    }
    "#;
    let result = action(json_str,config_str);
    println!("result {}",result);
}

fn update_json(json: &mut Value, update:&Value) {
    if let Some(update) = update.as_object(){
        for (key,value) in update {
            json[key] = value.clone();
        }
    }
}

fn add_json(json: &mut Value,add:&Value) {
    if let Some(add) = add.as_object() {
     for (key, value )  in add {
        json[key].as_object_mut().unwrap().
                extend(value.as_object().unwrap().
                    clone());
        }
    }
}

fn del_json(json:&mut Value, delete: &str) {
    let keys: Vec<&str> = delete.split(".").collect();
    let mut current = json;
    for key in keys[..keys.len()-1].iter(){
        current=current.get_mut(&key).unwrap();
    }
    current.as_object_mut().unwrap().
        remove(keys[keys.len()-1]);
}


fn action(json_str:&str,config_str:&str) -> String{
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
