# AI Engineer Career RPG - Implementation Plan

## Tech Stack
- **Engine**: Macroquad (simple 2D, fast iteration)
- **Style**: Top-down pixel art, programmatic sprites
- **Scale**: Full city with multiple districts

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    GAME STATE                           │
│  - Current screen (World, Dialog, Menu, Interview)      │
│  - Player data (reuse existing)                         │
│  - World state (NPC positions, time)                    │
└─────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   WORLD      │  │    UI        │  │   SYSTEMS    │
│              │  │              │  │              │
│ - Map/Tiles  │  │ - HUD        │  │ - Study      │
│ - Player     │  │ - Dialog     │  │ - Interview  │
│ - NPCs       │  │ - Menus      │  │ - Jobs       │
│ - Buildings  │  │ - Skills UI  │  │ - Time       │
└──────────────┘  └──────────────┘  └──────────────┘
```

## File Structure

```
ai_career_rpg/src/
├── main.rs                 # Entry point, game loop
├── lib.rs                  # Module exports
│
├── game/
│   ├── mod.rs
│   ├── state.rs            # GameState enum, transitions
│   └── time.rs             # Day cycle, time management
│
├── world/
│   ├── mod.rs
│   ├── map.rs              # TileMap, zones, collision
│   ├── player.rs           # Position, movement, animation
│   ├── npc.rs              # NPC entities, behavior
│   └── camera.rs           # Camera following player
│
├── ui/
│   ├── mod.rs
│   ├── hud.rs              # Energy, money, day display
│   ├── dialog.rs           # NPC conversations
│   ├── menu.rs             # Pause, job board
│   ├── skills.rs           # Skills panel
│   └── interview.rs        # Quiz UI
│
├── graphics/
│   ├── mod.rs
│   ├── sprites.rs          # Draw pixel sprites
│   └── tiles.rs            # Draw map tiles
│
├── player/                 # EXISTING - reuse
│   └── mod.rs              # Player stats, skills
│
├── skills/                 # EXISTING - reuse
│   └── mod.rs              # Skill definitions
│
├── jobs/                   # EXISTING - reuse
│   └── mod.rs              # Job, requirements
│
├── companies/              # EXISTING - reuse
│   └── mod.rs              # Company data
│
└── interview/              # EXISTING - extend
    └── mod.rs              # Add quiz questions
```

## Map Layout (Tile-based: 32x32 tiles)

### Downtown (center)
- Coffee Shop (meet recruiters)
- Job Center (browse all jobs)
- Co-working Space

### Tech District (north)
- SearchGiant HQ (FAANG)
- MegaTech Campus (Big Tech)
- DataStartup Office (Startup)
- TechCorp Inc (Mid-Size)

### University (east)
- Library (study)
- Lecture Hall (courses)
- Professor's Office

### Residential (south)
- Player Apartment (rest, save)
- Bookstore (buy books)
- Park (random encounters)

## Key Features

1. **Movement**: WASD/Arrows, smooth pixel movement
2. **Interaction**: Walk near NPC/building, press E
3. **Dialog**: Text box with portrait, choices appear
4. **Study**: At library, select skill, spend time
5. **Interview**: Walk into company, quiz UI with timer
6. **Time**: Actions advance time, must rest at home

## NPC Types

1. **Recruiters** - Tell you about open positions, give hints
2. **Senior Engineers** - Give advice, mentor (boost study)
3. **Other Job Seekers** - Share interview tips
4. **Professors** - Teach courses, unlock advanced skills
5. **Barista** - Small talk, energy boost from coffee

## Interview System

- Multiple choice quiz questions
- Timed responses
- Score based on skill proficiency + some randomness
- Multiple rounds per interview

## Dialog System

- Linear with branching choices
- Press E/Enter to interact when near NPC
- Text box with NPC portrait
- Choices affect reputation/hints

## Implementation Phases

| Phase | Tasks |
|-------|-------|
| 1 | Macroquad setup, player sprite, movement, camera |
| 2 | Tile map, buildings, collision |
| 3 | NPCs, dialog system, HUD |
| 4 | Study at library, skill UI |
| 5 | Job board, apply flow |
| 6 | Interview quiz UI |
| 7 | Polish, animations, day cycle |
