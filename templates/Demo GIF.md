Placeholders: [project_name],[github_username],[repository_name],[repository_url],[demo_gif_url],[overview],[install_tool]
<h1 align="center">[project_name]</h1>

![demo]([demo_gif_url])

<p align="center"><img src="https://img.shields.io/github/stars/[github_username]/[repository_name].svg" /></p>

### Overview

[overview]

### Installation

#### ?Using [install_tool]

```bash
[install_tool] install [repository_name]
```

#### ?From source

```bash
git clone [repository_url].git
cd [repository_name]
cargo install --path .
```

If you don't have cargo installed, you can download the executable from the [releases]([repository_name]/releases) section.

### Usage

```bash
[repository_name]
```

### Contributing
- Fork the repository
- Create a branch
  ```bash
  git checkout -b fix/amazingFix
  ```
- Commit your changes and push to your branch
  ```bash
  git commit -m "made an amazingFix"
  git push origin fix/amazingFix
  ```
- Open a pull request