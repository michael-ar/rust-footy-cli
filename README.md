Footy: A Rust CLI for Getting Football Data
========================================

This CLI provides a wrapper around a subset of the functionality offered by the excellent www.football-data.org service, from which you will need a [free API token](http://api.football-data.org/register).

## Usage

To install the CLI:

```bash
# Work out how distribute it first lol
```

Commands:

```shell
footy <command>

Commands:
  configure    Set your API token, and select your default league / team
  fixtures     Displays fixtures
  table        Displays a league table
  help         Prints this message or the help of the given subcommand
```

```shell
footy fixtures [options] 
(defaults to your set league's current matchday)

Options:
  -m, --manual  Select league fixtures to display
  -t, --team    Displays the fixtures for your default team
  -h, --help    Show help
```

```shell
footy table [options] 
(defaults to your set league's tab;e)

Options:
  -m, --manual  Select league table to display
  -h, --help    Show help
```