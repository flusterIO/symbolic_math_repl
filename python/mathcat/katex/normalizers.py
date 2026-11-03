"""Small normalization passes for KaTeX-produced LaTeX strings."""

import re


def normalize_input(source: str) -> str:
    """Remove surrounding math delimiters and normalize line endings."""
    source = source.replace("\r\n", "\n").replace("\r", "\n").strip()
    for opening, closing in (("$$", "$$"), ("$", "$"), (r"\[", r"\]"), (r"\(", r"\)")):
        if source.startswith(opening) and source.endswith(closing):
            source = source[len(opening) : -len(closing)].strip()
            break
    return source


def normalize_katex_spacing(source: str) -> str:
    """Replace KaTeX spacing commands commonly rejected by LaTeX parsers."""
    # KaTeX uses this form in conditional text, e.g. ``\text{for}\: R > 0``.
    return re.sub(r"\\text\{([^{}]*)\}\\(?:[,;:!]|quad|qquad)\s*", r"\\text{\1} ", source)


def normalize_case_labels(source: str) -> str:
    """Remove prose labels from KaTeX cases while preserving conditions.

    KaTeX commonly renders SymPy-style conditions as ``& \text{for}\: x``
    and the fallback row as ``& \text{otherwise}``. The words are annotation,
    not part of either branch's mathematics, and several LaTeX parsers reject
    them in a cases environment.
    """
    source = re.sub(
        r"\\text\{for\}(?:\\(?:[,;:!]|quad|qquad))?\s*", "", source
    )
    return re.sub(r"\\text\{otherwise\}", "", source)


def normalize_outer_matrix(source: str) -> str:
    """Use a single standard bracketed-matrix environment for outer matrices."""
    match = re.fullmatch(
        r"\s*\\left\[\s*\\begin\{matrix\}(.*)"
        r"\\end\{matrix\}\s*\\right\]\s*",
        source,
        flags=re.DOTALL,
    )
    if match is None:
        return source

    return r"\begin{bmatrix}" + match.group(1) + r"\end{bmatrix}"


def normalize_environment_spacing(source: str) -> str:
    """Make whitespace around environment boundaries consistent."""
    source = re.sub(r"\\(begin|end)\{\s*([A-Za-z*]+)\s*\}", r"\\\1{\2}", source)
    return source
