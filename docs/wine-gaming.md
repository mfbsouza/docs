# Setting up wine manually for gaming

## Requirements

- wine-staging
- winetricks
- and wine dependencies. check GloriousEggroll's [How to get out of Wine Dependency hell](https://www.gloriouseggroll.tv/how-to-get-out-of-wine-dependency-hell/)
 
## Creating a wine prefix

A wine prefix is a folder the contains all of the wine configurations as well as all of the Windows pieces that WINE uses for compatibility, including libraries and a registry. One can use only a single wine prefix but some games may need diferent libs (DLLs) and/or registry configs, so creating a wine prefix per game is safer.

to create a wine prefix just run:

	mkdir -p 'path/to/prefix'
	WINEPREFIX='/path/to/prefix' winecfg

This will make wine populate the new prefix with all the basic files and pop a Wine Configuration window where you can change the windows version to 10 and hit 'ok' or just ok with you fine with the default

## Installing games

Now that we have a prefix one can easly install a game by just:

	WINEPREFIX='/path/to/prefix' wine ./path/to/installer.exe

If you have the game files already pre-installed you can just to copy it to somewhere in `/path/to/prefix/drive_c/`

## Installing DXVK

Instaling DXVK is easy as:

	WINEPREFIX='/path/to/prefix' winetricks dxvk

## Installing game dependencies

Well, sometimes you need to do some workarounds to make a game run. There is this amazing tool called winetricks that helps you do some fixes and installs in your wine prefix.

to launch winetricks just do:

	WINEPREFIX='/path/to/prefix' winetricks

If you know what you need to install you can do that in the command line, for example i have this game Silent Hill 3 that i need to install some libraries to get the game working and i just do:

	WINEPREFIX='/path/to/prefix' winetricks quartz devenum dsdmo dmband dmcompos dmime dmloader dmscript dmstyle dmsynth dmusic dmusic32 dsound
 dswave directmusic d3dx9

## Configuring DLL Overrides

Sometimes you also need to tell wine if there is some libraries (DLLs) that you want to make your game use it from either wine it self or from Windows/Game files. That's how your games uses DXVK for exemple, when you install DXVK it sets a DLL Overrides telling wine to use the D3D9/10/11 DLLs from it. To set a DLL Override it's pretty easy, just run winecfg and to to 'libraries'.

	WINEPREFIX='/path/to/prefix' winecfg

