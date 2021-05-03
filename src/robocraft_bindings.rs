use libc;
use std::ffi::CString;
use std::os::raw::c_char;
use libfj::robocraft_simple as robocraft;
use libfj::robocraft::FactoryRobotListInfo;

#[repr(C)]
pub struct FactoryRobotListInfoC {
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
    pub cube_amounts: *mut c_char, // JSON as str
}

impl FactoryRobotListInfoC {
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

impl Default for FactoryRobotListInfoC {
    fn default() -> Self {
        FactoryRobotListInfoC {
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
            cube_amounts: unsafe {allocate_cstring("")},
        }
    }
}

impl From<FactoryRobotListInfo> for FactoryRobotListInfoC {
    fn from(frli: FactoryRobotListInfo) -> Self {
        let def = "".to_string();
        FactoryRobotListInfoC {
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
            cube_amounts: unsafe {allocate_cstring(&frli.cube_amounts)},
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_factory_front_page(items: libc::c_uint, array_ptr: *mut FactoryRobotListInfoC) {
    if items == 0 {return;} // nothing to populate, so it's useless to do work
    let factory_api = robocraft::FactoryAPI::new();
    let result = factory_api.list();
    if let Ok(info) = result {
        let max = if info.response.roboshop_items.len() < (items as usize) { info.response.roboshop_items.len() } else {items as usize};
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        // copy results to output array
        for i in 0..max {
            c_result[i] = info.response.roboshop_items[i].clone().into();
        }
    } else if let Err(e) = result {
        println!("{}", e);
        // place error info into first array item
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        c_result[0] = FactoryRobotListInfoC::mock_error(
            &format!("{}", &e),
            ""
        );
    }
}

unsafe fn allocate_cstring(input: &str) -> *mut c_char {
    let input_c = CString::new(input).expect("Rust &str -> CString conversion failed");
    let space = libc::malloc(libc::strlen(input_c.as_ptr()) + 1) as *mut c_char;
    libc::strcpy(space, input_c.as_ptr());
    return space;
}
