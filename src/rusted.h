#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define BACKDROP_HEIGHT 10

#define BACKDROP_WIDTH 13

#define FONT_HEIGHT 8

#define FONT_WIDTH 8

#define HALFTILE_HEIGHT 8

#define HALFTILE_WIDTH 8

#define HEALTH_COUNT 8

#define INVENTORY_WIDTH (HEALTH_COUNT / 2)

/**
 * The height of the level in full tiles
 */
#define LEVEL_HEIGHT 90

/**
 * The width of the level in full tiles
 */
#define LEVEL_WIDTH 128

#define Health_MAX 8

#define MAX_FIREPOWER 4

#define MAX_LIFE 8

#define MAX_TILES_PER_FILE 50

#define PICTURE_HEIGHT 200

#define PICTURE_WIDTH 40

#define SCORE_DIGITS 8

#define TILE_HEIGHT (HALFTILE_HEIGHT * 2)

#define TILE_WIDTH (HALFTILE_WIDTH * 2)

#define WINDOW_HEIGHT 200

#define WINDOW_WIDTH 320

typedef enum {
    ActorMessageType_OpenDoor,
    ActorMessageType_Teleport,
    ActorMessageType_Expand,
} ActorMessageType;

typedef enum {
    ActorType_FireWheelBot,
    ActorType_FlameGnomeBot,
    ActorType_FlyingBot,
    ActorType_FootBot,
    ActorType_HelicopterBot,
    ActorType_RabbitoidBot,
    ActorType_RedBallJumping,
    ActorType_RedBallLying,
    ActorType_Robot,
    ActorType_RobotDisappearing,
    ActorType_SnakeBot,
    ActorType_TankBot,
    ActorType_WallCrawlerBotLeft,
    ActorType_WallCrawlerBotRight,
    ActorType_DrProton,
    ActorType_Camera,
    ActorType_Explosion,
    ActorType_DustCloud,
    ActorType_Steam,
    ActorType_ParticlePink,
    ActorType_ParticleBlue,
    ActorType_ParticleWhite,
    ActorType_ParticleGreen,
    ActorType_Rocket,
    ActorType_Bomb,
    ActorType_BombFire,
    ActorType_Water,
    ActorType_ExitDoor,
    ActorType_Notebook,
    ActorType_SurveillanceScreen,
    ActorType_HostileShotLeft,
    ActorType_HostileShotRight,
    ActorType_Soda,
    ActorType_SodaFlying,
    ActorType_UnstableFloor,
    ActorType_ExpandingFloor,
    ActorType_ConveyorLeftMovingRightEnd,
    ActorType_ConveyorRightMovingRightEnd,
    ActorType_FanLeft,
    ActorType_FanRight,
    ActorType_BrokenWallBackground,
    ActorType_StoneBackground,
    ActorType_Teleporter1,
    ActorType_Teleporter2,
    ActorType_FenceBackground,
    ActorType_StoneWindowBackground,
    ActorType_WindowLeftBackground,
    ActorType_WindowRightBackground,
    ActorType_Screen,
    ActorType_BoxGreyEmpty,
    ActorType_BoxGreyBoots,
    ActorType_Boots,
    ActorType_BoxGreyClamps,
    ActorType_Clamps,
    ActorType_BoxGreyGun,
    ActorType_Gun,
    ActorType_BoxGreyBomb,
    ActorType_BoxRedSoda,
    ActorType_BoxRedChicken,
    ActorType_ChickenSingle,
    ActorType_ChickenDouble,
    ActorType_BoxBlueFootball,
    ActorType_Football,
    ActorType_Flag,
    ActorType_BoxBlueJoystick,
    ActorType_Joystick,
    ActorType_BoxBlueDisk,
    ActorType_Disk,
    ActorType_BoxBlueBalloon,
    ActorType_Balloon,
    ActorType_BoxGreyGlove,
    ActorType_Glove,
    ActorType_BoxGreyFullLife,
    ActorType_FullLife,
    ActorType_BoxBlueFlag,
    ActorType_BlueFlag,
    ActorType_BoxBlueRadio,
    ActorType_Radio,
    ActorType_BoxGreyAccessCard,
    ActorType_AccessCard,
    ActorType_BoxGreyLetterD,
    ActorType_LetterD,
    ActorType_BoxGreyLetterU,
    ActorType_LetterU,
    ActorType_BoxGreyLetterK,
    ActorType_LetterK,
    ActorType_BoxGreyLetterE,
    ActorType_LetterE,
    ActorType_AccessCardSlot,
    ActorType_GloveSlot,
    ActorType_KeyRed,
    ActorType_KeyholeRed,
    ActorType_DoorRed,
    ActorType_KeyBlue,
    ActorType_KeyholeBlue,
    ActorType_DoorBlue,
    ActorType_KeyPink,
    ActorType_KeyholePink,
    ActorType_DoorPink,
    ActorType_KeyGreen,
    ActorType_KeyholeGreen,
    ActorType_DoorGreen,
    ActorType_ShootableWall,
    ActorType_Lift,
    ActorType_Acme,
    ActorType_FireRight,
    ActorType_FireLeft,
    ActorType_Mill,
    ActorType_Laserbeam,
    ActorType_AccessCardDoor,
    ActorType_SpikesUp,
    ActorType_SpikesDown,
    ActorType_Spike,
    ActorType_Score100,
    ActorType_Score200,
    ActorType_Score500,
    ActorType_Score1000,
    ActorType_Score2000,
    ActorType_Score5000,
    ActorType_Score10000,
    ActorType_ScoreBonus1Left,
    ActorType_ScoreBonus1Right,
    ActorType_ScoreBonus2Left,
    ActorType_ScoreBonus2Right,
    ActorType_ScoreBonus3Left,
    ActorType_ScoreBonus3Right,
    ActorType_ScoreBonus4Left,
    ActorType_ScoreBonus4Right,
    ActorType_ScoreBonus5Left,
    ActorType_ScoreBonus5Right,
    ActorType_ScoreBonus6Left,
    ActorType_ScoreBonus6Right,
    ActorType_ScoreBonus7Left,
    ActorType_ScoreBonus7Right,
    ActorType_BlueLightBackground1,
    ActorType_BlueLightBackground2,
    ActorType_BlueLightBackground3,
    ActorType_BlueLightBackground4,
    ActorType_TextOnScreenBackground,
    ActorType_HighVoltageFlashBackground,
    ActorType_RedFlashlightBackground,
    ActorType_BlueFlashlightBackground,
    ActorType_KeypanelBackground,
    ActorType_RedRotationLightBackground,
    ActorType_UpArrowBackground,
    ActorType_GreenPoisonBackground,
    ActorType_LavaBackground,
} ActorType;

