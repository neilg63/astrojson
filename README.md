# AstroJSON

AstroJSON is a simple wrapper for the Linux version of the Swiss Ephemeris command line astrology utility that combines one command into many.

After installing Swiss Ephemeris (apt-get install sweph on Debian / Ubuntu), you should be able to run the bundled shell scripts, which run all potential queries for a given data/time and location.

First argument: Path to astroiq.sh (which should have run permissions)
Second argument: ISO 8601 datetime e.g. 2016-04-09T19:38:23
Third argument: Decimal longitude, latitide and elevation e.g. 56.3231,-3.7929,30

This bring back not only the positions of all celestial bodies for a given time and place, but also all ayanamsas and the most common house systems in both Western and Indian Astrologies as a complete range of aspects.

The JSON output can then be easily integrated with other Web or desktop applications.

I chose Rust rather than Go or NodeJS mainly to optimise performance. This is my first rust project and is used in production on AstroIQ.com.
