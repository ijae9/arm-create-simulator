use wasm_bindgen::prelude::*;
use rand::Rng;
use serde::Serialize;
use serde_wasm_bindgen;

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq)]
pub enum WeaponType {
    Sword = 1,
    Spear = 2,
    Axe = 3,
    Knuckle = 4,
    Bow = 5,
    MagicWand = 6,
    Knife = 7,
}

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq)]
pub enum EquipmentType {
    Weapon = 1,
    Armor = 2,
    Accessory = 3,
}

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq)]
pub enum ElementType {
    Fire = 1,
    Wind = 2,
    Star = 3,
    Thunder = 4,
    Water = 5,
    Light = 6,
    Dark = 7,
}

#[derive(Serialize, Debug)]
struct Equipment {
    damage: OptionWithRange,
    weight: OptionWithRange,
    price: OptionWithRange,
}

#[derive(Serialize, Debug)]
pub struct OptionWithRange {
    value: i32,
    min: i32,
    max: i32,
}


impl Equipment {
    fn new(damage: OptionWithRange, weight: OptionWithRange, price: OptionWithRange) -> Self {
        Equipment { damage, weight, price }
    }
}

#[wasm_bindgen]
pub fn make_town_equipment( equipment_type: EquipmentType, weapon_type: WeaponType, damage_price: i32, weigh_price: i32, weapon_element: ElementType,
                        creator_element: ElementType, contry_element: ElementType) -> JsValue {
    let mut bonus = 1.0;
    if weapon_element == creator_element {
        bonus += 0.2;
    } else if weapon_element == contry_element {
        bonus += 0.2;
    }
    let damage_price = (damage_price as f64 * bonus) as i32;
    let weigh_price = (weigh_price as f64 * bonus) as i32;
    
    let equipment = match equipment_type {
        // 추가로 무기 타입에 따른 처리도 필요함
        EquipmentType::Weapon | EquipmentType::Armor => { make_arm(damage_price, weigh_price, bonus, &weapon_type)},
        EquipmentType::Accessory => { make_acc(damage_price, weigh_price, bonus) }
    };

    serde_wasm_bindgen::to_value(&equipment).unwrap()
}

fn sqrt(x: i32) -> i32 {
    (x as f64).sqrt() as i32
}

fn make_arm(damage_price: i32, weigh_price: i32, bonus: f64, weapon_type: &WeaponType) -> Equipment {
    let (mut damage, mut min_damage, mut max_damage) = calc_arm_damage(damage_price, weigh_price, bonus);
    let mut weigh= calc_arm_weigh(damage, weigh_price, bonus);
    let mut min_weigh = calc_arm_weigh_vertax(min_damage, weigh_price, bonus, true);
    let mut max_weigh = calc_arm_weigh_vertax(max_damage, weigh_price, bonus, false);

    let (price, min_price, max_price) = calc_arm_price(damage, weigh);

    match weapon_type {
        WeaponType::Axe => { 
            damage = (damage as f64 * 1.15) as i32 ; weigh += 20; 
            min_damage = (min_damage as f64 * 1.15) as i32; max_damage = (max_damage as f64 * 1.15) as i32;
            min_weigh += 20; max_weigh += 20;
        },
        WeaponType::Knuckle => { 
            damage = (damage as f64 * 0.8) as i32 ; weigh -= 5; 
            min_damage = (min_damage as f64 * 0.8) as i32; max_damage = (max_damage as f64 * 0.8) as i32;
            min_weigh -= 5; max_weigh -= 5;
        
        },
        WeaponType::Bow => { 
            damage = (damage as f64 * 1.1) as i32 ; weigh += 12;
            min_damage = (min_damage as f64 * 1.1) as i32; max_damage = (max_damage as f64 * 1.1) as i32;
            min_weigh += 12; max_weigh += 12;
        },
        WeaponType::MagicWand => { 
            damage = (damage as f64 * 0.9) as i32 ; weigh -= 5;
            min_damage = (min_damage as f64 * 0.9) as i32; max_damage = (max_damage as f64 * 0.9) as i32;
            min_weigh -= 5; max_weigh -= 5;
        },
        WeaponType::Knife => { 
            damage = (damage as f64 * 0.95) as i32 ; weigh -= 3;
            min_damage = (min_damage as f64 * 0.95) as i32; max_damage = (max_damage as f64 * 0.95) as i32;
            min_weigh -= 3; max_weigh -= 3;
        },
        _ => {}
    }

    let damage = OptionWithRange { value: damage, min: min_damage, max: max_damage };
    let weight = OptionWithRange { value: weigh, min: min_weigh, max: max_weigh };
    let price = OptionWithRange { value: price, min: min_price, max: max_price };

    Equipment::new(damage, weight, price)
}

