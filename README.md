## NESYS

NES Emulator written in Rust. Hopefully will be able to run both ordinarily as well as in the browser using WebAssembly.

## Roadmap

### Key Milestones

- [ ] Parse iNES files
- [ ] Create Cartridges (iNES + Mapper interface)
- [ ] CPU
    - [ ] Set Up Memory Map
    - [x] Hardware Structures (registers)
    - [ ] Core Loop / Basic Functionality
        - [x] Read / Write RAM
        - [ ] Addressing Modes
        - [ ] Fetch - Decode - Execute
    - [ ] Official Opcodes Implemented
    - [ ] Handle Interrupts
- [ ] PPU
    - [ ] Set Up Basic Rendering Context (HTMLCanvas + SDL)
    - [ ] Implement Registers + Memory Map them
    - [ ] Implement DMA
    - [ ] Generate NMI -> CPU
    - [ ] Core rendering loop
        - [ ] Background Rendering
        - [ ] Sprite Rendering - _currently not hardware accurate_
        - [ ] Proper Background / Foreground blending
    - [ ] Sprite Zero Hit
    - [ ] Misc PPU flags (emphasize RGB, Greyscale, etc...)
- [ ] APU
    - [ ] Implement Registers + Memory Map them
    - [ ] Frame Timer IRQ
    - [ ] Set Up Basic Sound Output Context (SDL)
    - [ ] Channels
        - [ ] Pulse 1
        - [ ] Pulse 2
        - [ ] Triangle
        - [ ] Noise
        - [ ] DMC
    - [ ] DMC DMA
- [ ] Joypads
    - [ ] Basic Controller
    - [ ] Zapper - _still needs work_
    - [ ] NES Four Score
- [ ] WebAssembly support
    - [ ] Compile core to wasm
    - [ ] Add HTMLCanvas based PPU implementation
    - [ ] Add Web-based APU implementation

It must be blatantly obvious that the project is in a very nascent stage. It will take a long time to get this done.

The roadmaps are inspired from [the ANESE repo.](https://github.com/daniel5151/ANESE)