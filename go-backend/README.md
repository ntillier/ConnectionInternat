# A program to continually connect

# Developpment:

Because this isn't a scuffed way to do things at all, you have to use go 1.8.1, replace the code from the original crypto library

This only has to be done on the development machine ofc, and only for the 1.8.1 version of go

Original crypto lib fork was by https://github.com/mordyovits/golang-crypto-tls ; read all the code that was changed compared to normal 1.8.1 library, not that thoroughly though; it's important to understand that the procedure doesn't really have to be secure anwyays, I was moreso checking for rats or trojans to the computer itself

Here are the terminal commands I used to do all this stuff:

```bash
❯ go install golang.org/dl/go1.8.1@latest
❯ go1.8.1 download 
❯ s=$(go1.8.1 env GOROOT)/src/crypto
❯ delta $s/tls/ ./crypto-tls-replacement
❯ mv $s/tls/ $s/tls.bak
❯ cp -r ./crypto-tls-replacement $s/tls
❯ go1.8.1 build -a main.go
❯ ./main
```
