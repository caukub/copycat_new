>I built 'Copycat', a Pastebin for Minecraft Logs + an open-source Rust Library for Log Parsing and Analysis

Hey Admincraft,

I'm happy to announce I just released new Minecraft-focused Pastebin called [Copycat](https://github.com/caukub/copycat) and published the underlying open-source Rust library [`mclog`](https://github.com/caukub/mclog), a library for Minecraft log parsing and analysis.

Pastebin site: https://paste.caukub.dev

It can be used as a regular pastebin since all other file formats are automatically highlighted. However, the reason Copycat was built is specialized support for Minecraft logs (and YAML files).

Example log paste showing many features: https://paste.caukub.dev/a3pvvE7a

Example invalid YAML paste: https://paste.caukub.dev/abcdefgh

### Pastebin Features
- Static Analysis

Server info - version, platform, version

Plugin info - plugins and their version

Port info - server, query, rcon, __plugins__, __collisions__, shows if port is TCP/UDP

- ! __Dynamic Analysis__ !

This is the main reason Copycat was created.

get info from static analyzer and you can write custom scripts that

Dynamic analysis examples with explanation and documentation: https://github.com/caukub/copycat/detection-examples/

Dynamic analysis detections used on the site: https://github.com/caukub/copycat/detections/

Currently the detections scripts are very basic not so many, more checks will be added and improved over time.
you can help too (showed who added it)! i will be glad if you can contribute detection (contributing guide napsat - detailed explanation of why atd)

- Highlighting

Different formats for the log level supported not only `/INFO]` but also e.g. `00:00:00 INFO` (some people send console output instead of log which is formatted by hosts)

The lines are highlighted depending on the log level (INFO/WARN/ERROR and even custom for custom messages like ones that appear from hosts and panels in the console).

.jar files in stacktraces (not any, this has some rules, see here)

[CHAT] in client logs

- IP addresses censorship

IP addresses are automatically censored.
Plugins and server related versions that match IP addresses are not.

Features listed above are all features implemented using the [`mclog`](https://github.com/caukub/mclog) library.

- Developer API

Mainly for shared hosts

- Invalid YAML detection

If file content seems to be YAML but the format is not valid YAML (detected by awesome tool [magika](https://securityresearch.google/magika/introduction/overview)), the paste will show you parser error which tries to be human friendly.

It is even possible to request AI explanation of what's wrong (AI explanation is opt-in feature, by default, only the parser error is shown and even the Magika detection can be disabled too).

- Highlighting specific lines (and range of lines)

>Why not just use mclo.gs?

The main motivation for building copycat.

As many of you surely know, is heavily based on _context_

Limitations:

- Dynamic analysis will be provided only for servers. Some static analysis may be added in the future but right now there is none.

- No crash log support

- Mod support for static analysis is not implemented yet, meaning IP address censorship will hide mod versions matching IP addresses except mod load message thus you can't use it in dynamic analysis

Some extra info:

- Works without JavaScript (but few non-important features requires JS)

- For the Pastebin service itself there is no license meaning it's not open-source (this will likely change soon, but I can't promise) to use for everyone but the source code and documentation is available: https://github.com/caukub/copycat

- I will gladly ... feedback (easiest on discord which you can find on github).

<attach images!!>