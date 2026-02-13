# AI Engineer Career RPG

A 2D top-down RPG where you play as an aspiring AI engineer navigating a career in tech. Study skills, apply for jobs, ace interviews, and climb the career ladder.

## Features

- **Explore a City** - Navigate through Downtown, Tech District, University, and Residential areas
- **Study & Learn** - Visit the library to improve your AI/ML skills
- **Meet NPCs** - Recruiters, senior engineers, professors, and other job seekers
- **Apply for Jobs** - Browse job boards and apply at various tech companies
- **Interview System** - Take timed quiz-style interviews at companies
- **Career Progression** - Start as a junior, work your way up to senior roles

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- A C compiler (for macroquad dependencies)

### Platform-Specific Dependencies

**Windows:**
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ workload

**Linux (Ubuntu/Debian):**
```bash
sudo apt install build-essential libasound2-dev
```

**macOS:**
```bash
xcode-select --install
```

## Build & Run

```bash
# Clone the repository
git clone https://github.com/yourusername/game-ai-engineer.git
cd game-ai-engineer

# Run in debug mode
cargo run

# Run in release mode (better performance)
cargo run --release
```

## Controls

- **WASD / Arrow Keys** - Move player
- **E / Enter** - Interact with NPCs and buildings
- **ESC** - Open menu / Pause

## Architecture

Built with [Macroquad](https://github.com/not-fl3/macroquad) for simple 2D game development in Rust. See [PLAN.md](PLAN.md) for detailed architecture and implementation phases.

## License

MIT
