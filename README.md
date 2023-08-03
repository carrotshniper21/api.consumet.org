<p align="center">
  <a href="https://consumet.org/">
    <img alt="Consumet" src="https://consumet.org/images/consumetlogo.png" width="150">
  </a>
</p>

<h1 align="center">
  Consumet API Rust
</h1>
<p align="center">
  Consumet provides an APIs for accessing information and links for various entertainments like movies, books, anime, etc.
</p>
<p align="center">
    <a href="https://github.com/consumet/api.consumet.org/actions/workflows/docker-build.yml">
      <img src="https://github.com/consumet/api.consumet.org/actions/workflows/docker-build.yml/badge.svg" alt="Discord">
    </a>
    <a href="https://github.com/consumet/api.consumet.org/actions/workflows/codeql-analysis.yml">
      <img src="https://github.com/consumet/api.consumet.org/actions/workflows/codeql-analysis.yml/badge.svg" alt="Discord">
    </a>
    <a href="https://discord.gg/qTPfvMxzNH">
      <img src="https://img.shields.io/discord/987492554486452315?color=7289da&label=discord&logo=discord&logoColor=7289da" alt="Discord">
    </a>
    <a href="https://github.com/consumet/api/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/consumet/api" alt="GitHub">
  </a>
</p>

Consumet scrapes data from various websites and provides APIs for accessing the data to satisfy your needs.

<h2> Table of Contents </h2>

- [Installation](#installation)
  - [Locally](#locally)
  - [Docker](#docker)
- [Documentation](#documentation)
- [Development](#development)
- [Showcases](#showcases)
- [Provider Request](#provider-request)
- [Support](#support)
- [Related repositories](#related-repositories)

## Installation
### Locally
installation is simple.

Run the following command to clone the repository, and install the dependencies.

```sh
$ git clone https://github.com/carrotshniper21/consumet-api.git
$ cd consumet-api
$ cargo build --release
```

start the server!

```sh
$ cargo run
```

### Docker
Docker image is available at [Docker Hub](https://hub.docker.com/r/carrotshniper21/consumet-api).

run the following command to pull and run the docker image.

```sh
$ docker build -t consumet-api . && docker run -p 8080:8080 consumet-api
```
This will start the server on port 8080. You can access the server at http://localhost:8080/, And can change the port by changing the -p option to `-p <port>:8080`.

You can add `-d` flag to run the server in detached mode.

## Documentation
Please refer to the [documentation](https://docs.consumet.org). Join our [Discord server](https://discord.gg/qTPfvMxzNH) if you need any additional help or have any questions, comments, or suggestions.

## Development
Pull requests and stars are always welcome, for bugs and features create a new [issue](https://github.com/carrotshniper21/consumet-api/issues). If you're brave to make make a commit to the project see [consumet-api-rs](https://github.com/carrotshniper21/consumet-api-rs/blob/main).

## Showcases
Showcases are welcome! If you have a project that uses Consumet API Rust, please let us know by making a new discussion [here](https://github.com/consumet/api.consumet.org/discussions/categories/show-and-tell) or by joining our [Discord server](https://discord.gg/qTPfvMxzNH). We will add your project to our [showcases page](https://consumet.org/showcase).

## Provider Request
Make a new [issue](https://github.com/carrrotshniper21/consumet-api/issues/new?assignees=&labels=provider+request&template=provider-request.yml) with the name of the provider on the title, as well as a link to the provider in the body paragraph.

## Support
You can contact the maintainers of consumet api rsvia [email](mailto:vipershniper07@gmail.com), or [join the discord server](https://discord.gg/qTPfvMxzNH) (Recommended).

<a href="https://discord.gg/qTPfvMxzNH">
   <img src="https://discordapp.com/api/guilds/987492554486452315/widget.png?style=banner2">
</p>

## Related repositories
 - [consumet-api-rs](https://github.com/carrotshniper21/consumet-api-rs)
 - [Website](https://github.com/consumet/consumet.org)
 - [Providers Status](https://github.com/consumet/providers-status)