typedef enum {
    BotType_FireWheel,
    BotType_FlameGnome,
    BotType_FlyingBot,
    BotType_FootBot,
    BotType_Helicopter,
    BotType_Rabbitoid,
    BotType_RedBallJumping,
    BotType_RedBallLying,
    BotType_SnakeBot,
    BotType_TankBot,
    BotType_WallCrawlerLeft,
    BotType_WallCrawlerRight,
    BotType_DrProton,
} BotType;

typedef enum {
    FnInputBoxAnswer_Ok,
    FnInputBoxAnswer_Quit,
} FnInputBoxAnswer;

typedef enum {
    HorizontalDirection_Center,
    HorizontalDirection_Left,
    HorizontalDirection_Right,
} HorizontalDirection;

typedef enum {
    InventoryItem_KeyRed,
    InventoryItem_KeyGreen,
    InventoryItem_KeyBlue,
    InventoryItem_KeyPink,
    InventoryItem_Boot,
    InventoryItem_Glove,
    InventoryItem_Clamp,
    InventoryItem_AccessCard,
} InventoryItem;

typedef enum {
    MainMenuEntry_Start,
    MainMenuEntry_Restore,
    MainMenuEntry_Instructions,
    MainMenuEntry_OrderingInfo,
    MainMenuEntry_FullScreenToggle,
    MainMenuEntry_EpisodeChange,
    MainMenuEntry_HighScores,
    MainMenuEntry_Previews,
    MainMenuEntry_ViewUserDemo,
    MainMenuEntry_TitleScreen,
    MainMenuEntry_Credits,
    MainMenuEntry_Quit,
    MainMenuEntry_Invalid,
} MainMenuEntry;

typedef enum {
    UserEvent_Timer,
    UserEvent_HeroMoved,
    UserEvent_HeroScored,
    UserEvent_HeroFirepowerChanged,
    UserEvent_HeroInventoryChanged,
    UserEvent_HeroHealthChanged,
    UserEvent_HeroLanded,
} UserEvent;

typedef enum {
    VerticalDirection_Center,
    VerticalDirection_Up,
    VerticalDirection_Down,
} VerticalDirection;

