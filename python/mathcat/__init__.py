from base64 import encodebytes
import json
import imgcat
import os
import subprocess
from pathlib import Path
from IPython.core.getipython import get_ipython
from mathcat.katex.parser import parse_katex

ipython = get_ipython()


IPYTHON_MATH_TRANSPARENT = os.environ.get(
    "IPYTHON_MATH_TRANSPARENT", "true"
).strip().lower() in {"1", "true", "yes", "on"}
IPYTHON_MATH_DARK_MODE = os.environ.get(
    "IPYTHON_MATH_DARK_MODE", "true"
).strip().lower() in {"1", "true", "yes", "on"}

IMAGE_CODE = "\033]1337;File=name=name;inline=true;:{}\a"


def imcat(image_data, metadata):
    try:
        print(IMAGE_CODE.format(encodebytes(image_data).decode()))
    # bug workaround
    except:
        print(IMAGE_CODE.format(image_data))


def temp_folder():
    home = Path.home()
    return home / ".jupyter" / "extensions" / "mathcat" / "mathcat" / "tempdir"


def removeLeadingDolarSign(s: str):
    if s.strip().startswith("$"):
        return s.strip()[1:]
    return s


def removeTrailingDolarSign(s: str):
    if s.strip().endswith("$"):
        return s.strip()[:-1]
    return s


def _mathcat(data, meta):
    _newData = parse_katex(data)
    latex = _newData.strip("$").strip()

    try:
        result = subprocess.run(
            ["tex-to-image", "convert"],
            input=json.dumps(
                {
                    "latex": latex,
                    "transparent": IPYTHON_MATH_TRANSPARENT,
                    "dark_mode": IPYTHON_MATH_DARK_MODE,
                }
            ),
            capture_output=True,
            text=True,
            check=True,
        )
        outputPath = Path(result.stdout.strip())
    except (OSError, subprocess.CalledProcessError) as error:
        print(f"Input Latex: {_newData}")
        print(f"Failed generating png: {error}")
        return

    if outputPath.is_file():
        if ipython:
            print("\n")  ## Otherwise there's overlap
            _ = ipython.run_line_magic("icat", outputPath.__str__())
        else:
            print("You broke something...")
    else:
        print(f"Input Latex: {_newData}")
        print(f"PNG file not found: {outputPath}")


def clean_temp_folder():
    (temp_folder() / "render.png").unlink(missing_ok=True)


def mathcat(data):
    return _mathcat(data, "")


# imgCmd = f"cat {outputPath} | imgcat"
# imgCmd = f"%imgcat {outputPath}"
# ip = get_ipython()
# ip.system_raw(imgCmd)


def register_mimerenderer(ipython, mime, handler):
    ipython.display_formatter.active_types.append(mime)
    ipython.display_formatter.formatters[mime].enabled = True
    ipython.mime_renderers[mime] = handler


def load_ipython_extension(ipython):
    register_mimerenderer(ipython, "text/latex", _mathcat)
