# Reddit Post
Title: I built 'Copycat', a Pastebin for Minecraft logs + an open-source Rust library for log parsing and analysis
Hey Admincraft,
I'm happy to announce I finally released new Pastebin service for (not just) Minecraft Logs and a Minecraft Log Parser and Analyzer.

>> Features
- invalid YAML detection and AI description of whats wrong

similar to mclo.gs

the current state:
- just for Minecraft servers, not clients (basic parsing may be supported in the future but not )

Currently the detections scripts are very basic not so many, I made it mainly for host, when the checks will improve in time.

works without js
automatic highlighting (highlights all formats)

>Why not just use mclo.gs?
The main motivation for building copycat.

Site: https://paste.caukub.dev

As many of you surely know, is heavily based on _context_

Dynamic detection examples: https://github.com/copycat/detection-examples/

Example pastes:
- Minecraft Log - https://paste.caukub.dev/abc
- Invalid YAML - https://paste.caukub.dev/abc

For the Pastebin service itself there is no license meaning it's not open-source (this will probably change in the future but I can't promise it) to use for everyone but the source code and documentation is available: https://github.com/caukub/copycat

I will gladly ... feedback.

attach images!!!