typedef struct ActorMessageQueue ActorMessageQueue;

typedef struct ActorQueue ActorQueue;

typedef struct Bot Bot;

typedef struct FetchedLetterState FetchedLetterState;

typedef struct File File;

typedef struct Firepower Firepower;

typedef struct Health Health;

typedef struct HeroData HeroData;

typedef struct InfoMessageQueue InfoMessageQueue;

typedef struct InputField InputField;

typedef struct Inventory Inventory;

typedef struct Menu Menu;

typedef struct Position Position;

typedef struct Score Score;

typedef struct Texture Texture;

typedef struct TileCache TileCache;

typedef Texture FnTexture;

typedef File FnFile;

typedef struct {
    uint32_t flags;
    int32_t bits_per_pixel;
    uint32_t transparent;
} TextureCreationParams;

typedef TextureCreationParams FnTextureCreationParams;

typedef TileCache FnTileCache;

typedef Firepower FnHeroFirepower;

typedef Inventory FnHeroInventory;

typedef Bot FnBot;

typedef BotType FnBotType;

typedef struct {
    int16_t x;
    int16_t y;
    uint16_t w;
    uint16_t h;
} Geometry;

typedef struct {
    ActorType actor_type;
    Geometry position;
    bool is_in_foreground;
    bool hurts_hero;
    bool is_alive;
} ActorData;

typedef ActorData FnLevelActorData;

typedef struct {
    uint16_t tiles[LEVEL_HEIGHT][LEVEL_WIDTH];
} LevelTiles;

typedef LevelTiles FnLevelTiles;

typedef struct {
    bool solids[LEVEL_HEIGHT][LEVEL_WIDTH];
} LevelSolids;

typedef LevelSolids FnLevelSolids;

typedef struct {
    FnLevelTiles tiles;
    FnLevelSolids solids;
    bool do_play;
    bool level_passed;
} LevelData;

typedef LevelData FnLevelData;

typedef struct {
    FnLevelActorData *general;
    void **specific;
    FnLevelData *level_data;
} FnLevelActorCreateParams;

typedef struct {
    void **specific;
} FnLevelActorFreeParams;

typedef ActorQueue FnLevelActorQueue;

typedef HeroData FnHeroData;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnLevelActorQueue *actor_queue;
    FnHeroData *hero_data;
} FnLevelActorHeroTouchStartParams;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnHeroData *hero_data;
} FnLevelActorHeroTouchEndParams;

typedef InfoMessageQueue FnInfoMessageQueue;

typedef ActorMessageQueue FnLevelActorMessageQueue;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnLevelData *level_data;
    FnHeroData *hero_data;
    FnInfoMessageQueue *info_message_queue;
    FnLevelActorMessageQueue *actor_message_queue;
} FnLevelActorHeroInteractStartParams;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnLevelData *level_data;
    FnHeroData *hero_data;
} FnLevelActorHeroInteractEndParams;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnLevelData *level_data;
    FnLevelActorQueue *actor_queue;
    FnHeroData *hero_data;
} FnLevelActorActParams;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnHeroData *hero_data;
    const FnTileCache *tilecache;
    SDL_Surface *target;
} FnLevelActorBlitParams;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnLevelData *level_data;
    FnLevelActorQueue *actor_queue;
    FnHeroData *hero_data;
} FnLevelActorShotParams;

typedef ActorMessageType FnLevelActorMessageType;

typedef struct {
    FnLevelActorData *general;
    void *specific;
    FnLevelActorMessageType message;
    FnHeroData *hero_data;
    FnLevelData *level_data;
} FnLevelActorReceiveMessageParams;

typedef Geometry FnGeometry;

typedef FetchedLetterState FnHeroFetchedLetterState;

typedef Health FnHeroHealth;

typedef Position FnHeroPosition;

typedef Score FnHeroScore;

typedef InventoryItem FnHeroInventoryItem;

typedef HorizontalDirection FnHorizontalDirection;

typedef InputField FnInputField;

typedef ActorType FnLevelActorType;

typedef struct {
    ActorType receivers;
    ActorMessageType message;
} ActorMessage;

typedef ActorMessage FnLevelActorMessage;

typedef struct {
    ActorType actor_type;
    uint16_t x;
    uint16_t y;
} ActorQueueItem;

typedef ActorQueueItem FnLevelActorQueueItem;

typedef MainMenuEntry FnMainMenuEntry;

typedef Menu FnMenu;

