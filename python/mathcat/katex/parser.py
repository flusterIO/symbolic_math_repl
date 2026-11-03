"""Public KaTeX-to-LaTeX normalization entry point."""

from mathcat.katex.normalizers import (
    normalize_case_labels,
    normalize_environment_spacing,
    normalize_input,
    normalize_katex_spacing,
    normalize_outer_matrix,
)


def parse_katex(source: str) -> str:
    """Return parser-friendly LaTeX from a KaTeX-rendered LaTeX string.

    This performs syntax normalization only; it does not evaluate or simplify
    the mathematical expression. Individual normalization passes are kept
    separate so additional KaTeX compatibility fixes can be added modularly.
    """
    if not isinstance(source, str):
        raise TypeError("source must be a string")

    normalized = normalize_input(source)
    normalized = normalize_katex_spacing(normalized)
    normalized = normalize_case_labels(normalized)
    normalized = normalize_outer_matrix(normalized)
    return normalize_environment_spacing(normalized)
