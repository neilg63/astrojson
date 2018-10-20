# AstroJSON

AstroJSON is a simple wrapper for the command line version of the Swiss Ephemeris astrology utility, that combines many queries into one convenient dataset with most combinations required for Western and Indian astrology.
The development version no longer requires a shell script (available only for reference purposes). 

After installing Swiss Ephemeris (apt-get install sweph on Debian / Ubuntu), you should be able to compile and run astro or set up a convenient alias, which run all potential queries for a given data/time and location.

First argument: Path to astroiq.sh (which should have run permissions)
Second argument: ISO 8601 datetime e.g. 2016-04-09T19:38:23
Third argument: Decimal longitude, latitide and elevation e.g. 56.3231,-3.7929,30

## Installation

1. CD over the directory with the source code.
2. Run "cargo build --release" . This requires the [Rust Package Manager](https://github.com/rust-lang/cargo/) for you system
3. Install Swiss Ephemeris command line on Linux or [swetest.exe on Windows](https://www.astro.com/swisseph/sweph_e.htm).
4. Run the command e.g. ./astro ./scripts/astroiq-all.sh 1987-06-06T12:30:45 45.39383,4.3398,50

This bring back not only the positions of all celestial bodies for a given time and place, but also all ayanamsas and the most common house systems in both Western and Indian Astrologies as a complete range of aspects.

The JSON output can then be easily integrated with other Web or desktop applications.

I chose Rust rather than Go or NodeJS mainly to optimise performance. This is my first rust project and is used in production on [AstroIQ.com](http://www.astroiq.com).

However, the rest of the web site uses NodeJS and MongoDB at the backend with VueJS and D3. It has a separate [repository](https://github.com/neilg63/astroiq).
