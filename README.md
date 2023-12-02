# Blockchain Commons command line parser/validator for deterministic CBOR ("dCBOR")

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally_

---

`dcbor` is a command line Deterministic CBOR ("dCBOR") validation and diagnostic tool based on [the crate of the same name](https://crates.io/crates/dcbor).

* Validates dCBOR inputs.
* Receives inputs in hex or binary format.
* Formats output several different ways:
  * CBOR diagnostic notation (compact or annotated).
  * Hexadecimal (compact or annotated).
  * Binary

## Related Projects

* [dCBOR Overview](https://github.com/BlockchainCommons/crypto-commons/blob/master/dcbor.md)
* [dCBOR Library for Rust](https://github.com/BlockchainCommons/bc-dcbor-rust)
* [dCBOR Library for Swift](https://github.com/BlockchainCommons/BCSwiftDCBOR)

## Installation

To install from crates.io, run:

```bash
cargo install dcbor-cli
```

To install from source, clone this repo, change to its root directory and run:

```bash
cargo install --path .
```

## Command Line Syntax

This is the command line syntax as output by typing `dcbor --help`:

```
Command line parser/validator for deterministic CBOR ("dCBOR").

Usage: dcbor [OPTIONS] [HEX]

Arguments:
  [HEX]
          Input dCBOR as hexadecimal. If not provided here or input format is binary, input is read from STDIN

Options:
  -i, --in <IN>
          The input format

          [default: hex]

          Possible values:
          - hex: Hexadecimal
          - bin: Raw binary

  -o, --out <OUT>
          The output format

          [default: diag]

          Possible values:
          - diag: CBOR diagnostic notation
          - hex:  Hexadecimal
          - bin:  Raw binary
          - none: No output: merely succeeds on validation of input

  -c, --compact
          Output diagnostic notation or hexadecimal in compact form. Ignored for other output formats

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

### Validate dCBOR and print it out as CBOR diagnostic notation

```
$ dcbor 6548656C6C6F
"Hello"
```

```
$ CBOR_HEX=d99d6ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e
```

```
$ dcbor $CBOR_HEX
40300(
   {
      1:
      h'59f2293a5bce7d4de59e71b4207ac5d2',
      2:
      1(2021-02-24T00:00:00Z),   / date /
      3:
      "Dark Purple Aqua Love",
      4:
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
   }
)

$ dcbor --compact $CBOR_HEX
40300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})
```

### Validate dCBOR and print it out as annotated hexadecimal

```
$ dcbor --out hex $CBOR_HEX
d9 9d6c                                  # tag(40300)
   a4                                    # map(4)
      01                                 # unsigned(1)
      50                                 # bytes(16)
         59f2293a5bce7d4de59e71b4207ac5d2
      02                                 # unsigned(2)
      c1                                 # tag(1) date
         1a60359700                      # unsigned(1614124800)
      03                                 # unsigned(3)
      75                                 # text(21)
         4461726b20507572706c652041717561204c6f7665 # "Dark Purple Aqua Love"
      04                                 # unsigned(4)
      78 7b                              # text(123)
         4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e # "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
```

### Convert dCBOR from Hexadecimal to Binary and Back

```
# Write the binary to a file using stdout.
$ dcbor --out bin $CBOR_HEX >test.bin

# Read it back in from the same file.
$ dcbor --in bin --out hex --compact <test.bin
d99d6ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e
```

## Status - Alpha

`dcbor`  is currently under active development and in the alpha testing phase. It should not be used for production tasks until it has had further testing and auditing. See [Blockchain Commons' Development Phases](https://github.com/BlockchainCommons/Community/blob/master/release-path.md).

## Financial Support

`dcbor` is a project of [Blockchain Commons](https://www.blockchaincommons.com/). We are proudly a "not-for-profit" social benefit corporation committed to open source & open development. Our work is funded entirely by donations and collaborative partnerships with people like you. Every contribution will be spent on building open tools, technologies, and techniques that sustain and advance blockchain and internet security infrastructure and promote an open web.

To financially support further development of `dcbor` and other projects, please consider becoming a Patron of Blockchain Commons through ongoing monthly patronage as a [GitHub Sponsor](https://github.com/sponsors/BlockchainCommons). You can also support Blockchain Commons with bitcoins at our [BTCPay Server](https://btcpay.blockchaincommons.com/).

## Contributing

We encourage public contributions through issues and pull requests! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for details on our development process. All contributions to this repository require a GPG signed [Contributor License Agreement](./CLA.md).

### Discussions

The best place to talk about Blockchain Commons and its projects is in our GitHub Discussions areas.

[**Gordian Developer Community**](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). For standards and open-source developers who want to talk about interoperable wallet specifications, please use the Discussions area of the [Gordian Developer Community repo](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). This is where you talk about Gordian specifications such as [Gordian Envelope](https://github.com/BlockchainCommons/Gordian/tree/master/Envelope#articles), [bc-shamir](https://github.com/BlockchainCommons/bc-shamir), [Sharded Secret Key Reconstruction](https://github.com/BlockchainCommons/bc-sskr), and [bc-ur](https://github.com/BlockchainCommons/bc-ur) as well as the larger [Gordian Architecture](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Architecture.md), its [Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles) of independence, privacy, resilience, and openness, and its macro-architectural ideas such as functional partition (including airgapping, the original name of this community).

[**Gordian User Community**](https://github.com/BlockchainCommons/Gordian/discussions). For users of the Gordian reference apps, including [Gordian Coordinator](https://github.com/BlockchainCommons/iOS-GordianCoordinator), [Gordian Seed Tool](https://github.com/BlockchainCommons/GordianSeedTool-iOS), [Gordian Server](https://github.com/BlockchainCommons/GordianServer-macOS), [Gordian Wallet](https://github.com/BlockchainCommons/GordianWallet-iOS), and [SpotBit](https://github.com/BlockchainCommons/spotbit) as well as our whole series of [CLI apps](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Apps.md#cli-apps). This is a place to talk about bug reports and feature requests as well as to explore how our reference apps embody the [Gordian Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles).

[**Blockchain Commons Discussions**](https://github.com/BlockchainCommons/Community/discussions). For developers, interns, and patrons of Blockchain Commons, please use the discussions area of the [Community repo](https://github.com/BlockchainCommons/Community) to talk about general Blockchain Commons issues, the intern program, or topics other than those covered by the [Gordian Developer Community](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions) or the
[Gordian User Community](https://github.com/BlockchainCommons/Gordian/discussions).

### Other Questions & Problems

As an open-source, open-development community, Blockchain Commons does not have the resources to provide direct support of our projects. Please consider the discussions area as a locale where you might get answers to questions. Alternatively, please use this repository's [issues](./issues) feature. Unfortunately, we can not make any promises on response time.

If your company requires support to use our projects, please feel free to contact us directly about options. We may be able to offer you a contract for support from one of our contributors, or we might be able to point you to another entity who can offer the contractual support that you need.

### Credits

The following people directly contributed to this repository. You can add your name here by getting involved. The first step is learning how to contribute from our [CONTRIBUTING.md](./CONTRIBUTING.md) documentation.

| Name              | Role                | Github                                            | Email                                 | GPG Fingerprint                                    |
| ----------------- | ------------------- | ------------------------------------------------- | ------------------------------------- | -------------------------------------------------- |
| Christopher Allen | Principal Architect | [@ChristopherA](https://github.com/ChristopherA)  | \<ChristopherA@LifeWithAlacrity.com\> | FDFE 14A5 4ECB 30FC 5D22  74EF F8D3 6C91 3574 05ED |
| Wolf McNally      | Lead Researcher/Engineer         | [@WolfMcNally](https://github.com/wolfmcnally)    | \<Wolf@WolfMcNally.com\>              | 9436 52EE 3844 1760 C3DC  3536 4B6C 2FCF 8947 80AE |

## Responsible Disclosure

We want to keep all of our software safe for everyone. If you have discovered a security vulnerability, we appreciate your help in disclosing it to us in a responsible manner. We are unfortunately not able to offer bug bounties at this time.

We do ask that you offer us good faith and use best efforts not to leak information or harm any user, their data, or our developer community. Please give us a reasonable amount of time to fix the issue before you publish it. Do not defraud our users or us in the process of discovery. We promise not to bring legal action against researchers who point out a problem provided they do their best to follow the these guidelines.

### Reporting a Vulnerability

Please report suspected security vulnerabilities in private via email to ChristopherA@BlockchainCommons.com (do not use this email for support). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

The following keys may be used to communicate sensitive information to developers:

| Name              | Fingerprint                                        |
| ----------------- | -------------------------------------------------- |
| Christopher Allen | FDFE 14A5 4ECB 30FC 5D22  74EF F8D3 6C91 3574 05ED |

You can import a key by running the following command with that individual’s fingerprint: `gpg --recv-keys "<fingerprint>"` Ensure that you put quotes around fingerprints that contain spaces.
