<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a id="readme-top"></a>

<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star.
*** Thanks again.
-->

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown reference style links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, and so on. This is an optional, concise syntax.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

<!-- [![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url] -->

<!-- PROJECT LOGO -->
<br />
<div align="center">
	<a href="https://start.spring.io">
		<img src="./assets/spring.png" alt="Spring Initializr Logo" width="80" height="80">
	</a>

<h3 align="center">spring-tui</h3>

<p align="center">
	Terminal UI for generating Spring Boot projects using Spring Initializr
	<br />
	<a href="https://start.spring.io"><strong>Explore Spring Initializr</strong></a>
	<br />
	<br />
	<a href="https://github.com/punixcorn/spring-tui">View Repo</a>
	&middot;
	<a href="https://github.com/punixcorn/spring-tui/issues/new?labels=bug">Report Bug</a>
	&middot;
	<a href="https://github.com/punixcorn/spring-tui/issues/new?labels=enhancement">Request Feature</a>
</p>

</div>

<!-- TABLE OF CONTENTS -->
<details>
	<summary>Table of Contents</summary>
	<ol>
		<li>
			<a href="#about-the-project">About The Project</a>
			<ul>
				<li><a href="#built-with">Built With</a></li>
			</ul>
		</li>
		<li>
			<a href="#getting-started">Getting Started</a>
			<ul>
				<li><a href="#prerequisites">Prerequisites</a></li>
				<li><a href="#installation">Installation</a></li>
			</ul>
		</li>
		<li><a href="#usage">Usage</a></li>
		<li><a href="#roadmap">Roadmap</a></li>
		<li><a href="#contributing">Contributing</a></li>
		<li><a href="#license">License</a></li>
		<li><a href="#contact">Contact</a></li>
		<li><a href="#acknowledgments">Acknowledgments</a></li>
	</ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

![Product Name Screen Shot][product-screenshot]

spring-tui is a terminal UI that lets you configure and generate Spring Boot projects using Spring Initializr.
It provides a fast keyboard driven workflow for selecting options and dependencies, and can export a config file for later reuse.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

- [Rust](https://www.rust-lang.org/)
- [ratatui](https://github.com/ratatui-org/ratatui)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [serde](https://github.com/serde-rs/serde)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

Follow these steps to build and run the app locally.

### Prerequisites

- Rust toolchain
  - https://www.rust-lang.org/tools/install

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/punixcorn/spring-tui.git
   ```
2. Build
   ```sh
   cargo build
   ```
3. Run the TUI
   ```sh
   cargo run
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Run from a config file:

```sh
./target/debug/spring-tui --file config.yaml
```

Config formats supported: YAML, JSON, TOML.

Example files:

- [rake-service-config.yaml](./example/rake-service-config.yaml)
- [security-config.toml](./example/security-config.toml)

TUI controls:
| Keybinding| Action|
|-|-|
| Tab| switches between Config and Dependencies panes |
| Up |and Down navigate items |
| Enter| selects or edits the current field |
| Shift + c| opens configuration menu |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

- [x] TUI configuration workflow
- [x] Dependency search and selection
- [x] Export configuration file
- [x] Config import from TUI
- [x] Project extraction after download
- [ ] Default config for most settings at ~/.config
- [ ] Ui improvments

See the [open issues](https://github.com/punixcorn/spring-tui/issues) for a full list of proposed features and known issues.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are welcome.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/my-change`)
3. Commit your changes (`git commit -m "Add feature"`)
4. Push to the branch (`git push origin feature/my-change`)
5. Open a pull request

Please keep changes focused and include relevant screenshots for UI changes.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the Apache License 2.0. See [LICENSE.txt](LICENSE.txt) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Project Link: https://github.com/punixcorn/spring-tui

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->

## Acknowledgments

- [Spring Initializr](https://start.spring.io)
- [ratatui](https://github.com/ratatui-org/ratatui)
- [crossterm](https://github.com/crossterm-rs/crossterm)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/punixcorn/spring-tui.svg?style=for-the-badge
[contributors-url]: https://github.com/punixcorn/spring-tui/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/punixcorn/spring-tui.svg?style=for-the-badge
[forks-url]: https://github.com/punixcorn/spring-tui/network/members
[stars-shield]: https://img.shields.io/github/stars/punixcorn/spring-tui.svg?style=for-the-badge
[stars-url]: https://github.com/punixcorn/spring-tui/stargazers
[issues-shield]: https://img.shields.io/github/issues/punixcorn/spring-tui.svg?style=for-the-badge
[issues-url]: https://github.com/punixcorn/spring-tui/issues
[license-shield]: https://img.shields.io/github/license/punixcorn/spring-tui.svg?style=for-the-badge
[license-url]: https://github.com/punixcorn/spring-tui/blob/main/LICENSE.txt
[product-screenshot]: ./assets/spring-tui-demo.png