typedef struct {
    uint8_t pixelsize;
    bool fullscreen;
    bool draw_collision_bounds;
} Settings;

typedef Settings FnSettings;

typedef struct {
    uint8_t tiles;
    uint8_t width;
    uint8_t height;
} TileHeader;

typedef TileHeader FnTileHeader;

typedef UserEvent FnUserEvent;

typedef VerticalDirection FnVerticalDirection;

FnTexture *fn_backdrop_load(FnFile *ptr, FnTextureCreationParams params);

void fn_borders_blit(SDL_Surface *screen,
                     FnTextureCreationParams params,
                     const FnTileCache *tilecache);

void fn_borders_blit_firepower(SDL_Surface *screen,
                               FnTextureCreationParams params,
                               const FnTileCache *tilecache,
                               const FnHeroFirepower *firepower);

void fn_borders_blit_inventory(SDL_Surface *screen,
                               FnTextureCreationParams params,
                               const FnTileCache *tilecache,
                               const FnHeroInventory *inventory);

void fn_borders_blit_life(SDL_Surface *screen,
                          FnTextureCreationParams params,
                          const FnTileCache *tilecache,
                          uint8_t health);

void fn_borders_blit_score(SDL_Surface *screen,
                           FnTextureCreationParams params,
                           const FnTileCache *tilecache,
                           uintptr_t score);

void fn_bot_blit(const FnBot *bot,
                 SDL_Surface *surface,
                 const FnTileCache *tilecache);

FnBot *fn_bot_create(FnBotType bot_type, uintptr_t x, uintptr_t y);

void fn_bot_free(FnBot *ptr);

uintptr_t fn_bot_get_x(const FnBot *bot);

uintptr_t fn_bot_get_y(const FnBot *bot);

void fn_expose_all_function_parameters(FnLevelActorCreateParams _a,
                                       FnLevelActorFreeParams _b,
                                       FnLevelActorHeroTouchStartParams _c,
                                       FnLevelActorHeroTouchEndParams _d,
                                       FnLevelActorHeroInteractStartParams _e,
                                       FnLevelActorHeroInteractEndParams _f,
                                       FnLevelActorActParams _g,
                                       FnLevelActorBlitParams _h,
                                       FnLevelActorShotParams _i,
                                       FnLevelActorReceiveMessageParams _j);

void fn_file_free(FnFile *ptr);

FnFile *fn_file_open(const char *path);

void fn_file_read(FnFile *ptr, void *buffer, size_t length);

SDL_Rect fn_geometry_as_sdl_rect(const FnGeometry *ptr);

FnGeometry fn_geometry_create(int16_t x, int16_t y, uint16_t w, uint16_t h);

void fn_geometry_draw_outline(SDL_Surface *surface,
                              FnGeometry geometry,
                              uint32_t color);

int32_t fn_geometry_horizontal_distance(FnGeometry r1, FnGeometry r2);

bool fn_geometry_overlaps(FnGeometry r1, FnGeometry r2);

bool fn_geometry_overlaps_vertically(FnGeometry r1, FnGeometry r2);

bool fn_geometry_touches(FnGeometry r1, FnGeometry r2);

FnHeroData *fn_hero_data_create(void);

void fn_hero_data_free(FnHeroData *ptr);

FnHeroFetchedLetterState *fn_hero_data_get_fetched_letter_state(FnHeroData *ptr);

FnHeroFirepower *fn_hero_data_get_firepower(FnHeroData *ptr);

FnHeroHealth *fn_hero_data_get_health(FnHeroData *ptr);

bool fn_hero_data_get_hidden(const FnHeroData *ptr);

FnHeroInventory *fn_hero_data_get_inventory(FnHeroData *ptr);

FnHeroPosition *fn_hero_data_get_position(FnHeroData *ptr);

FnHeroScore *fn_hero_data_get_score(FnHeroData *ptr);

void fn_hero_data_reset(FnHeroData *data);

void fn_hero_data_set_hidden(FnHeroData *ptr, bool hidden);

void fn_hero_fetched_letter_state_picked(FnHeroFetchedLetterState *ptr,
                                         char letter);

void fn_hero_fetched_letter_state_reset(FnHeroFetchedLetterState *ptr);

bool fn_hero_fetched_letter_state_succeeded(const FnHeroFetchedLetterState *ptr);

void fn_hero_firepower_increase(FnHeroFirepower *firepower, uint8_t amount);

