# keyprs

[![CI](https://github.com/DanNixon/keyprs/actions/workflows/ci.yml/badge.svg)](https://github.com/DanNixon/keyprs/actions/workflows/ci.yml)

A very barebones tool to backup secrets to paper.

Takes secret text (e.g. passwords, keys, textual treasure maps) and prints them using a thermal/POS printer.
Adds a timestamp and an optional note/identifier for easy discovery at the appropriate time.

For the following types of secret text, `keyprs` will reformat the text to make validation and copying by eye easier:

- [age](https://github.com/FiloSottile/age) secret keys

## Why?

Textual paper backups are an amazing means of cold key backup.
At a glance you can immediately tell if the key is intact, opposed to flash media which must be verified in another system, or QR codes which ideally should be scanned to be verified.

Older thermal receipt printers (the ones typically seen on early PC based POS systems) are a good choice of printer for this.
Compared to a typical document printer, they are very dumb, i.e. have a very basic protocol, have no persistent storage, are not networked.

## Example

```text
> echo "AGE-SECRET-KEY-1K4476QULRXHS5E3Q7WR8R9KAXWZF8QK0G7W39KF445T26Z7UXLXSHTD0MY" | keyprs --note "a thing" --serial-port /dev/ttyUSB0
🔑 The input looks like an age secret key.

🖨  I will print the following:
~~~
Printed: 2024-04-27 17:21:21 +01:00
Note: a thing

AGE-SECRET-KEY-
1K44
76QU
LRXH
S5E3
Q7WR
8R9K
AXWZ
F8QK
0G7W
39KF
445T
26Z7
UXLX
SHTD
0MY
~~~

👀 Be sure to verify the printed output matches the above text!
```
