# MMC3 IRQ Notes

## Current status

The current MMC3 work should not rely on an aggressive A12-rate clamp as the primary
fix for `SuperContra(U).nes`.

There are two separate concerns:

1. Preserve MMC3's real A12 low-time requirement.
2. Make PPU CHR fetch timing accurate enough that MMC3 only sees legitimate A12 pulses.

One subtle but important sprite rule also matters here:

- `OAM Y = 0xFF` is the special "top of screen" value.
- `OAM Y = 0xF0..0xFE` is the bottom hidden band and must not wrap into scanline `0`.

## Why this fixes the remaining jitter

Tracing the boss scene in `SuperContra(U).nes` showed that the previous implementation
was still manufacturing mapper-visible CHR activity in two ways:

1. The emulator was feeding MMC3 A12 with nametable/attribute/garbage fetches that
   should not qualify as mapper-visible CHR activity.
2. Sprite pattern fetches were timed incorrectly: subcycles `4` and `6` each performed
   a full low+high pattern read pair, so every sprite slot exposed duplicate CHR reads.
3. Sprite scanline matching treated the whole `0xF0..0xFF` range as wrapping into the
   top of the frame. That is too broad: only `0xFF` should behave as the topmost
   "Y - 1" sprite position. Values `0xF0..0xFE` must stay hidden below the visible area.

That second issue polluted the A12 waveform seen by MMC3 and made the low-time filter
look like the fix when it was really hiding a PPU timing bug.

That third issue was the remaining `Super C` boss problem. It pulled large 8x16 boss
sprites from the bottom hidden band into the pre-render line and the first few visible
scanlines, which created extra valid-looking MMC3 A12 rises about 20 PPU cycles apart.
Those extra rises moved the IRQ split by roughly 5 scanlines and caused the lower
background to jitter.

The correct direction is:

- keep the MMC3 low-time requirement because hardware has one;
- only expose CHR/pattern-table accesses to MMC3 A12 tracking;
- ensure each sprite fetch slot performs exactly one low-plane and one high-plane CHR
  read at the proper subcycles.

## Regression checklist

Automated:

- Run `cargo test`.
- Run `cargo test cartridge::tests::mmc3 -- --nocapture` when iterating on MMC3 logic.
- Run `cargo test ppu::tests::sprite_fetch_phase_reads_each_pattern_plane_once_per_slot -- --nocapture`.
- Run `cargo test supercontra_mmc3_rom_boot_frame_matches_reference_hash -- --ignored`.

Manual:

- Boot `roms/mmc3/SuperContra(U).nes`.
- Verify stage 1 scroll is stable before the boss.
- Verify the stage 1 boss split screen does not jitter, disappear, or jump.
- Verify the lower status/background region stays stable while the helicopter boss is on screen.
- Verify the screen returns to normal after the boss is defeated.
- Verify save/load still works in an MMC3 game after any MMC3 state-format change.

Tracing:

- Use `NES_TRACE_MMC3=1` to log key MMC3 events such as IRQ hits and relevant CPU-side MMC3 writes.
- Add `NES_TRACE_MMC3_VERBOSE=1` only when investigating filtered or accepted A12 rises; the log becomes very large.