uint8_t fn_hero_firepower_num_shots(const FnHeroFirepower *firepower);

void fn_hero_health_decrease(FnHeroHealth *health, uint8_t amount);

void fn_hero_health_fill_max(FnHeroHealth *health);

uint8_t fn_hero_health_get(const FnHeroHealth *health);

void fn_hero_health_increase(FnHeroHealth *health, uint8_t amount);

void fn_hero_health_kill(FnHeroHealth *health);

void fn_hero_inventory_clear(FnHeroInventory *inventory);

bool fn_hero_inventory_is_set(const FnHeroInventory *inventory,
                              FnHeroInventoryItem item);

void fn_hero_inventory_set(FnHeroInventory *inventory,
                           FnHeroInventoryItem item);

void fn_hero_inventory_unset(FnHeroInventory *inventory,
                             FnHeroInventoryItem item);

FnGeometry fn_hero_position_get_geometry(const FnHeroPosition *position);

void fn_hero_position_move_to(FnHeroPosition *position,
                              uint16_t x,
                              uint16_t y);

void fn_hero_position_move_x_by(FnHeroPosition *position, int16_t x);

void fn_hero_position_move_x_to(FnHeroPosition *position, uint16_t x);

void fn_hero_position_move_y_by(FnHeroPosition *position, int16_t y);

void fn_hero_position_move_y_to(FnHeroPosition *position, uint16_t y);

int16_t fn_hero_position_push_horizontally(FnHeroPosition *position,
                                           const FnLevelSolids *solids,
                                           int16_t offset);

int16_t fn_hero_position_push_vertically(FnHeroPosition *position,
                                         const FnLevelSolids *solids,
                                         int16_t offset);

void fn_hero_score_add(FnHeroScore *score, uint64_t amount);

uint64_t fn_hero_score_get(const FnHeroScore *score);

void fn_horizontal_direction_print(FnHorizontalDirection direction);

FnInfoMessageQueue *fn_info_message_queue_create(void);

void fn_info_message_queue_free(FnInfoMessageQueue *ptr);

void fn_info_message_queue_process(FnInfoMessageQueue *ptr,
                                   SDL_Surface *screen,
                                   const FnTileCache *tilecache,
                                   FnTextureCreationParams params);

void fn_info_message_queue_push(FnInfoMessageQueue *ptr, const char *message);

void fn_infobox_show(SDL_Surface *screen,
                     const FnTileCache *tilecache,
                     FnTextureCreationParams params,
                     const char *message);

FnInputBoxAnswer fn_inputbox_show(SDL_Surface *screen,
                                  const FnTileCache *tilecache,
                                  FnTextureCreationParams params,
                                  const char *message,
                                  char *answer,
                                  size_t answer_length);

void fn_inputfield_backspace_pressed(FnInputField *ptr);

void fn_inputfield_blit(const FnInputField *ptr,
                        FnTexture *target,
                        const FnTileCache *tilecache);

uintptr_t fn_inputfield_copy_text_to(const FnInputField *ptr,
                                     char *buffer,
                                     uintptr_t max_length);

FnInputField *fn_inputfield_create(uint8_t max_length);

void fn_inputfield_delete_pressed(FnInputField *ptr);

void fn_inputfield_free(FnInputField *ptr);

void fn_inputfield_left_pressed(FnInputField *ptr);

void fn_inputfield_right_pressed(FnInputField *ptr);

void fn_inputfield_symbol_pressed(FnInputField *ptr, char symbol);

uintptr_t fn_inputfield_text_length(const FnInputField *ptr);

FnLevelActorData *fn_level_actor_data_create(FnLevelActorType actor_type);

void fn_level_actor_data_free(FnLevelActorData *ptr);

void fn_level_actor_function_accesscard_slot_act(FnLevelActorActParams p);

void fn_level_actor_function_accesscard_slot_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_accesscard_slot_create(FnLevelActorCreateParams p);

void fn_level_actor_function_accesscard_slot_free(FnLevelActorFreeParams p);

void fn_level_actor_function_accesscard_slot_hero_interact_start(FnLevelActorHeroInteractStartParams p);

void fn_level_actor_function_acme_act(FnLevelActorActParams p);

void fn_level_actor_function_acme_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_acme_create(FnLevelActorCreateParams p);

void fn_level_actor_function_acme_free(FnLevelActorFreeParams p);

void fn_level_actor_function_acme_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_acme_shot(FnLevelActorShotParams p);

