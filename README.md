# better-ping-display (Horizon Rust port), 1:1

Exact 1:1 Rust translation of [Better Ping Display](https://github.com/vladmarica/better-ping-display-fabric)
(upstream Java, MIT, (c) 2020 Vlad Marica). The port carries the upstream MIT
license and credit (see `LICENSE`). Port by sparticle for the Horizon client.

The mod draws the numeric ping next to each entry in the vanilla player list:
a `+123 ms` text colored by a threshold gradient, with the upstream config
(auto color, text format, text color, optional vanilla ping bars).

## Module map (upstream -> here)

- `BetterPingDisplayMod.java` -> `better_ping_display_mod` (lifecycle)
- `Config.java` -> `config` (the Gson `ConfigData` as serde, `#rrggbb` color adapter, the `%d` format check)
- `hud/ColorUtil.java` -> `color_util` (upstream's own unit tests carried, 1:1)
- `hud/PingColors.java` -> `ping_colors` (the START/MID/END gradient at 0/150/300 ms)
- `hud/CustomPlayerListHud.java` -> `custom_player_list_hud` (`renderPingDisplay`)
- `mixin/PlayerListHudMixin.java` + `mixin/PlayerListHudInvoker.java` -> `mixin_seams`
- `integ/YaclConfigScreenFactory.java` -> `yacl_config_screen` (the same four options: order, bindings, availability listener, `%d` validator, save callback)
- `integ/ModMenuIntegration.java` -> `mod_menu`

## Seams

The Minecraft client, `DrawContext` and `PlayerListHudInvoker` surfaces live on
module-local env traits; the handler bodies are upstream's verbatim. The one
engine seam: the tab-list render injection — the slot-width constant bump
(`13 -> 13 + 45`) and the latency-icon call redirect — has no Horizon mod API
surface yet (tracked on horizon-rust/horizon#19399), so the numeric ping waits
on that surface; the port's config, colors and draw math are exact and tested.

Upstream's `pingTextFormatString` has no Horizon setting kind (there is no
string setting), so it stays in the mod's config file the way upstream keeps
it, validated the same way (it must contain `%d`); the other three fields
(`autoColorPingText`, `renderPingBars`, `pingTextColor`) ride the mod API's
settings page with upstream's lang keys.

## Build

`cargo test` (6 tests, upstream's `ColorUtilTest` 1:1). The package pins
Horizon `f00125f` / horizon-mixins `e59da909`.
