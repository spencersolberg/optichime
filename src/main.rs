use std::env;
use std::fs;
// use std::fs::File;
// use std::io::prelude::*;
use std::collections::HashMap;
use serde_json::json;
use std::path::Path;
// use std::ffi::OsStr;
use walkdir::WalkDir;

#[derive (Debug)]
struct Item {
    parent: String,
    texture: String,
    overrides: Vec<Override>,
    name: String
}


#[derive (Debug, Clone)]
struct Override {
    model: String,
    predicate: Predicate
}

#[derive (Debug, Clone)]
enum Predicate {
    Name(String),
    Nbt(String, String),
    Enchantments(String)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let optifine_pack_path = String::from(args[1].trim());
    let optifine_pack_name = Path::new(&optifine_pack_path).file_name().expect("error reading path").to_string_lossy();
    let chime_pack_path = format!("output/{}_CHIME", &optifine_pack_name);

    if !fs::metadata(&optifine_pack_path).expect("Error reading Optifine Pack Path").is_dir() {
        panic!("You have to unzip the pack first, sorry.")
    }

    let mut file_paths: Vec<String> = Vec::new();

    for entry in WalkDir::new(&optifine_pack_path) {
        // println!("{}", entry.expect("Error reading entry").path().display());
        let entry_path: String = entry.expect("Error reading entry").path().to_str().expect("Error converting to str").to_string();

        if entry_path.ends_with(".properties") {
            file_paths.push(entry_path);
        }
    }


    let mut items: Vec<Item> = Vec::new();
    // let mut new_items: Vec<Item> = Vec::new();
    // let mut index = 0;
    for path in file_paths {
        convert_optifine_model_to_item_struct(&path, &mut items);
        
    }

    

    // items.push(convert_optifine_model_to_item_struct(&String::from("input/Purple Tools/assets/minecraft/optifine/cit/items/purple_diamond_sword/purple_diamond_sword.properties")));

    // println!("Name: {}\nParent: {}\nTexture: {}\nOverride: {{model: {}}}", items[0].name, items[0].parent, items[0].texture, items[0].overrides[0].model);

    // println!("{:?}", items[0]);


    let mut original_models: Vec<String> = Vec::new();
    let mut override_models: Vec<String> = Vec::new();
    for item in items {
        let original_model = serialize_item_struct_to_original_model(&item);
        let override_models2 = serialize_item_struct_to_override_model(&item); //r#override_models was causing errors, so I used override_models2 instead...

        original_models.push(serde_json::to_string(&original_model).expect("Error Formatting Model"));

        for override_model in override_models2 {
            override_models.push(serde_json::to_string(&override_model).expect("Error Formatting Model"))
        }
    }

    // println!("Original Models:\n{:#?}\n\n\n\nOverride Models:\n{:#?}", original_models, override_models);

    fs::create_dir_all(format!("{}/assets/minecraft", &chime_pack_path)).expect("Error creating directory for Chime pack");
    fs::create_dir_all(format!("{}/assets/minecraft/models/item", &chime_pack_path)).expect("Error creating directory for models");
    fs::create_dir_all(format!("{}/assets/minecraft/textures/item", &chime_pack_path)).expect("Error creating directory for textures");

    for model in original_models {
        let model_value = serde_json::from_str::<serde_json::Value>(&model.as_str()).expect("Error converting model to Value");
        let model_name: String = model_value["textures"]["layer0"].to_string().replace("minecraft:item/", "").replace("\"", "");
        println!("Model: {}", model_name);
        fs::write(format!("{}/assets/minecraft/models/item/{}.json", &chime_pack_path, &model_name), &model).expect("Error writing model to file");
    }

    for model in override_models {
        let model_value = serde_json::from_str::<serde_json::Value>(&model.as_str()).expect("Error converting model to Value");
        let model_name: String = model_value["textures"]["layer0"].to_string().replace("item/", "").replace("\"", "");
        println!("Model: {}", model_name);
        fs::write(format!("{}/assets/minecraft/models/item/{}.json", &chime_pack_path, &model_name), &model).expect("Error writing model to file");
    }

    fs::copy(format!("{}/pack.mcmeta", optifine_pack_path), format!("{}/pack.mcmeta", chime_pack_path)).expect("Error copying pack.mcmeta");
    fs::copy(format!("{}/pack.png", optifine_pack_path), format!("{}/pack.png", chime_pack_path)).expect("Error copying pack.png");
    
    for entry in WalkDir::new(&optifine_pack_path) {
        // println!("{}", entry.expect("Error reading entry").path().display());
        let entry_path: String = entry.expect("Error reading entry").path().to_str().expect("Error converting to str").to_string();
        let entry_name: String = Path::new(&entry_path).file_name().expect("error reading path").to_string_lossy().to_string();
        if entry_path.ends_with(".png") {
            fs::copy(entry_path, format!("{}/assets/minecraft/textures/item/{}", chime_pack_path, entry_name)).expect("Error copying item texture");
        };
    }

