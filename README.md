# keyprs

[![CI](https://github.com/DanNixon/keyprs/actions/workflows/ci.yml/badge.svg)](https://github.com/DanNixon/keyprs/actions/workflows/ci.yml)

A very barebones tool to backup secrets to paper.

Takes secret text (e.g. passwords, keys, textual treasure maps) and prints them using a thermal/POS printer.
Adds a timestamp and an optional note/identifier for easy discovery at the appropriate time.

Provides the option of outputting any combination of the following:

- the raw text
- a QR code
- a friendly representation of the input

Friendly representations change the formatting of certain types of secrets to assist manual reproduction when required.
The following have friendly representations, requesting a friendly representation for anything else will raise an error:

- [age](https://github.com/FiloSottile/age) secret keys
- anything in PGP ASCII armor format

## Why?

Textual paper backups are an amazing means of cold key backup.
At a glance you can immediately tell if the key is intact, opposed to flash media which must be verified in another system, or QR codes which ideally should be scanned to be verified.

Older thermal receipt printers (the ones typically seen on early PC based POS systems) are a good choice of printer for this.
Compared to a typical document printer, they are very dumb, i.e. have a very basic protocol, have no persistent storage, are not networked.
