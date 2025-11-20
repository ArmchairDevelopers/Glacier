<p align="center">
<img src="https://s3.kyber.gg/frontend-assets/icons/armc-glacier-logo-default.svg" width="250rem;">
</p>
<h4 align="center">project.glacier (GLACIER) is a tool for working with STAR WARS™ Battlefront™ II (2017) game data.</h4>
<br>
<p align="center">
  <a href="https://x.com/BattleDashDev"><img src="https://img.shields.io/badge/Twitter-@BattleDashDev-1da1f2.svg?logo=twitter"></a>
  <a href="https://discord.gg/kyber">
      <img src="https://img.shields.io/discord/305338604316655616.svg?label=Discord&logo=discord&color=778cd4">
  </a>
</p>

------

This project was created by our previous lead developer, [battledash](https://github.com/battledash). He was hired by EA, so the project is unlikely to receive further development, but it contains useful tools and information for mod tool developers.

GLACIER only supports STAR WARS™ Battlefront™ II (2017). Supporting other Frostbite games is not in scope due to game-specific implementations.

## Project Goals

GLACIER was originally intended as a replacement for Frosty that would entirely replicate the Frostbite pipeline. Unlike Frosty, which works at a lower level, Glacier was designed to be more high-level and automatic. The goal was to:

- Delete and rebuild bundles, network registries, shader databases, and other game structures from scratch when converting from game source
- Automatically handle heavy pipeline work that would normally be manual
- Use heavy caching so operations are only slow on the first run
- Provide a more abstracted, developer-friendly interface for modding

The reverse pipeline (extracting game data) is the most complete feature.

## Features

The most fleshed out feature is the **reverse pipeline**, which extracts game files, shaders, bundles, and other assets from Battlefront 2. The project also includes an RVM database reader for shaders that has never been made public before.

## Reverse Pipeline

The reverse pipeline converts packaged Frostbite game data into a source format that can be manipulated. It extracts:

- Game files (EBX assets)
- Shaders (via RVM database reading)
- Bundles
- Other game resources

### Usage

Run the reverse pipeline with:

```bash
cargo run --release -p glacier_reverse_pipeline -- --source-data "<base-game-path>" --output-data "<target-dir>"
```

The `--source-data` path should point to a directory containing `Data` and `Patch` folders from a Battlefront 2 installation.

## Project Structure

This is a Rust workspace containing multiple crates:

- `glacier_reverse_pipeline` - Main reverse pipeline tool
- `glacier_resource` - Resource handling, including readers for shader databases and terrain
- `glacier_fs` - Frostbite filesystem and format readers/writers
- `glacier_reflect` - Type reflection system
- `glacier_pipeline` - Forward pipeline
- Various utility and SDK crates

## Legal

This project is not affiliated with Electronic Arts Inc. or EA Digital Illusions CE AB.
