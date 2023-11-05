<h1 align="center">README Generator</h1>

![demo](https://us-east-1.tixte.net/uploads/jsch.tixte.co/readme-generator-demo.gif)

<p align="center"><img src="https://img.shields.io/crates/d/generate-readme?color=%23b7410e" /></p>

## Overview

Generate `README.md` files effortlessly using this lightweight command line utility. While it includes several default templates, its strength lies in its ability to work with your own custom templates.

## Using Custom Templates:
1. Create a directory named `readme-templates` inside the `Documents` folder of your operating system.
2. Add your markdown templates to this directory.
3. At the beginning of each template, specify placeholders by adding a line like `Placeholder: [placeholder_name_1],[placeholder_name_2]`. Avoid spaces between placeholders.

   For instance: `Placeholder: [project_name],[author_name]`

When you use the tool, it will prompt you to input values for each of these placeholders. The tool will then replace all occurrences of the placeholders throughout the file with your input.

## A Few Points to Remember:
- **Placeholders are Optional**: You can choose to include them or not.
- **No Spaces in Placeholder List**: Ensure there's no space between the placeholders you list.
  
If you would like to see some examples, check the templates folder.

**Tip**: To make a section in your template optional, simply prefix its heading with `?`. For example, `## ?Contributing` denotes an optional "Contributing" section.


## Installation

### Using cargo

```bash
cargo install readme-generator
```

### From source

```bash
git clone https://github.com/jacobschwantes/readme-generator.git
cd readme-generator
cargo install --path .
```
<!-- 
If you don't have cargo installed, you can download the executable from the [releases](readme-generator/releases) section. -->

## Usage

```bash
readme-generator
```
```bash
# To use a path to custom templates other than the default ~/Documents/readme-templates
readme-generator --template-dir path/to/custom/templates
```

## Contributing
Fork the repository
Create a branch
```bash
git checkout -b fix/amazingFix
```
Commit your changes and push to your branch
```bash
git commit -m "made an amazingFix"
git push origin fix/amazingFix
```
Open a pull request