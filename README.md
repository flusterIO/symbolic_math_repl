# Tex-To-Image

This is a dead simple latex to katex to image script that is mostly used as part of a personal workflow that allows me to reliably work with symbolic math in the terminal. I never bothered to create a decent installation workflow, as this is really just something I built for myself, so the installation process is a little clunky. You only have to do everything once though.

![Screenshot of iPython](./assets/screenshot.png)

To get the setup above, follow the following steps:

1. Install [Ghostty](https://github.com/ghostty-org/ghostty). Other terminals that have adopted the kitty protocol _might_ work, but Ghostty works for sure.
2. Clone this repository somewhere on your machine.
3. Copy the `python` directory's contents into your `$HOME/.jupyter/extensions/` directory. This will enable an extension called `mathcat`, but you havent loaded it yet.
4. Install [Rust](https://rust-lang.org/) if you don't have it installed already. This will enable the `cargo` command.
5. Run `cargo install --path <Path to wherever you put this repository>`. This will install the Rust portion of this plugin. Rust is doing the image conversion, and Python is just calling back and forth to the terminal.
6. Install [ptPython](https://github.com/prompt-toolkit/ptpython). This is an amazing ipython wrapper that renders the images in Ghostty flawlessly.
7. This step is optional, but if you don't do it you'll have to run it every time you open the terminal: In `$HOME/.ipython/profile_default/ipython_config.py`, add the following line (modify it if the `c.InteractiveShellApp.exec_lines` is already set)

```

c.InteractiveShellApp.exec_lines = [
    "%load_ext icat",
    "%load_ext mathcat",
    ]
```

You can then set two properties through environment variables:

`IPYTHON_MATH_TRANSPARENT` and `IPYTHON_MATH_DARK_MODE`

Where both accept "1", "true", "yes" or "on"... or the opposite of course.

This package depends on [icat](https://github.com/Kabilan108/ipython-icat), so if your python installation doesn't automatically install the dependencies in your extensions folder, you'll have to run `pip install python-icat` from within your intended python environment.