void fn_level_actor_function_balloon_act(FnLevelActorActParams p);

void fn_level_actor_function_balloon_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_balloon_create(FnLevelActorCreateParams p);

void fn_level_actor_function_balloon_free(FnLevelActorFreeParams p);

void fn_level_actor_function_balloon_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_balloon_shot(FnLevelActorShotParams p);

void fn_level_actor_function_bomb_act(FnLevelActorActParams p);

void fn_level_actor_function_bomb_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_bomb_create(FnLevelActorCreateParams p);

void fn_level_actor_function_bomb_free(FnLevelActorFreeParams p);

void fn_level_actor_function_camera_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_camera_create(FnLevelActorCreateParams p);

void fn_level_actor_function_camera_free(FnLevelActorFreeParams _p);

void fn_level_actor_function_camera_shot(FnLevelActorShotParams p);

void fn_level_actor_function_elevator_act(FnLevelActorActParams p);

void fn_level_actor_function_elevator_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_elevator_create(FnLevelActorCreateParams p);

void fn_level_actor_function_elevator_free(FnLevelActorFreeParams p);

void fn_level_actor_function_elevator_hero_interact_end(FnLevelActorHeroInteractEndParams p);

void fn_level_actor_function_elevator_hero_interact_start(FnLevelActorHeroInteractStartParams p);

void fn_level_actor_function_expandingfloor_act(FnLevelActorActParams p);

void fn_level_actor_function_expandingfloor_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_expandingfloor_create(FnLevelActorCreateParams p);

void fn_level_actor_function_expandingfloor_free(FnLevelActorFreeParams p);

void fn_level_actor_function_expandingfloor_receive_message(FnLevelActorReceiveMessageParams p);

void fn_level_actor_function_fire_act(FnLevelActorActParams p);

void fn_level_actor_function_fire_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_fire_create(FnLevelActorCreateParams p);

void fn_level_actor_function_fire_free(FnLevelActorFreeParams p);

void fn_level_actor_function_fire_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_fire_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_firewheelbot_act(FnLevelActorActParams p);

void fn_level_actor_function_firewheelbot_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_firewheelbot_create(FnLevelActorCreateParams p);

void fn_level_actor_function_firewheelbot_free(FnLevelActorFreeParams p);

void fn_level_actor_function_firewheelbot_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_firewheelbot_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_firewheelbot_shot(FnLevelActorShotParams p);

void fn_level_actor_function_glove_slot_act(FnLevelActorActParams p);

void fn_level_actor_function_glove_slot_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_glove_slot_create(FnLevelActorCreateParams p);

void fn_level_actor_function_glove_slot_free(FnLevelActorFreeParams p);

void fn_level_actor_function_glove_slot_hero_interact_start(FnLevelActorHeroInteractStartParams p);

void fn_level_actor_function_item_act(FnLevelActorActParams p);

void fn_level_actor_function_item_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_item_create(FnLevelActorCreateParams p);

void fn_level_actor_function_item_free(FnLevelActorFreeParams p);

void fn_level_actor_function_item_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_item_shot(FnLevelActorShotParams p);

void fn_level_actor_function_mill_act(FnLevelActorActParams p);

void fn_level_actor_function_mill_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_mill_create(FnLevelActorCreateParams p);

void fn_level_actor_function_mill_free(FnLevelActorFreeParams p);

void fn_level_actor_function_mill_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_mill_shot(FnLevelActorShotParams p);

void fn_level_actor_function_particle_act(FnLevelActorActParams p);

void fn_level_actor_function_particle_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_particle_create(FnLevelActorCreateParams p);

void fn_level_actor_function_particle_free(FnLevelActorFreeParams p);

void fn_level_actor_function_redball_jumping_act(FnLevelActorActParams p);

void fn_level_actor_function_redball_jumping_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_redball_jumping_create(FnLevelActorCreateParams p);

void fn_level_actor_function_redball_jumping_free(FnLevelActorFreeParams p);

void fn_level_actor_function_redball_jumping_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_redball_jumping_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_redball_jumping_shot(FnLevelActorShotParams _p);

void fn_level_actor_function_redball_lying_act(FnLevelActorActParams p);

void fn_level_actor_function_redball_lying_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_redball_lying_create(FnLevelActorCreateParams p);

void fn_level_actor_function_redball_lying_free(FnLevelActorFreeParams p);

void fn_level_actor_function_redball_lying_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_redball_lying_shot(FnLevelActorShotParams _p);

