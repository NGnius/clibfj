#ifndef LIBFJ
#define LIBFJ

typedef struct FactoryRobotListInfo_struct{
    uint32_t item_id;
    char* item_name;
    char* item_description;
    char* thumbnail; // url
    char* added_by;
    char* added_by_display_name;
    char* added_date; // ISO date
    char* expiry_date; // ISO date
    uint32_t cpu;
    uint32_t total_robot_ranking;
    uint32_t rent_count;
    uint32_t buy_count;
    uint32_t buyable; // bool
    char* removed_date;
    char* ban_date;
    uint32_t featured; // bool
    char* banner_message;
    float combat_rating;
    float cosmetic_rating;
    char* cube_amounts; // JSON as str
} FactoryRobotListInfo;

void get_factory_front_page(uint32_t size, FactoryRobotListInfo* array_ptr);

#endif