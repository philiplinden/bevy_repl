# Sticky Prompt and DECSTBM Scroll Region

Type: prototype
Status: open
Blocked by: 01

## Question

What is the exact ANSI escape sequence flow and system ordering required to maintain a sticky prompt line at the bottom of the terminal while allowing output to scroll cleanly above it?

Specifically:
1. How and when is the DECSTBM scroll region (`\x1B[1;{bottom}r`) initialized and reset?
2. How should the prompt line (prefix symbol + buffer text + cursor position) be redrawn per frame without screen flickering?
3. How are explicit CRLF (`\r\n`) and column positioning enforced across all terminal write paths?