void fn_level_actor_function_robot_act(FnLevelActorActParams p);

void fn_level_actor_function_robot_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_robot_create(FnLevelActorCreateParams p);

void fn_level_actor_function_robot_free(FnLevelActorFreeParams p);

void fn_level_actor_function_robot_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_robot_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_robot_shot(FnLevelActorShotParams p);

void fn_level_actor_function_rocket_act(FnLevelActorActParams p);

void fn_level_actor_function_rocket_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_rocket_create(FnLevelActorCreateParams p);

void fn_level_actor_function_rocket_free(FnLevelActorFreeParams p);

void fn_level_actor_function_rocket_shot(FnLevelActorShotParams p);

void fn_level_actor_function_score_act(FnLevelActorActParams p);

void fn_level_actor_function_score_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_score_create(FnLevelActorCreateParams p);

void fn_level_actor_function_score_free(FnLevelActorFreeParams p);

void fn_level_actor_function_simpleanimation_act(FnLevelActorActParams p);

void fn_level_actor_function_simpleanimation_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_simpleanimation_create(FnLevelActorCreateParams p);

void fn_level_actor_function_simpleanimation_free(FnLevelActorFreeParams p);

void fn_level_actor_function_singleanimation_act(FnLevelActorActParams p);

void fn_level_actor_function_singleanimation_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_singleanimation_create(FnLevelActorCreateParams p);

void fn_level_actor_function_singleanimation_free(FnLevelActorFreeParams p);

void fn_level_actor_function_singleanimation_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_singleanimation_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_soda_flying_act(FnLevelActorActParams p);

void fn_level_actor_function_soda_flying_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_soda_flying_create(FnLevelActorCreateParams p);

void fn_level_actor_function_soda_flying_free(FnLevelActorFreeParams _p);

void fn_level_actor_function_soda_flying_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_spikes_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_spikes_create(FnLevelActorCreateParams p);

void fn_level_actor_function_spikes_free(FnLevelActorFreeParams p);

void fn_level_actor_function_spikes_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_spikes_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_tankbot_act(FnLevelActorActParams p);

void fn_level_actor_function_tankbot_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_tankbot_create(FnLevelActorCreateParams p);

void fn_level_actor_function_tankbot_free(FnLevelActorFreeParams p);

void fn_level_actor_function_tankbot_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_tankbot_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_tankbot_shot(FnLevelActorShotParams p);

void fn_level_actor_function_teleporter_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_teleporter_create(FnLevelActorCreateParams p);

void fn_level_actor_function_teleporter_free(FnLevelActorFreeParams _p);

void fn_level_actor_function_teleporter_hero_interact_start(FnLevelActorHeroInteractStartParams p);

void fn_level_actor_function_teleporter_receive_message(FnLevelActorReceiveMessageParams p);

void fn_level_actor_function_unstablefloor_act(FnLevelActorActParams p);

void fn_level_actor_function_unstablefloor_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_unstablefloor_create(FnLevelActorCreateParams p);

void fn_level_actor_function_unstablefloor_free(FnLevelActorFreeParams p);

void fn_level_actor_function_wallcrawler_act(FnLevelActorActParams p);

void fn_level_actor_function_wallcrawler_blit(FnLevelActorBlitParams p);

void fn_level_actor_function_wallcrawler_create(FnLevelActorCreateParams p);

void fn_level_actor_function_wallcrawler_free(FnLevelActorFreeParams p);

void fn_level_actor_function_wallcrawler_hero_touch_end(FnLevelActorHeroTouchEndParams p);

void fn_level_actor_function_wallcrawler_hero_touch_start(FnLevelActorHeroTouchStartParams p);

void fn_level_actor_function_wallcrawler_shot(FnLevelActorShotParams p);

FnLevelActorMessageQueue *fn_level_actor_message_queue_create(void);

void fn_level_actor_message_queue_free(FnLevelActorMessageQueue *ptr);

bool fn_level_actor_message_queue_has_items(const FnLevelActorMessageQueue *ptr);

FnLevelActorMessage fn_level_actor_message_queue_pop_front(FnLevelActorMessageQueue *ptr);

void fn_level_actor_message_queue_push_back(FnLevelActorMessageQueue *ptr,
                                            FnLevelActorType receivers,
                                            FnLevelActorMessageType message);

FnLevelActorQueue *fn_level_actor_queue_create(void);

void fn_level_actor_queue_free(FnLevelActorQueue *ptr);

