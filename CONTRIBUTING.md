<!--
SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

<!-- omit in toc -->
# Contributing to Tic-Tac-Rustle

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

With that being said, if You like the project, but just don't have time to contribute, that's fine. You can still support the project and show your appreciation in other ways, which we would also be very happy about:

- Star the project
- Tweet (or X) about it
- Cite the project in your publications if You found it helpful
- Refer this project in your project's [README](README.md)
- Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Improving The Documentation](#improving-the-documentation)
- [Style Guides](#style-guides)
  - [Commit Messages](#commit-messages)
- [Join The Project Team](#join-the-project-team)



## I Have a Question

If You want to ask a question, we assume that You have read the available [Documentation](https://docs.rs/tictacrustle).

Before You ask a question, it is best to search for existing [Issues](https://github.com/AliSajid/tictacrustle/issues) that might help You. In case You have found a suitable issue and still need clarification, You can write your question in this issue. It is also advisable to search the internet for answers first.

If You then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/AliSajid/tictacrustle/issues/new).
- Provide as much context as You can about what You're running into.
- Provide project and platform versions (rust version, cargo version, etc.), depending on what seems relevant.

We will then take care of the issue as soon as possible.

## I Want To Contribute

### Legal Notice <!-- omit in toc -->

When contributing to this project, You must agree that You have authored 100% of the content, that You have the necessary rights to the content and that the content You contribute may be provided under the project license.


This project is dual-licensed under [MIT](LICENSES/MIT.txt) and [Apache 2.0](LICENSES/Apache-2.0.txt). When You submit changes, your submissions are understood to be under the same [MIT](LICENSES/MIT.txt) and [Apache 2.0](LICENSES/Apache-2.0.txt) that covers the project. Feel free to contact the maintainers if that's a concern.


### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase You up for more information. Therefore, we ask You to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that You are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that You have read the [documentation](https://docs.rs/tictacrustle). If You are looking for support, You might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue You are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/AliSajid/tictacrustle/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback). We use RUST_BACKTRACE=1 to get a full stack trace.
  - OS, Platform and Version (Windows, Linux, macos, x86, ARM)
  - Version of Rust, Cargo, and other environment components if applicable
  - Possibly your input and the output
  - Can You reliably reproduce the issue? And can You also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?


We use GitHub issues to track bugs and errors. If You run into an issue with the project:

- Open an [Issue](https://github.com/AliSajid/tictacrustle/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask You not to talk about a bug yet and not to label the issue.)
- Explain the behavior You would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports You should isolate the problem and create a reduced test case.
- Provide the information You collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask You for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).


### Suggesting Enhancements

This section guides You through submitting an enhancement suggestion for Gainful Key, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that You are using the latest version.
- Read the [documentation](https://docs.rs/tictacrustle) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/AliSajid/tictacrustle/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to You to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If You're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->
#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/AliSajid/tictacrustle/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior You expected to see instead** and why. At this point You can also tell which alternatives do not work for You.
- You may want to **include screenshots and animated GIFs** which help You demonstrate the steps or point out the part which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macos and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most Gainful Key users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

### Your First Code Contribution

The repository includes most config files that are needed to build and run the project. You will need a recent version of Rust and Cargo to build the project. You can find the latest version of Rust and Cargo [here](https://www.rust-lang.org/tools/install).

In addition, You need the following tools (available as cargo packages):

1. [`mdbook`](https://github.com/rust-lang/mdBook)
2. [`mdbook-inline-highlighting`](https://github.com/phoenixr-codes/mdbook-inline-highlighting)
3. [`mdbook-plantuml`](https://github.com/sytsereitsma/mdbook-plantuml)
4. [`bacon`](https://github.com/Canop/bacon)
5. [`cargo-nextest`](https://nexte.st/)
6. [`cargo-about`](https://github.com/EmbarkStudios/cargo-about)
7. [`cargo-deny`](https://embarkstudios.github.io/cargo-deny/)
8. [`reuse-tool`](https://github.com/fsfe/reuse-tool)

### Improving The Documentation

Updating, improving and correcting the documentation is another great way to contribute to the project. If You see a typo, error, or something that could be improved, please open an issue or submit a pull request. If You are not sure how to do this, please read the [documentation](https://docs.rs/tictacrustle) first. If You still have questions, please open an issue.

## Style Guides

We use `rustfmt`, `clippy` and `cargo check` to ensure a consistent code style. We also use `cargo test` to ensure that all tests pass. Please do not allow specific `clippy` lints without discussion with the team first.

### Commit Messages

We use the Conventional Commits specification for commit messages. This leads to **more readable messages** that are easy to follow when looking through the **project history**. But also, the commit messages are used to **generate the TTT Menace change log**.

## Join The Project Team

We are always open to people joining our team. Please [open an issue](https://github.com/AliSajid/tictacrustle/issues) to alert the team that You are interested in joining. We will then discuss the details in the issue.

<!-- omit in toc -->
## Attribution

This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