fn make_acc(damage_price: i32, weigh_price: i32, bonus: f64) -> Equipment{
    let (damage, min_damage, max_damage) = calc_acc_damage(damage_price, weigh_price, bonus);
    let weigh = calc_acc_weigh(damage, weigh_price, bonus);
    let min_weigh = calc_acc_weight_vertax(min_damage, weigh_price, bonus, true);
    let max_weigh = calc_acc_weight_vertax(max_damage, weigh_price, bonus, false);

    let (price, min_price, max_price) = calc_acc_price(damage, weigh);

    let damage = OptionWithRange { value: damage, min: min_damage, max: max_damage };
    let weight = OptionWithRange { value: weigh, min: min_weigh, max: max_weigh };
    let price = OptionWithRange { value: price, min: min_price, max: max_price };

    Equipment::new(damage, weight, price)
}

fn calc_acc_damage(damage_price: i32, weigh_price: i32, bonus: f64) -> (i32, i32, i32) {
    let mut rng = rand::thread_rng();
    
    let mut min_damage: i32 = (sqrt(damage_price) / 3) + 0 - (sqrt(weigh_price) / 5);
    let mut max_damage: i32 = (sqrt(damage_price) / 3) + sqrt(damage_price) - 0; 
    let mut acc_damage: i32 = (sqrt(damage_price) / 3) + rng.gen_range(0..(sqrt(damage_price))) - rng.gen_range(0..(sqrt(weigh_price) / 5)); 
    
    if acc_damage > 55 + (bonus * 50.0) as i32 {
        acc_damage = 55 + (bonus * 50.0) as i32
    }
    if max_damage > 55 + (bonus * 50.0) as i32 {
        max_damage = 55 + (bonus * 50.0) as i32
    }

    min_damage += rng.gen_range(0..((bonus * 20.0) as i32) + 3);
    max_damage += rng.gen_range(0..((bonus * 20.0) as i32) + 3);
    acc_damage += rng.gen_range(0..((bonus * 20.0) as i32) + 3);
    
    (acc_damage, min_damage, max_damage)
}

fn calc_acc_weigh(damage: i32, weigh_price: i32, bonus: f64) -> i32 {
    let mut rng = rand::thread_rng();
    
    let mut acc_weigh: i32;
    if damage < 50 {
        acc_weigh = damage - sqrt(weigh_price) - rng.gen_range(0..(sqrt(weigh_price) * 2));
        acc_weigh -= rng.gen_range(0..((bonus * 10.0) as i32) + 2);
        if acc_weigh < 0 {
            acc_weigh = 0 - rng.gen_range(0..10);
        }
    }
    else {
        acc_weigh = damage - (sqrt(weigh_price) / 2) - rng.gen_range(0..(sqrt(weigh_price)));
        acc_weigh -= rng.gen_range(0..((bonus * 10.0) as i32) + 2);
        if acc_weigh < 5 {
            acc_weigh = 5 - rng.gen_range(0..5);
        }
    }
    acc_weigh
}

