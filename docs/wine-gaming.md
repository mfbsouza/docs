# Setting up wine manually for gaming

## Sections
- [Requirements](#requirements)
- [Basic setup up](#basic)
	- [Creating a wine prefix](#creating-a-wine-prefix)
	- [Installing games](#installing-games)
- [Tweaks and Performance Otimizations](#tweaks-and-performance-otimizations)
	- [Installing DXVK](#installing-dxvk)
	- [Installing game dependencies](#installing-game-dependencies)
	- [Configuring DLL Overrides](#configuring-dll-overrides)
	- [Enabling ESYNC](#enabling-esync)
	- [Enabling FSYNC](#enabling-fsync)
	- [Enabling AMD FSR](#enabling-amd-fsr)
	- [Updating Wine Prefix](#updating-wine-prefix)

---

## Requirements

- wine-staging
- winetricks
- and wine dependencies. check GloriousEggroll's [How to get out of Wine Dependency hell](https://www.gloriouseggroll.tv/how-to-get-out-of-wine-dependency-hell/)
 
## Basic

### Creating a wine prefix

A wine prefix is a folder the contains all of the wine configurations as well as all of the Windows pieces that WINE uses for compatibility, including libraries and a registry. One can use only a single wine prefix but some games may need diferent libs (DLLs) and/or registry configs, so creating a wine prefix per game is safer.

to create a wine prefix just run:

	mkdir -p 'path/to/prefix'
	WINEPREFIX='/path/to/prefix' winecfg

This will make wine populate the new prefix with all the basic files and pop a Wine Configuration window where you can change the windows version to 10 and hit 'ok' or just 'ok' if you fine with the default

### Installing games

Now that we have a prefix one can easly install a game by just:

	WINEPREFIX='/path/to/prefix' wine ./path/to/installer.exe

If you have the game files already pre-installed you can just to copy it to somewhere in `/path/to/prefix/drive_c/`

## Tweaks and Performance Otimizations

### Installing DXVK

Instaling DXVK is easy as:

	WINEPREFIX='/path/to/prefix' winetricks dxvk

### Installing game dependencies

Well, sometimes you need to do some workarounds to make a game run. There is this amazing tool called winetricks that helps you do some fixes and installs in your wine prefix.

to launch winetricks just do:

	WINEPREFIX='/path/to/prefix' winetricks

If you know what you need to install you can do that in the command line, for example i have this game Silent Hill 3 that i need to install some libraries to get the game working and i just do:

	WINEPREFIX='/path/to/prefix' winetricks quartz devenum dsdmo dmband dmcompos dmime dmloader dmscript dmstyle dmsynth dmusic dmusic32 dsound dswave directmusic d3dx9

### Configuring DLL Overrides

Sometimes you also need to tell wine if there is some libraries (DLLs) that you want to make your game use it from either wine it self or from Windows/Game files. That's how your games uses DXVK for exemple, when you install DXVK it sets a DLL Overrides telling wine to use the D3D9/10/11 DLLs from it. To set a DLL Override it's pretty easy, just run winecfg and go to 'libraries'.

	WINEPREFIX='/path/to/prefix' winecfg

For exemple in Silent Hill 3 i need to set d3d8.dll to 'native, builtin' and dsound to 'builtin' to get the game running.

### Enabling ESYNC

wine staging has ESYNC support baked in, to enable it you can just:

	WINEESYNC=1 WINEPREFIX='/path/to/prefix' wine ./game.exe

**WARING** some games may not work with ESYNC enabled

### Enabling FSYNC

To use FSYNC you need a kernel and wine both with FSYNC patches (linux-zen, linux-tkg, wine-tkg, wine-ge...). Then you can simply do:

	WINEFSYNC=1 WINEPREFIX='/path/to/prefix' wine ./game.exe

**WARING** some games may not work with FSYNC enabled

### Enabling AMD FSR

To use AMD FidelityFX Super Resolution you need a wine with FShack patch with version >= 6.13. Then just do:

	WINE_FULLSCREEN_FSR=1 WINEPREFIX='/path/to/prefix' wine ./game.exe

### Updating Wine Prefix

When wine package is updated on the system most WINEPREFIX needs to be updated as well. This is handled automatically when app is started against the prefix. So something like `WINEPREFIX='/path/to/prefix' winecfg`would be sufficient.

for a more non-graphical way you can do something like:

	unset DISPLAY
	export WINEDEUG=-all
	WINEPREFIX='/path/to/prefix' wine net help >/dev/null

### Monitoring game performance

If using DXVK one can monitor games performance by enabling the bultin HUD. there a many options for what information to display, you can check it in [DXVK github page](https://github.com/doitsujin/dxvk). For basic status you can just:

	DXVK_HUD=1 WINEPREFIX='/path/to/prefix' wine ./game.exe

I would recommend you to use MangoHud which does the same things as DXVK Hud but more, like looking FPS for Vulkan and OpenGL games. check out it's [github page](https://github.com/flightlessmango/MangoHud)

**NOTE:** In some games the HUD may not work but it doesn't mean that DXVK is not in use. One can verify that your application uses DXVK instead of wined3d by checking for the presence of the log file d3d9.log or d3d11.log in the application's directory.

### Gamemode

Feral's GameMode is a daemon/lib combo for Linux that allows games to request a set of optimisations be temporily applied to the host OS and/or game process. It's really nice and really easy to use it, you just need to install it in your system and do:

	gamemoderun WINEPREFIX='/path/to/prefix' wine ./game.exe

Check it's [github page](https://github.com/FeralInteractive/gamemode) for more info.

