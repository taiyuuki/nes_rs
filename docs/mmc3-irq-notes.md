# MMC3 IRQ Notes

## Current status

The current MMC3 implementation is good enough to boot and render `SuperContra(U).nes`
correctly through the stage 1 boss scene, including the split-screen section that was
previously jittering.

The current fix has two parts:

1. Filter A12 rises using a low-time requirement.
2. Only expose CHR/pattern-table accesses to MMC3 A12 tracking during rendering.

## Why this fixes the remaining jitter

Tracing the boss scene in `SuperContra(U).nes` showed accepted `a12-clock` events only
`40` PPU cycles apart while IRQs were enabled. The root cause was not the MMC3 counter
itself, but that the emulator was feeding mapper A12 with nametable/attribute/garbage
fetches that should not qualify as MMC3-visible CHR activity.

Restricting mapper-visible A12 tracking to CHR/pattern accesses removes those duplicate
same-scanline clocks without relying on a scanline-sized spacing heuristic.

## Regression checklist

Automated:

- Run `cargo test`.
- Run `cargo test cartridge::tests::mmc3 -- --nocapture` when iterating on MMC3 logic.
- Run `cargo test supercontra_mmc3_rom_boot_frame_matches_reference_hash -- --ignored`.

Manual:

- Boot `roms/mmc3/SuperContra(U).nes`.
- Verify stage 1 scroll is stable before the boss.
- Verify the stage 1 boss split screen does not jitter, disappear, or jump.
- Verify the lower status/background region stays stable while the helicopter boss is on screen.
- Verify the screen returns to normal after the boss is defeated.
- Verify save/load still works in an MMC3 game after any MMC3 state-format change.

Tracing:

- Use `NES_TRACE_MMC3=1` to log key MMC3 events.
- Add `NES_TRACE_MMC3_VERBOSE=1` only when investigating filtered A12 rises; the log becomes very large.