fn calc_acc_weight_vertax(damage: i32, weigh_price: i32, bonus:f64, is_min: bool) -> i32 {    
    let mut acc_weigh: i32;
    if is_min {
        if damage < 50 {
            acc_weigh = damage - sqrt(weigh_price) - (sqrt(weigh_price) * 2);
            acc_weigh -= ((bonus * 10.0) as i32) + 2;
            if acc_weigh < 0 {
                acc_weigh = -10;
            }
        }
        else {
            acc_weigh = damage - (sqrt(weigh_price) / 2) - sqrt(weigh_price);
            acc_weigh -= ((bonus * 10.0) as i32) + 2;
            if acc_weigh < 5 {
                acc_weigh = 0;
            }
        }
    }
    else { // maximum
        if damage < 50 {
            acc_weigh = damage - sqrt(weigh_price);
            if acc_weigh < 0 {
                acc_weigh = 0;
            }
        }
        else {
            acc_weigh = damage - (sqrt(weigh_price) / 2);
            if acc_weigh < 5 {
                acc_weigh = 5;
            }
        }
    }
    acc_weigh
}

fn calc_acc_price(damage: i32, weigh: i32) -> (i32, i32, i32) {
    let mut sa = damage - weigh;
    if sa < 1 {
        sa = 1;
    }

    let mut price = (damage * damage * 2000) + (sa * sa * 4000);
    if damage < 20 {
        price = price / 10;
    }
    else if damage < 30 {
        price = price / 5;
    }
    else if damage < 40 {
        price = price / 3;
    }
    else if damage < 50 {
        price = price / 2;
    }
    else if damage < 60 {
        price = (price as f64 / 1.5) as i32;
    }
    
    let mut rng = rand::thread_rng();
    price= ((price as f64 * 0.8) + (rng.gen_range(0.0..(price as f64 * 0.4)))) as i32;
	let mut min_price = (price as f64 * 0.8) as i32;
    let mut max_price = (price as f64 * 1.2) as i32;
    
    if price < 1000 {
        price =1000;
    }
    if min_price < 1000 {
        min_price = 1000;
    }
    if max_price < 1000 {
        max_price = 1000;
    }

    (price, min_price, max_price)
}

fn calc_arm_damage(damage_price: i32, weigh_price: i32, bonus: f64) -> (i32, i32, i32) {
    let mut rng = rand::thread_rng();
    
    let mut arm_damage: i32 = sqrt(damage_price) + rng.gen_range(0..(sqrt(damage_price) * 4)) - rng.gen_range(0..(sqrt(weigh_price) / 2)); 
    let mut min_damage: i32 = sqrt(damage_price) + 0 - (sqrt(weigh_price) / 2);
    let mut max_damage: i32 = sqrt(damage_price) + (sqrt(damage_price) * 4) - 0;
    
    if arm_damage > 250 {
        arm_damage = 250;
    }
    if min_damage > 250 {
        min_damage = 250;
    }
    if max_damage > 250 {
        max_damage = 250;
    }

    arm_damage += rng.gen_range(0..((bonus * 50.0) as i32) + 5);
    max_damage += ((bonus * 50.0) as i32) + 5;

    let rand = rng.gen_range(0..10);

    if arm_damage < 10 + rand {
        arm_damage = 10 + rand;
    }
    if min_damage < 10 {
        min_damage = 10;
    }

    if max_damage < 20 {
        max_damage = 20;
    }

    (arm_damage, min_damage, max_damage)
}

fn calc_arm_weigh(damage: i32, weigh_price: i32, bonus: f64) -> i32 {
    let mut rng = rand::thread_rng();
    
    let mut arm_weigh: i32;
    if damage < 200 {
        arm_weigh = damage - sqrt(weigh_price) * 2 - rng.gen_range(0..(sqrt(weigh_price) * 5));
        if arm_weigh < 5 {
            arm_weigh = 5;
        }

        arm_weigh -= rng.gen_range(0..((bonus * 30.0) as i32) + 5);
        if arm_weigh < 5 {
            arm_weigh = 5 - rng.gen_range(0..10);
        }
    }
    else {
        arm_weigh = damage - sqrt(weigh_price) - rng.gen_range(0..(sqrt(weigh_price) * 3));
        if arm_weigh < 15 {
            arm_weigh = 15;
        }

        arm_weigh -= rng.gen_range(0..((bonus * 30.0) as i32) + 5);
        if arm_weigh < 15 {
            arm_weigh = 15 - rng.gen_range(0..10);
        }
    }
    arm_weigh
}

