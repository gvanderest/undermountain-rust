# Undermountain
An extensible MUD engine written in Rust.

# Goals
* Extensible - The engine should provide a foundation for creating MUD-like (any other experience) games, allowing for modules to be loaded to extend that functionality.
* Performant - Language choice of Rust was aimed for the ability to have a small memory footprint and make use of parallelization where possible to make the engine run quickly.
* Educational - This is a learning tool for me to get started in the world of Rust.  This will not necessarily speak of best practices, especially in its initial phases, but hopes to become a good learning tool.

# Non-Goals
* Completion - This is a pet project, so there's a high chance it will never actually be completed or actually truly functional.  Take heed, adventurer.  It's for this reason that I've selected a license that allows you to keep going where I might fall short.

# High-Level Design
* It's Entities All The Way Down - The initial thought is that every "thing" within the system is some kind of generic "entity" which can emit events or react to events, be it: areas, rooms, players, monsters, items, etc.
* Event Pipeline - Everything (mostly?) will be an event which travels through the system.  These events will be able to travel down an event bus (channels?) which can allow other portions of the system to listen and handle those events.
* Support Multiple Interfaces - MUDs are traditionally interfaced with over Telnet, but nowadays you might want something more secure like SSH, or apply newer ways of interacting like WebSockets.  Additionally, there are protocols which were created for adding sound/music or surfacing additional information to clients.
* Support Multiple Datastores - Because a MUD is persistent, its data needs to be stored somewhere.  The goal here is to allow for a layer of abstraction to exist, enabling the game to be stored in flat files (JSON or other) or connect to an actual database.

# Getting Started with Running

TBD - I'll update this once I get it running. :-)

# Getting Started with Developing

* Install Rust and tools (look into `rustup`)
* Pull this repository
* Make code changes
* `cargo run`