bool fn_level_actor_queue_has_items(const FnLevelActorQueue *ptr);

FnLevelActorQueueItem fn_level_actor_queue_pop_front(FnLevelActorQueue *ptr);

void fn_level_actor_queue_push_back(FnLevelActorQueue *ptr,
                                    FnLevelActorType actor_type,
                                    uint16_t x,
                                    uint16_t y);

void fn_level_actor_queue_push_particle_firework(FnLevelActorQueue *ptr,
                                                 uint16_t x,
                                                 uint16_t y,
                                                 uintptr_t count);

FnLevelData *fn_level_data_create(void);

void fn_level_data_free(FnLevelData *ptr);

bool fn_level_solids_collides(const FnLevelSolids *ptr, FnGeometry rect);

FnLevelSolids *fn_level_solids_create(void);

void fn_level_solids_free(FnLevelSolids *ptr);

bool fn_level_solids_get(const FnLevelSolids *ptr, uintptr_t x, uintptr_t y);

void fn_level_solids_set(FnLevelSolids *ptr,
                         uintptr_t x,
                         uintptr_t y,
                         bool value);

void fn_level_tiles_copy_from_to(FnLevelTiles *ptr,
                                 uintptr_t x_from,
                                 uintptr_t y_from,
                                 uintptr_t x_to,
                                 uintptr_t y_to);

FnLevelTiles *fn_level_tiles_create(void);

void fn_level_tiles_free(FnLevelTiles *ptr);

uint16_t fn_level_tiles_get(const FnLevelTiles *ptr,
                            uintptr_t x,
                            uintptr_t y);

void fn_level_tiles_set(FnLevelTiles *ptr,
                        uintptr_t x,
                        uintptr_t y,
                        uint16_t value);

FnMainMenuEntry fn_mainmenu(SDL_Surface *screen,
                            const FnTileCache *tilecache,
                            FnTextureCreationParams texture_creation_paramns);

void fn_menu_append_entry(FnMenu *ptr, char shortcut, char *name);

FnMenu *fn_menu_create(const char *header);

void fn_menu_free(FnMenu *ptr);

char fn_menu_get_choice(FnMenu *ptr,
                        SDL_Surface *screen,
                        const FnTileCache *tilecache,
                        FnTextureCreationParams texture_creation_paramns);

FnTexture *fn_messagebox(const char *text,
                         const FnTileCache *tilecache,
                         FnTextureCreationParams params);

void fn_messagebox_get_text_information(const char *text,
                                        uint16_t *cols,
                                        uint16_t *rows);

FnTexture *fn_picture_load(FnFile *file, FnTextureCreationParams params);

FnSettings fn_settings_load_or_create(void);

void fn_settings_save(FnSettings settings);

void fn_text_print(FnTexture *target,
                   FnGeometry geometry,
                   const FnTileCache *tilecache,
                   const char *text);

void fn_texture_blit_to_sdl_surface(const FnTexture *ptr,
                                    const Geometry *srcrect,
                                    SDL_Surface *destination,
                                    const Geometry *dstrect);

void fn_texture_clone_to_texture(const FnTexture *ptr,
                                 const Geometry *srcrect,
                                 FnTexture *destination,
                                 const Geometry *dstrect);

void fn_texture_fill_area(FnTexture *ptr,
                          const Geometry *area,
                          uint8_t red,
                          uint8_t green,
                          uint8_t blue);

void fn_texture_free(FnTexture *ptr);

uint16_t fn_texture_get_height(const FnTexture *ptr);

uint16_t fn_texture_get_width(const FnTexture *ptr);

FnTexture *fn_texture_new_with_params(uint16_t w,
                                      uint16_t h,
                                      FnTextureCreationParams params);

void fn_texture_set_data(FnTexture *ptr,
                         const uint8_t *data,
                         uint32_t transparent);

bool fn_tile_is_solid(uint16_t index);

FnTexture *fn_tile_load(FnFile *file,
                        FnTextureCreationParams params,
                        FnTileHeader header,
                        bool has_transparency);

void fn_tilecache_free(FnTileCache *ptr);

const FnTexture *fn_tilecache_get_tile(const FnTileCache *ptr,
                                       uintptr_t index);

FnTileCache *fn_tilecache_load(const char *path,
                               FnTextureCreationParams params);

FnTileHeader fn_tileheader_load(FnFile *file);

void fn_user_event_print(FnUserEvent e);

void fn_vertical_direction_print(FnVerticalDirection direction);