    ();
}

fn convert_optifine_model_to_item_struct(path: &String, items: &mut Vec<Item>) -> () {
    let file: String = fs::read_to_string(path).expect("Error reading file");
    // let mut texture = String::new();

    // println!("{}", file);

    // let mut i = 0;

    let mut properties: HashMap<String, String> = HashMap::new();

    for line in file.split('\n') {
        // collecting key and value pairs from optifine properties
        let mut split_by_equals = line.split('=').collect::<Vec<&str>>();

        let key = split_by_equals[0].to_string();
        split_by_equals.remove(0);

        let val = split_by_equals.join("=").to_string(); // joins the rest of the line back together, in case it had a '='. Probably redundant because I don't think I understand the .split() thing I'm using

        properties.insert(key, val);

        // if line.split('=').collect::<Vec<&str>>()[0] == "items" {texture = line.split('=').collect::<Vec<&str>>()[1].trim()}

        // i += 1;
        // println!("Line {}: {}", i, line);
    };

    // println!("{:?}", properties);

    let name = Path::new(&path).file_name().expect("error reading path").to_string_lossy().replace(".properties", "");

    let mut predicate: Predicate = Predicate::Name("Placeholder".to_string()); // what am i doing

    for (key, val) in &properties {
        println!("{}: {}", key, val);

        
        if key.contains("nbt.display.Name")  { 
            predicate = Predicate::Name(val.to_string()); 
        } else 
        if key.contains("enchantments") {
            predicate = Predicate::Enchantments(val.to_string());
        } else
        if key.contains("nbt") {
            predicate = Predicate::Nbt(key.to_string(), val.to_string());
        }

    }

    let override2 = Override {
        model: String::from(format!("item/{}", name)),
        predicate: predicate
    };

    let mut publish_new_item: bool = true;
    
    for item in &mut*items {
        if item.texture == String::from(format!("minecraft:item/{}", properties.get("items").expect("Property could not be registered").trim())) {
            item.overrides.push(override2.clone());
            publish_new_item = false;
        }
    }

    if publish_new_item {
        &items.push(Item {
            parent: String::from("minecraft:item/generated"),
            texture: String::from(format!("minecraft:item/{}", properties.get("items").expect("Property could not be registered").replace("minecraft:", "").replace("\r", ""))),
            overrides: vec![override2],
            name: name.to_string().replace("minecraft:", "").replace("\\r", "")
        });
    }
    
    ();
}

fn serialize_item_struct_to_original_model(item: &Item) -> serde_json::Value {
    let mut overrides_vec: Vec<serde_json::Value> = Vec::new();
    for r#override in &item.overrides {
        // println!("Item ID: {}", item.texture);
        let predicate = match &r#override.predicate {
            Predicate::Name(name) => json!({"name": name}),
            Predicate::Nbt(key, val) => convert_nbt_string_to_value(key.to_string(), val.to_string()),
            Predicate::Enchantments(list) => convert_enchantments_to_value(list.to_string(), item.texture.to_string())
        } ;

        overrides_vec.push(json!({"predicate": predicate, "model": r#override.model}));
    }

    let item_json = json!({
        "parent": &item.parent,
        "textures": {
            "layer0": &item.texture
        },
        "overrides": &overrides_vec
    });



    item_json
}

fn serialize_item_struct_to_override_model (item: &Item) -> Vec<serde_json::Value> {
    let mut model_json_vec: Vec<serde_json::Value> = vec![]; 

    for r#override in &item.overrides {
        model_json_vec.push(json!({
            "parent": "item/generated",
            "textures": {
                "layer0": format!("item/{}", r#override.model.replace("item/", ""))
            }
        }))
    }

    model_json_vec
}

fn convert_nbt_string_to_value (key: String, val: String) -> serde_json::Value {
    let dots = key.split(".").collect::<Vec<&str>>();
    
    let mut final_nbt: serde_json::Value = json!({});

    let mut store_value = true;
    for part in dots.iter().rev() {
        if store_value {
            final_nbt = json!({part.to_string(): &val.trim()});
            store_value = false;
        } else {
            final_nbt = json!({part.to_string(): final_nbt});
        }
    }

    final_nbt
}

fn convert_enchantments_to_value(list: String, id: String) -> serde_json::Value {
    let enchants: Vec<&str> = list.split_whitespace().collect();
    let mut enchants_value: Vec<serde_json::Value> = Vec::new();

    for enchant in enchants {
        enchants_value.push(json!({"id": format!("minecraft:{}", enchant)}));
    }

    if id == String::from("minecraft:item/enchanted_book") {
        json!({"nbt":{"StoredEnchantments": enchants_value}})
    } else {
        json!({"nbt":{"Enchantments": enchants_value}})
    }

}
