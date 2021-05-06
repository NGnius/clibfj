#ifndef LIBFJ
#define LIBFJ

#include <stdint.h>

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

typedef struct FactoryRobotGetInfo_struct{
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
    char* cube_data;
    char* colour_data;
    char* cube_amounts; // JSON as str
} FactoryRobotGetInfo;

typedef enum FactoryOrderType_enum {
    Suggested = 0,
    CombatRating = 1,
    CosmeticRating = 2,
    Added = 3,
    CPU = 4,
    MostBought = 5,
} FactoryOrderType;

typedef enum FactoryMovementType_enum {
    Wheels=100000,
    Hovers=200000,
    Aerofoils=300000,
    Thrusters=400000,
    Rudders=500000,
    InsectLegs=600000,
    MechLegs=700000,
    Skis=800000,
    TankTreads=900000,
    Rotors=1000000,
    Sprinters=1100000,
    Propellers=1200000
} FactoryMovementType;

typedef enum FactoryWeaponType_enum {
    Laser=10000000,
    PlasmaLauncher=20000000,
    GyroMortar=25000000,
    RailCannon=30000000,
    NanoDisruptor=40000000,
    TeslaBlade=50000000,
    AeroflakCannon=60000000,
    IonCannon=65000000,
    ProtoSeeker=70100000,
    ChainShredder=75000000,
} FactoryWeaponType;

typedef enum FactoryTextSearchType_enum {
    All=0,
    Player=1,
    Name=2,
} FactoryTextSearchType;

typedef struct FactorySearchQuery_struct{
    int32_t* page;
    int32_t* items_per_page;
    int32_t* order; // FactoryOrderType
    char* movement_filter; // CSV (no spaces) of movement integer values
    char* weapon_filter; // CSV (no spaces) of weapon integer values
    int32_t* minimum_cpu;
    int32_t* maximum_cpu;
    char* text_filter;
    int32_t* text_search_field; // FactoryTextSearchType
    uint32_t* buyable; // bool
    uint32_t* prepend_featured_robot; // bool
    uint32_t* featured_only; // bool
    uint32_t* default_page; // bool
} FactorySearchQuery;

typedef struct CubeData_struct {
    uint32_t id;
    uint8_t x;
    uint8_t y;
    uint8_t z;
    uint8_t orientation;
    uint8_t colour;
} CubeData;

void libfj_factory_front_page(uint32_t size, FactoryRobotListInfo* array_ptr);

void libfj_factory_search(uint32_t size, FactoryRobotListInfo* array_ptr, FactorySearchQuery* search);

void libfj_factory_robot(uint32_t item_id, FactoryRobotGetInfo* result_out);

void libfj_factory_robot_cubes(uint32_t size, CubeData* array_ptr, FactoryRobotGetInfo* info);

void libfj_factory_robot_cubes_raw(uint32_t size, CubeData* array_ptr, char* cube_data, char* colour_data);

#endif
