use num_enum::FromPrimitive;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AbilityFilterEnum {
    Any = 3,
    Ability0 = 0,
    Ability1 = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum AbilityEnum {
    #[num_enum(default)]
    Ability0 = 0,
    Ability1 = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NatureFilterEnum {
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,
    Any = 25,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum NatureEnum {
    #[num_enum(default)]
    Hardy = 0,
    Lonely = 1,
    Brave = 2,
    Adamant = 3,
    Naughty = 4,
    Bold = 5,
    Docile = 6,
    Relaxed = 7,
    Impish = 8,
    Lax = 9,
    Timid = 10,
    Hasty = 11,
    Serious = 12,
    Jolly = 13,
    Naive = 14,
    Modest = 15,
    Mild = 16,
    Quiet = 17,
    Bashful = 18,
    Rash = 19,
    Calm = 20,
    Gentle = 21,
    Sassy = 22,
    Careful = 23,
    Quirky = 24,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShinyFilterEnum {
    None = 0,
    Star = 1,
    Square = 2,
    Any = 3,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EncounterFilterEnum {
    Static = 0,
    Dynamic = 1,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShinyEnum {
    None = 0,
    Star = 1,
    Square = 2,
}