fn calc_arm_weigh_vertax(damage: i32, weigh_price: i32, bonus: f64, is_min: bool) -> i32 {
    let mut arm_weigh: i32;
    if is_min {
        if damage < 200 {
            arm_weigh = damage - sqrt(weigh_price) * 2 - (sqrt(weigh_price) * 5);
            if arm_weigh < 5 {
                arm_weigh = 5;
            }
            
            arm_weigh -= ((bonus * 30.0) as i32) + 5;
            if arm_weigh < 5 {
                arm_weigh = -5;
            }
        }
        else {
            arm_weigh = damage - sqrt(weigh_price) - sqrt(weigh_price) * 3;
            if arm_weigh < 15 {
                arm_weigh = 15;
            }

            arm_weigh -= ((bonus * 30.0) as i32) + 5;
            if arm_weigh < 15 {
                arm_weigh = 5;
            }
        }
    }
    else { // maximum
        if damage < 200 {
            arm_weigh = damage - sqrt(weigh_price) * 2;
            if arm_weigh < 5 {
                arm_weigh = 5;
            }
        }
        else {
            arm_weigh = damage - sqrt(weigh_price);
            if arm_weigh < 15 {
                arm_weigh = 15;
            }
        }
    }
    arm_weigh
}

fn calc_arm_price(damage: i32, weigh: i32) -> (i32, i32, i32) {
    let mut sa = damage - weigh;
    if sa < 1 {
        sa = 1;
    }

    let mut price = (damage * damage * 250) + (sa * sa * 500);
    if damage < 30 {
        price = price / 10;
    }
    else if damage < 50 {
        price = price / 5;
    }
    else if damage < 100 {
        price = price / 3;
    }
    else if damage < 150 {
        price = price / 2;
    }
    else if damage < 200 {
        price = (price as f64 / 1.5) as i32;
    }

    let mut rng = rand::thread_rng();

    price= ((price as f64 * 0.8) + (rng.gen_range(0.0..(price as f64 * 0.4)))) as i32;
	let mut min_price = (price as f64 * 0.8) as i32;
    let mut max_price = (price as f64 * 1.2) as i32;

    if price < 1000 {
        price = 1000;
    }
    if min_price < 1000 {
        min_price = 1000;
    }
    if max_price < 1000 {
        max_price = 1000;
    }

    (price, min_price, max_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_arm() {
        let damage_price = 300;
        let weigh_price = 600;
        let bonus = 1.4;
        let weapon_type = WeaponType::Axe;
        let equipment = make_arm(damage_price, weigh_price, bonus, &weapon_type);
        println!("make_arm: {:?}", equipment);
    }


    #[test]
    fn test_calc_acc_damage() {
        let damage_price = 100;
        let weigh_price = 50;
        let bonus = 1.2;
        let damage = calc_acc_damage(damage_price, weigh_price, bonus);
        println!("calc_acc_damage: {:?}", damage);
    }

    #[test]
    fn test_calc_acc_weigh() {
        let damage = 100;
        let weigh_price = 50;
        let bonus = 1.2;
        let weigh = calc_acc_weigh(damage, weigh_price, bonus);
        println!("calc_acc_weigh: {}", weigh);
    }

    #[test]
    fn test_calc_acc_price() {
        let damage = 15;
        let weigh = -1;
        let price = calc_acc_price(damage, weigh);
        println!("calc_acc_price: {:?}", price);
    }

    #[test]
    fn test_calc_arm_damage() {
        let damage_price = 300;
        let weigh_price = 600;
        let bonus = 1.4;
        let damage = calc_arm_damage(damage_price, weigh_price, bonus);
        println!("calc_arm_damage: {:?}", damage);
    }

    #[test]
    fn test_calc_arm_weigh() {
        let damage = 100;
        let weigh_price = 50;
        let bonus = 1.2;
        let weigh = calc_arm_weigh(damage, weigh_price, bonus);
        println!("calc_arm_weigh: {}", weigh);
    }

    #[test]
    fn test_calc_arm_price() {
        let damage = 111;
        let weigh = -4;
        let price = calc_arm_price(damage, weigh);
        println!("calc_arm_price: {:?}", price);
    }
}