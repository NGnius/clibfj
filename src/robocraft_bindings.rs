use libc;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::convert::TryFrom;
use libfj::robocraft_simple as robocraft;
use libfj::robocraft::{FactoryRobotListInfo, FactoryOrderType, FactoryTextSearchType};

use crate::allocate_cstring;

// C-style structs (for interop)

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

#[repr(C)]
pub struct FactorySearchQuery {
    page: *const i32,
    items_per_page: *const i32,
    order: *const i32, // FactoryOrderType
    movement_filter: *const c_char, // CSV (no spaces) of movement integer values
    weapon_filter: *const c_char, // CSV (no spaces) of weapon integer values
    minimum_cpu: *const i32,
    maximum_cpu: *const i32,
    text_filter: *const c_char,
    text_search_field: *const i32, // FactoryTextSearchType
    buyable: *const u32, // bool
    prepend_featured_robot: *const u32, // bool
    featured_only: *const u32, // bool
    default_page: *const u32, // bool
}

impl FactorySearchQuery {
    pub unsafe fn apply(&self, query: robocraft::FactorySearchBuilder) -> robocraft::FactorySearchBuilder {
        let mut result = query.clone();
        if !self.page.is_null() {result = result.page((*self.page) as isize);}
        if !self.items_per_page.is_null() {result = result.items_per_page((*self.items_per_page) as isize);}
        if !self.order.is_null() {result = result.order(FactoryOrderType::try_from(*self.order as u8).unwrap_or(FactoryOrderType::Suggested));}
        if !self.movement_filter.is_null() {
            let filter = CStr::from_ptr(self.movement_filter).to_str().unwrap_or("").to_string();
            result = result.movement_raw(filter);
        }
        if !self.weapon_filter.is_null() {
            let filter = CStr::from_ptr(self.weapon_filter).to_str().unwrap_or("").to_string();
            result = result.weapon_raw(filter);
        }
        if !self.minimum_cpu.is_null() {result = result.min_cpu(*self.minimum_cpu as isize);}
        if !self.maximum_cpu.is_null() {result = result.max_cpu(*self.maximum_cpu as isize);}
        if !self.text_filter.is_null() {
            let filter = CStr::from_ptr(self.text_filter).to_str().unwrap_or("").to_string();
            result = result.text(filter);
        }
        if !self.text_search_field.is_null() {result = result.text_search_type(FactoryTextSearchType::try_from(*self.text_search_field as u8).unwrap_or(FactoryTextSearchType::All));}
        if !self.buyable.is_null() {result = result.buyable(*self.buyable != 0);}
        if !self.prepend_featured_robot.is_null() {result = result.prepend_featured(*self.buyable != 0);}
        if !self.default_page.is_null() {result = result.default_page(*self.default_page != 0);}
        result
    }
}

// function bindings

#[no_mangle]
pub unsafe extern "C" fn libfj_factory_front_page(items: libc::c_uint, array_ptr: *mut FactoryRobotListInfoC) -> u32 {
    if items == 0 {return 0;} // nothing to populate, so it's useless to do work
    let factory_api = robocraft::FactoryAPI::new();
    let result = factory_api.list();
    if let Ok(info) = result {
        let max = if info.response.roboshop_items.len() < (items as usize) { info.response.roboshop_items.len() } else {items as usize};
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        // copy results to output array
        for i in 0..max {
            c_result[i] = info.response.roboshop_items[i].clone().into();
        }
        return max as u32;
    } else if let Err(e) = result {
        println!("{}", e);
        // place error info into first array item
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        c_result[0] = FactoryRobotListInfoC::mock_error(
            &format!("{}", &e),
            ""
        );
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn libfj_factory_search(items: libc::c_uint, array_ptr: *mut FactoryRobotListInfoC, query: *const FactorySearchQuery) -> u32 {
    if items == 0 {return 0;} // nothing to populate, so it's useless to do work
    let factory_api = robocraft::FactoryAPI::new();
    let mut builder = factory_api.list_builder();
    if !query.is_null() {
        let query = &*query; // rustic
        builder = query.apply(builder);
    }
    let result = builder.send();
    if let Ok(info) = result {
        let max = if info.response.roboshop_items.len() < (items as usize) { info.response.roboshop_items.len() } else {items as usize};
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        // copy results to output array
        for i in 0..max {
            c_result[i] = info.response.roboshop_items[i].clone().into();
        }
        return max as u32;
    } else if let Err(e) = result {
        println!("{}", e);
        // place error info into first array item
        let c_result = std::slice::from_raw_parts_mut(array_ptr, items as usize);
        c_result[0] = FactoryRobotListInfoC::mock_error(
            &format!("{}", &e),
            ""
        );
    }
    return 0;
}
