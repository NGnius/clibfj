use libc;
use std::ffi::CStr;
use std::os::raw::c_char;
use base64::{decode_config_buf, STANDARD};
use libfj::robocraft_simple as robocraft;
use libfj::robocraft::{FactoryRobotGetInfo, Cube, Cubes};

use crate::allocate_cstring;

// C-style struct (for interop)

#[repr(C)]
pub struct FactoryRobotGetInfoC {
    pub item_id: u32,
    pub item_name: *mut c_char,
    pub item_description: *mut c_char,
    pub thumbnail: *mut c_char, // url
    pub added_by: *mut c_char,
    pub added_by_display_name: *mut c_char,
    pub added_date: *mut c_char, // ISO date
    pub expiry_date: *mut c_char, // ISO date
    pub cpu: u32,
    pub total_robot_ranking: u32,
    pub rent_count: u32,
    pub buy_count: u32,
    pub buyable: bool, // bool
    pub removed_date: *mut c_char,
    pub ban_date: *mut c_char,
    pub featured: bool, // bool
    pub banner_message: *mut c_char,
    pub combat_rating: f32,
    pub cosmetic_rating: f32,
    pub cube_data: *mut c_char,
    pub colour_data: *mut c_char,
    pub cube_amounts: *mut c_char, // JSON as str
}

impl FactoryRobotGetInfoC {
    fn mock_error(msg: &str, url: &str) -> Self {
        let mut err = Self::default();
        err.item_name = unsafe {allocate_cstring(msg)};
        err.thumbnail = unsafe {allocate_cstring(url)};
        err.added_by_display_name = unsafe {allocate_cstring("ERROR")};
        err.added_by = unsafe {allocate_cstring("ERROR")};
        err.item_id = u32::MAX;
        err
    }
}


impl Default for FactoryRobotGetInfoC {
    fn default() -> Self {
        Self {
            item_id: 0,
            item_name: unsafe {allocate_cstring("")},
            item_description: unsafe {allocate_cstring("")},
            thumbnail: unsafe {allocate_cstring("")},
            added_by: unsafe {allocate_cstring("")},
            added_by_display_name: unsafe {allocate_cstring("")},
            added_date: unsafe {allocate_cstring("")},
            expiry_date: unsafe {allocate_cstring("")},
            cpu: 0,
            total_robot_ranking: 0,
            rent_count: 0,
            buy_count: 0,
            buyable: false,
            removed_date: unsafe {allocate_cstring("")},
            ban_date: unsafe {allocate_cstring("")},
            featured: false,
            banner_message: unsafe {allocate_cstring("")},
            combat_rating: 0.0,
            cosmetic_rating: 0.0,
            cube_data: unsafe {allocate_cstring("")},
            colour_data: unsafe {allocate_cstring("")},
            cube_amounts: unsafe {allocate_cstring("")},
        }
    }
}

impl From<FactoryRobotGetInfo> for FactoryRobotGetInfoC {
    fn from(frli: FactoryRobotGetInfo) -> Self {
        let def = "".to_string();
        Self {
            item_id: frli.item_id as u32,
            item_name: unsafe {allocate_cstring(&frli.item_name)},
            item_description: unsafe {allocate_cstring(&frli.item_description)},
            thumbnail: unsafe {allocate_cstring(&frli.thumbnail)},
            added_by: unsafe {allocate_cstring(&frli.added_by)},
            added_by_display_name: unsafe {allocate_cstring(&frli.added_by_display_name)},
            added_date: unsafe {allocate_cstring(&frli.added_date)},
            expiry_date: unsafe {allocate_cstring(&frli.expiry_date)},
            cpu: frli.cpu  as u32,
            total_robot_ranking: frli.total_robot_ranking  as u32,
            rent_count: frli.rent_count  as u32,
            buy_count: frli.buy_count as u32,
            buyable: frli.buyable,
            removed_date: unsafe {allocate_cstring(&frli.removed_date.unwrap_or(def.clone()))},
            ban_date: unsafe {allocate_cstring(&frli.ban_date.unwrap_or(def.clone()))},
            featured: frli.featured,
            banner_message: unsafe {allocate_cstring(&frli.banner_message.unwrap_or(def.clone()))},
            combat_rating: frli.combat_rating,
            cosmetic_rating: frli.cosmetic_rating,
            cube_data: unsafe {allocate_cstring(&frli.cube_data)},
            colour_data: unsafe {allocate_cstring(&frli.colour_data)},
            cube_amounts: unsafe {allocate_cstring(&frli.cube_amounts)},
        }
    }
}

#[repr(C)]
pub struct CubeC {
    pub id: u32,
    pub x: u8, // left to right
    pub y: u8, // bottom to top
    pub z: u8, // back to front
    pub orientation: u8,
    pub colour: u8,
}

impl From<Cube> for CubeC {
    fn from(c: Cube) -> Self {
        Self {
            id: c.id,
            x: c.x,
            y: c.y,
            z: c.z,
            orientation: c.orientation,
            colour: c.colour,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libfj_factory_robot(item_id: u32, result_out: *mut FactoryRobotGetInfoC) {
    let factory_api = robocraft::FactoryAPI::new();
    let result = factory_api.get(item_id as usize);
    if let Ok(info) = result {
        *result_out = info.response.into();
    } else if let Err(e) = result {
        *result_out = FactoryRobotGetInfoC::mock_error(
            &format!("{}", &e),
            ""
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn libfj_factory_robot_cubes(items: libc::c_uint, array_ptr: *mut CubeC, info: *const FactoryRobotGetInfoC) {
    libfj_factory_robot_cubes_raw(items, array_ptr, (*info).cube_data, (*info).colour_data)
}

#[no_mangle]
pub unsafe extern "C" fn libfj_factory_robot_cubes_raw(items: libc::c_uint, array_ptr: *mut CubeC, cube_data: *const c_char, colour_data: *const c_char) {
    let cube_str = CStr::from_ptr(cube_data).to_str().unwrap();
    let colour_str = CStr::from_ptr(colour_data).to_str().unwrap();
    let mut cube_buf = Vec::new();
    let mut colour_buf = Vec::new();
    decode_config_buf(cube_str, STANDARD, &mut cube_buf).unwrap();
    decode_config_buf(colour_str, STANDARD, &mut colour_buf).unwrap();
    let cubes_res = Cubes::parse(&mut cube_buf, &mut colour_buf);
    if let Ok(cubes) = cubes_res {
        // convert to C-compatible format and copy into return array
        let max = if cubes.len() < (items as usize) { cubes.len() } else {items as usize};
        let mut iter = cubes.into_iter();
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        // copy results to output array
        for i in 0..max {
            if let Some(c) = iter.next() {
                c_result[i] = c.clone().into();
            } else {
                // TODO report error somehow?
            }
        }
    } else {
        // TODO report error somehow?
    }
}
