ArcheAge instance time fix Proof-of-Concept
https://github.com/Ingramz/aa-time-fix

What this does:

When launching the game, launcher will start aa-time-fix.exe first,
then aa-time-fix.exe will search for -time_offset parameter and change
its value:
1) from -120 to -180 for EU
2) from 300 to 240 for NA

and then launch the original archeage.exe with the changed parameter to
account for the time change that was configured on the server.

See main.rs for the source code of the entire program.

How to install:

1) copy aa-time-fix.exe to ArcheAge\Bin64 directory
2) rename archeage.exe to archeage-original.exe
3) rename aa-time-fix.exe to archeage.exe

Now launch game as normal.

How to remove:

1) delete archeage.exe (aa-time-fix.exe)
2) rename archeage-original.exe to archeage.exe
