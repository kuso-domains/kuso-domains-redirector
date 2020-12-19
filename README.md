# kuso.domains redirector

## Usage

In such case: `kuso.example.com -> https://twitter.com/KOBA789`

Set two records like this:
```
kuso.example.com                   CNAME redirect.kuso.domains.
_kuso-domains-to.kuso.example.com. TXT   https://twitter.com/KOBA789
```
